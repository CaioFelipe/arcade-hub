"""
server.py - Servidor Multi-threaded Local e Gerenciador de Jogos In-Window para o Arcade Hub
Roda 100% offline em localhost com concorrência, sandbox AST e motor in-window em memória.
"""
import os
import sys
import time
import json
import re
import base64
import urllib.parse
from http.server import ThreadingHTTPServer, SimpleHTTPRequestHandler

import shutil

# Adiciona caminhos para importação direta de módulos (suporta PyInstaller frozen e script local)
if getattr(sys, 'frozen', False):
    BUNDLE_DIR = getattr(sys, '_MEIPASS', os.path.dirname(sys.executable))
    BASE_DIR = os.path.dirname(sys.executable)
else:
    BUNDLE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    BASE_DIR = BUNDLE_DIR

APP_DIR = os.path.join(BUNDLE_DIR, "app") if not os.path.exists(os.path.join(BASE_DIR, "app")) else os.path.join(BASE_DIR, "app")
GAMES_DIR = os.path.join(BASE_DIR, "games")
DATA_DIR = os.path.join(BASE_DIR, "data")

# Se rodando a partir do executável em outra pasta sem a pasta games/, extrai o catálogo embutido
if getattr(sys, 'frozen', False) and not os.path.exists(GAMES_DIR):
    bundled_games = os.path.join(BUNDLE_DIR, "games")
    if os.path.exists(bundled_games):
        shutil.copytree(bundled_games, GAMES_DIR, dirs_exist_ok=True)

if BASE_DIR not in sys.path:
    sys.path.insert(0, BASE_DIR)
if APP_DIR not in sys.path:
    sys.path.insert(0, APP_DIR)

from app import ast_validator
from app import hub_sdk
from app.game_engine import runner

os.makedirs(DATA_DIR, exist_ok=True)
os.makedirs(GAMES_DIR, exist_ok=True)

SCORES_FILE = os.path.join(DATA_DIR, "scores.json")
LAST_HEARTBEAT = time.time()
if not os.path.exists(SCORES_FILE):
    with open(SCORES_FILE, "w", encoding="utf-8") as f:
        json.dump({
            "space_odyssey": [{"name": "Comandante Fox", "score": 148200, "level": 14},
                              {"name": "Piloto Zero", "score": 122950, "level": 11},
                              {"name": "Astro", "score": 98400, "level": 9}],
            "snake": [{"name": "SnakeMaster", "score": 420, "level": 8}],
            "breakout": [{"name": "BlockBreaker", "score": 1850, "level": 5}]
        }, f, indent=2, ensure_ascii=False)

GAME_TEMPLATE = '''"""
Jogo Criado no Arcade Hub
Autor: Desenvolvedor Comunitário
"""
import math
import random
import pygame
import hub_sdk

class Game:
    def __init__(self, width=800, height=600):
        self.width = width
        self.height = height
        self.reset()

    def reset(self):
        self.score = 0
        self.level = 1
        self.game_over = False
        self.player_x = 400
        self.player_y = 510
        self.targets = []
        for _ in range(5):
            self.targets.append([
                random.randint(60, 740),
                random.randint(60, 320),
                random.choice([-3, 3]),
                random.choice([(244, 63, 94), (250, 204, 21), (96, 165, 250)])
            ])

    def update(self, keys, events):
        if self.game_over:
            return

        # Movimento via teclado
        if "ArrowLeft" in keys or "a" in keys or "A" in keys:
            self.player_x = max(30, self.player_x - 7)
        if "ArrowRight" in keys or "d" in keys or "D" in keys:
            self.player_x = min(770, self.player_x + 7)

        # Mouse / Clique para disparar pontuação
        for ev in events:
            if ev.get("type") == "mousemove":
                self.player_x = max(30, min(770, ev.get("x", self.player_x)))
            elif ev.get("type") == "click":
                self.score += 50
                if self.score % 500 == 0:
                    self.level += 1

        # Mover alvos
        for t in self.targets:
            t[0] += t[2]
            if t[0] < 50 or t[0] > 750:
                t[2] *= -1

    def draw(self, surface, font):
        surface.fill((9, 12, 20))

        # Alvos
        for t in self.targets:
            pygame.draw.circle(surface, t[3], (int(t[0]), int(t[1])), 18)
            pygame.draw.circle(surface, (255, 255, 255), (int(t[0]), int(t[1])), 6)

        # Jogador
        px, py = int(self.player_x), int(self.player_y)
        pygame.draw.circle(surface, (37, 99, 235), (px, py), 22)
        pygame.draw.circle(surface, (96, 165, 250), (px, py), 14)
        pygame.draw.circle(surface, (250, 204, 21), (px, py), 6)

        # HUD
        txt_score = font.render(f"PONTUAÇÃO: {self.score}", True, (250, 204, 21))
        surface.blit(txt_score, (20, 20))
        txt_lvl = font.render(f"NÍVEL: {self.level}", True, (96, 165, 250))
        surface.blit(txt_lvl, (20, 50))
'''

def scan_and_sync_games():
    """
    Escaneia subpastas em games/, valida main.py via AST e sincroniza manifest.json e catalog.json.
    """
    cat_path = os.path.join(GAMES_DIR, "catalog.json")
    catalog = {"collections": [], "games": []}
    if os.path.exists(cat_path):
        try:
            with open(cat_path, "r", encoding="utf-8") as f:
                catalog = json.load(f)
        except Exception:
            catalog = {"collections": [], "games": []}

    valid_folders = set(item for item in os.listdir(GAMES_DIR) if os.path.isdir(os.path.join(GAMES_DIR, item)))
    existing_games = {g["id"]: g for g in catalog.get("games", []) if g["id"] in valid_folders}

    for item in os.listdir(GAMES_DIR):
        folder = os.path.join(GAMES_DIR, item)
        if not os.path.isdir(folder):
            continue

        main_py = os.path.join(folder, "main.py")
        manifest_path = os.path.join(folder, "manifest.json")

        if os.path.exists(main_py):
            # Validação AST de Segurança
            val = ast_validator.validate_game_file(main_py)

            manifest = {}
            if os.path.exists(manifest_path):
                try:
                    with open(manifest_path, "r", encoding="utf-8") as mf:
                        manifest = json.load(mf)
                except Exception:
                    manifest = {}

            # Preencher campos mínimos caso não existam
            if not manifest:
                manifest = {
                    "id": item,
                    "title": item.replace("_", " ").title(),
                    "author": "Comunidade",
                    "category": "Comunitário",
                    "players": "1 Jogador",
                    "description": f"Jogo {item} adicionado à biblioteca.",
                    "controls": {"movement": "Setas ou WASD", "action": "Espaço / Clique"},
                    "engine": "Pygame In-Window",
                    "type": "community",
                    "featured": False
                }

            # Atualizar auditoria de segurança
            manifest["security_verified"] = val["valid"]
            manifest["security_errors"] = val["errors"]
            manifest["sha256"] = val["sha256"]

            # Gravar manifest atualizado
            try:
                with open(manifest_path, "w", encoding="utf-8") as mf:
                    json.dump(manifest, mf, indent=2, ensure_ascii=False)
            except Exception:
                pass

            existing_games[item] = manifest

    catalog["games"] = list(existing_games.values())
    try:
        with open(cat_path, "w", encoding="utf-8") as f:
            json.dump(catalog, f, indent=2, ensure_ascii=False)
    except Exception:
        pass

    return catalog


class ArcadeHandler(SimpleHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=APP_DIR, **kwargs)

    def log_message(self, format, *args):
        # Evita erro quando sys.stderr é None em modo executável --noconsole
        if sys.stderr is not None:
            try:
                super().log_message(format, *args)
            except Exception:
                pass

    def do_GET(self):
        global LAST_HEARTBEAT
        LAST_HEARTBEAT = time.time()
        parsed = urllib.parse.urlparse(self.path)
        path = parsed.path

        if path == "/api/heartbeat":
            self.send_json_response({"status": "alive", "app": "ArcadeHub"})
        elif path == "/api/catalog":
            catalog = scan_and_sync_games()
            catalog["template_code"] = GAME_TEMPLATE
            self.send_json_response(catalog)
        elif path == "/api/scores":
            self.send_json_response(self.get_scores())
        elif path.startswith("/games/"):
            # Serve capas e ativos estáticos da pasta games
            self.serve_game_asset(path)
        elif path == "/" or path == "/index.html":
            index_file = os.path.join(APP_DIR, "index.html")
            if os.path.exists(index_file):
                with open(index_file, "rb") as f:
                    content = f.read()
                self.send_response(200)
                self.send_header("Content-Type", "text/html; charset=utf-8")
                self.send_header("Content-Length", str(len(content)))
                self.send_header("Connection", "close")
                self.end_headers()
                self.wfile.write(content)
            else:
                self.send_response(404)
                self.end_headers()
        else:
            return super().do_GET()

    def do_POST(self):
        global LAST_HEARTBEAT
        LAST_HEARTBEAT = time.time()
        parsed = urllib.parse.urlparse(self.path)
        path = parsed.path

        content_length = int(self.headers.get('Content-Length', 0))
        body = self.rfile.read(content_length).decode('utf-8') if content_length > 0 else ""
        try:
            data = json.loads(body) if body else {}
        except Exception:
            data = {}

        if path == "/api/launch":
            game_id = data.get("id")
            result = self.handle_launch(game_id)
            self.send_json_response(result)
        elif path == "/api/game/tick":
            keys = data.get("keys", [])
            events = data.get("events", [])
            result = runner.step(keys, events)
            self.send_json_response(result)
        elif path == "/api/game/action":
            action = data.get("action")
            result = self.handle_game_action(action)
            self.send_json_response(result)
        elif path == "/api/games/validate":
            code = data.get("code", "")
            result = ast_validator.validate_game_code(code)
            self.send_json_response(result)
        elif path == "/api/games/create":
            result = self.handle_create_game(data)
            self.send_json_response(result)
        elif path == "/api/score":
            result = self.save_score(data)
            self.send_json_response(result)
        else:
            self.send_response(404)
            self.end_headers()

    def handle_launch(self, game_id: str):
        if not game_id:
            return {"status": "error", "message": "ID do jogo não informado"}

        game_path = os.path.join(GAMES_DIR, game_id, "main.py")
        if not os.path.exists(game_path):
            return {"status": "error", "message": f"Jogo '{game_id}' não encontrado em {game_path}"}

        # Validação na Sandbox AST antes de executar
        val = ast_validator.validate_game_file(game_path)
        if not val["valid"]:
            return {
                "status": "error",
                "message": "Bloqueado pela Sandbox AST de Segurança",
                "errors": val["errors"],
                "game_id": game_id
            }

        try:
            # Inicia o jogo no motor in-window (100% em memória, zero janelas externas)
            result = runner.start_game(game_id)
            return {
                "status": "success",
                "message": f"Jogo '{game_id}' iniciado com sucesso no Hub.",
                "game_id": game_id,
                **result
            }
        except Exception as e:
            return {
                "status": "error",
                "message": f"Falha ao iniciar jogo in-window: {str(e)}",
                "game_id": game_id
            }

    def handle_game_action(self, action: str):
        if action == "restart":
            if runner.game_id:
                result = runner.start_game(runner.game_id)
                return {"status": "ok", "action": "restart", **result}
            return {"status": "error", "message": "Nenhum jogo ativo"}
        elif action == "pause":
            return runner.toggle_pause()
        elif action == "stop":
            current_game = runner.game_id
            current_score = runner.score
            current_level = runner.level
            if current_score > 0 and current_game:
                hub_sdk.submit_score(current_score, current_level, "Jogador Arcade", current_game)
            result = runner.stop_game()
            return result
        else:
            return {"status": "error", "message": f"Ação desconhecida: {action}"}

    def handle_create_game(self, data: dict):
        title = data.get("title", "").strip()
        code = data.get("code", "").strip()

        if not title:
            return {"status": "error", "message": "O título do jogo é obrigatório."}
        if not code:
            return {"status": "error", "message": "O código Python é obrigatório."}

        # 1. Validação AST
        val = ast_validator.validate_game_code(code)
        if not val["valid"]:
            return {
                "status": "error",
                "message": "Código bloqueado pela Sandbox AST",
                "errors": val["errors"]
            }

        # 2. Gerar slug do ID
        slug = re.sub(r'[^a-z0-9_]+', '_', title.lower()).strip('_')
        if not slug:
            slug = f"game_{abs(hash(title)) % 100000}"

        game_dir = os.path.join(GAMES_DIR, slug)
        os.makedirs(game_dir, exist_ok=True)

        # 3. Salvar main.py
        main_path = os.path.join(game_dir, "main.py")
        with open(main_path, "w", encoding="utf-8") as f:
            f.write(code)

        # 4. Salvar Capa (cover.png ou cover.svg)
        cover_data = data.get("cover_data")
        cover_svg = data.get("cover_svg")

        if cover_data and isinstance(cover_data, str):
            try:
                if "," in cover_data:
                    cover_data = cover_data.split(",", 1)[1]
                img_bytes = base64.b64decode(cover_data)
                with open(os.path.join(game_dir, "cover.png"), "wb") as f:
                    f.write(img_bytes)
            except Exception:
                pass
        elif cover_svg and isinstance(cover_svg, str):
            try:
                with open(os.path.join(game_dir, "cover.svg"), "w", encoding="utf-8") as f:
                    f.write(cover_svg)
            except Exception:
                pass

        # 5. Criar manifest.json
        manifest = {
            "id": slug,
            "title": title,
            "author": data.get("author", "Comunidade"),
            "category": data.get("category", "Arcade Comunitário"),
            "players": data.get("players", "1 Jogador"),
            "license": data.get("license", "MIT"),
            "repo_url": data.get("repo_url", ""),
            "description": data.get("description", "Jogo criado no Arcade Hub."),
            "controls": data.get("controls", {"movement": "Setas ou WASD", "action": "Espaço / Clique"}),
            "engine": "Pygame In-Window",
            "type": "community",
            "featured": False,
            "security_verified": True,
            "security_errors": [],
            "sha256": val["sha256"]
        }

        with open(os.path.join(game_dir, "manifest.json"), "w", encoding="utf-8") as f:
            json.dump(manifest, f, indent=2, ensure_ascii=False)

        # 6. Sincronizar catálogo
        scan_and_sync_games()

        return {
            "status": "success",
            "message": f"Jogo '{title}' criado e catalogado com sucesso!",
            "game": manifest
        }

    def serve_game_asset(self, path: str):
        rel_path = path.lstrip("/")
        full_path = os.path.abspath(os.path.join(BASE_DIR, rel_path))

        # Prevenção contra Path Traversal
        if not full_path.startswith(GAMES_DIR) or not os.path.exists(full_path):
            self.send_response(404)
            self.end_headers()
            return

        ext = os.path.splitext(full_path)[1].lower()
        mime_types = {
            ".png": "image/png",
            ".jpg": "image/jpeg",
            ".jpeg": "image/jpeg",
            ".svg": "image/svg+xml",
            ".json": "application/json"
        }
        content_type = mime_types.get(ext, "application/octet-stream")

        try:
            with open(full_path, "rb") as f:
                content = f.read()
            self.send_response(200)
            self.send_header("Content-Type", content_type)
            self.send_header("Content-Length", str(len(content)))
            self.send_header("Access-Control-Allow-Origin", "*")
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(content)
        except Exception:
            self.send_response(500)
            self.end_headers()

    def send_json_response(self, data, status=200):
        body = json.dumps(data, ensure_ascii=False).encode('utf-8')
        self.send_response(status)
        self.send_header('Content-Type', 'application/json; charset=utf-8')
        self.send_header('Content-Length', str(len(body)))
        self.send_header('Connection', 'close')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()
        self.wfile.write(body)

    def do_OPTIONS(self):
        self.send_response(200)
        self.send_header('Connection', 'close')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()

    def get_scores(self):
        if os.path.exists(SCORES_FILE):
            with open(SCORES_FILE, "r", encoding="utf-8") as f:
                return json.load(f)
        return {}

    def save_score(self, data):
        game_id = data.get("game_id")
        name = data.get("name", "Jogador")
        score = data.get("score", 0)
        level = data.get("level", 1)
        scores = self.get_scores()
        if game_id not in scores:
            scores[game_id] = []
        scores[game_id].append({"name": name, "score": score, "level": level, "date": "hoje"})
        scores[game_id].sort(key=lambda x: x.get("score", 0), reverse=True)
        scores[game_id] = scores[game_id][:10]
        with open(SCORES_FILE, "w", encoding="utf-8") as f:
            json.dump(scores, f, indent=2, ensure_ascii=False)
        return {"status": "ok", "scores": scores[game_id]}


def run(port=8090):
    ThreadingHTTPServer.allow_reuse_address = False
    server_address = ('127.0.0.1', port)
    httpd = ThreadingHTTPServer(server_address, ArcadeHandler)
    print(f"Arcade Hub Server ativo em http://localhost:{port}")
    httpd.serve_forever()

if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8090
    run(port)
