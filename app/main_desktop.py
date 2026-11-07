"""
main_desktop.py - Ponto de Entrada Executável Desktop do Arcade Hub
Inicia o servidor HTTP multi-threaded local em thread de fundo
e abre a interface em uma janela dedicada de aplicativo nativo desktop (modo --app).
"""
import os
import sys
import time
import socket
import shutil
import threading
import subprocess
import webbrowser
import urllib.request
from http.server import ThreadingHTTPServer

# Configuração de caminhos para execução normal ou compilada (PyInstaller)
if getattr(sys, 'frozen', False):
    BASE_DIR = getattr(sys, '_MEIPASS', os.path.dirname(sys.executable))
    WORKING_DIR = os.path.dirname(sys.executable)
else:
    BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    WORKING_DIR = BASE_DIR

if BASE_DIR not in sys.path:
    sys.path.insert(0, BASE_DIR)

APP_DIR = os.path.join(BASE_DIR, "app")
if APP_DIR not in sys.path:
    sys.path.insert(0, APP_DIR)

# Redireciona stdout/stderr se rodando em modo --noconsole (PyInstaller)
if sys.stdout is None:
    sys.stdout = open(os.devnull, "w")
if sys.stderr is None:
    sys.stderr = open(os.devnull, "w")

import app.server as arcade_server
from app.server import ArcadeHandler, scan_and_sync_games


def is_arcade_hub_running(port):
    """Verifica se já existe um servidor Arcade Hub ativo e respondendo na porta."""
    try:
        req = urllib.request.urlopen(f"http://127.0.0.1:{port}/api/heartbeat", timeout=0.6)
        return req.status == 200
    except Exception:
        return False


def find_available_port(start_port=8090, max_attempts=50):
    """Encontra uma porta TCP livre (não ocupada por outro processo) a partir de start_port."""
    for port in range(start_port, start_port + max_attempts):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as check_sock:
            check_sock.settimeout(0.15)
            if check_sock.connect_ex(('127.0.0.1', port)) == 0 or check_sock.connect_ex(('localhost', port)) == 0:
                continue
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            try:
                s.bind(('127.0.0.1', port))
                return port
            except OSError:
                continue
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(('127.0.0.1', 0))
        return s.getsockname()[1]


def find_browser_app_binary():
    """Identifica o executável do Edge ou Chrome para abrir em modo --app."""
    candidates = [
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
        os.path.expandvars(r"%LOCALAPPDATA%\Microsoft\Edge\Application\msedge.exe"),
        shutil.which("msedge") or "",
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        os.path.expandvars(r"%LOCALAPPDATA%\Google\Chrome\Application\chrome.exe"),
        shutil.which("chrome") or "",
    ]
    for path in candidates:
        if path and os.path.exists(path):
            return path
    return None


def launch_desktop_window(url):
    """Abre a janela dedicada do aplicativo desktop com perfil isolado."""
    browser_bin = find_browser_app_binary()
    if browser_bin:
        # Perfil isolado garante que o Edge/Chrome crie um processo dedicado para a janela do App
        profile_dir = os.path.join(os.path.expandvars(r"%LOCALAPPDATA%"), "ArcadeHub", "AppWindowProfile")
        os.makedirs(profile_dir, exist_ok=True)
        cmd = [
            browser_bin,
            f"--app={url}",
            f"--user-data-dir={profile_dir}",
            "--window-size=1360,860",
            "--no-first-run",
            "--no-default-browser-check",
            "--disable-features=Translate"
        ]
        try:
            process = subprocess.Popen(cmd)
            return process
        except Exception as e:
            print(f"Erro ao abrir janela nativa: {e}")

    webbrowser.open(url)
    return None


def wait_for_server_ready(port, timeout=8.0):
    """Aguarda até que o servidor HTTP interno esteja respondendo antes de abrir a janela."""
    deadline = time.time() + timeout
    while time.time() < deadline:
        if is_arcade_hub_running(port):
            return True
        time.sleep(0.1)
    return False


def main():
    # Se já houver uma instância do Arcade Hub rodando na porta padrão, apenas abre a janela para ela
    for existing_port in (8090, 8091, 8092):
        if is_arcade_hub_running(existing_port):
            launch_desktop_window(f"http://127.0.0.1:{existing_port}/index.html")
            return

    try:
        scan_and_sync_games()
    except Exception as e:
        print(f"Aviso na sincronização do catálogo: {e}")

    port = find_available_port(8090)
    ThreadingHTTPServer.allow_reuse_address = False
    server_address = ('127.0.0.1', port)
    httpd = ThreadingHTTPServer(server_address, ArcadeHandler)

    server_thread = threading.Thread(target=httpd.serve_forever, daemon=True)
    server_thread.start()

    # Garante que o servidor esteja escutando antes de abrir a janela
    wait_for_server_ready(port)

    url = f"http://127.0.0.1:{port}/index.html"
    arcade_server.LAST_HEARTBEAT = time.time()
    start_time = time.time()
    browser_proc = launch_desktop_window(url)

    try:
        if browser_proc:
            browser_proc.wait()
            elapsed = time.time() - start_time
            # Se o processo do navegador delegou a abertura para outro processo existente e fechou rápido (< 5s),
            # mantém o servidor vivo enquanto a janela continuar enviando heartbeats!
            if elapsed < 5.0:
                arcade_server.LAST_HEARTBEAT = time.time()
                while True:
                    time.sleep(2.0)
                    # Dá 25s de carência inicial e depois encerra se ficar 12s sem heartbeat da janela
                    if time.time() - start_time > 25.0 and (time.time() - arcade_server.LAST_HEARTBEAT) > 12.0:
                        break
        else:
            while True:
                time.sleep(2.0)
                if time.time() - start_time > 25.0 and (time.time() - arcade_server.LAST_HEARTBEAT) > 12.0:
                    break
    except KeyboardInterrupt:
        pass
    finally:
        try:
            httpd.shutdown()
        except Exception:
            pass


if __name__ == "__main__":
    main()
