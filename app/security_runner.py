"""
security_runner.py - Carregador e Sandbox em Memória do Arcade Hub
Valida integridade criptográfica (SHA-256), audita AST e executa jogos em namespace restrito.
"""
import os
import sys
import json
import builtins
import argparse

# Garante acesso aos módulos internos
CURRENT_DIR = os.path.dirname(os.path.abspath(__file__))
if CURRENT_DIR not in sys.path:
    sys.path.insert(0, CURRENT_DIR)

import ast_validator
import hub_sdk

if getattr(sys, 'frozen', False):
    BASE_DIR = os.path.dirname(sys.executable)
else:
    BASE_DIR = os.path.dirname(CURRENT_DIR)
GAMES_DIR = os.path.join(BASE_DIR, "games")
CATALOG_FILE = os.path.join(GAMES_DIR, "catalog.json")


def install_audit_hook():
    """Registra gancho de auditoria PEP 578 para bloquear rede, subprocessos e DLLs."""
    def _audit_hook(event, args):
        blocked_events = {
            "os.system", "os.spawn", "os.exec", "os.fork", "os.kill",
            "subprocess.Popen", "socket.connect", "socket.bind", "socket.send",
            "socket.getaddrinfo", "urllib.Request", "ctypes.dlopen", "ctypes.dlsym"
        }
        if event in blocked_events:
            raise PermissionError(f"Ação bloqueada pelo Security Runner: evento de sistema proibido '{event}'.")

    try:
        sys.addaudithook(_audit_hook)
    except Exception:
        pass


def create_safe_builtins():
    """Cria um dicionário de builtins restritos sem funções perigosas."""
    if isinstance(builtins, dict):
        safe = dict(builtins)
    else:
        safe = dict(builtins.__dict__)

    # Remove todos os built-ins proibidos
    for forbidden in ast_validator.FORBIDDEN_BUILTINS:
        safe.pop(forbidden, None)

    # Substitui __import__ pela versão validada contra a ALLOWLIST
    def _safe_import(name, globals=None, locals=None, fromlist=(), level=0):
        root = name.split('.')[0]
        if root not in ast_validator.ALLOWLIST or root in ast_validator.BLOCKLIST:
            raise PermissionError(f"Módulo proibido em tempo de execução: '{name}'. Permitidos apenas allowlist.")
        if root == "hub_sdk":
            return hub_sdk
        return __import__(name, globals, locals, fromlist, level)

    safe["__import__"] = _safe_import
    return safe


def load_verified_game_class(game_id: str, game_path: str, expected_sha256: str = None):
    """
    Valida e carrega em memória a classe `Game` definida no script do jogo.
    1. Executa validação estática via ast_validator.validate_game_file.
    2. Compara o hash SHA-256 com o valor esperado (se fornecido).
    3. Constrói ambiente com safe_builtins.
    4. Executa em namespace isolado e retorna a classe Game.
    """
    if not os.path.exists(game_path):
        raise FileNotFoundError(f"Arquivo do jogo não encontrado: {game_path}")

    # 1. Validação estática
    validation = ast_validator.validate_game_file(game_path)
    if not validation["valid"]:
        errors_joined = "; ".join(validation["errors"])
        raise PermissionError(f"Falha na validação de segurança do código: {errors_joined}")

    # 2. Verificação de Assinatura SHA-256
    current_sha256 = validation["sha256"]
    if expected_sha256 and expected_sha256.lower() != current_sha256.lower():
        raise PermissionError(
            f"Assinatura SHA-256 divergente para o jogo '{game_id}'. Esperado: {expected_sha256}, Obtido: {current_sha256}"
        )

    # 3. Preparar safe_builtins e ambiente isolado
    safe_builtins = create_safe_builtins()

    # Informa ao SDK qual jogo está sendo executado
    hub_sdk.set_current_game(game_id)

    with open(game_path, "r", encoding="utf-8") as f:
        source_code = f.read()

    # 4. Compilação e Execução no Namespace
    code_obj = compile(source_code, game_path, "exec")
    namespace = {
        "__builtins__": safe_builtins,
        "__name__": "__game_module__",
        "__file__": os.path.abspath(game_path),
        "hub_sdk": hub_sdk
    }

    exec(code_obj, namespace)

    # 5. Retorna a classe Game (ou None)
    game_class = namespace.get("Game")
    return game_class


def run_standalone_game(game_id: str):
    """Inicializa e executa o jogo em janela independente Pygame."""
    game_path = os.path.join(GAMES_DIR, game_id, "main.py")
    if not os.path.exists(game_path):
        print(f"Erro: Jogo '{game_id}' não encontrado em {game_path}")
        return

    # Buscar hash esperado no manifest do jogo ou no catálogo
    expected_sha256 = None
    manifest_path = os.path.join(GAMES_DIR, game_id, "manifest.json")
    if os.path.exists(manifest_path):
        try:
            with open(manifest_path, "r", encoding="utf-8") as f:
                manifest_data = json.load(f)
                expected_sha256 = manifest_data.get("sha256")
        except Exception:
            pass

    install_audit_hook()

    try:
        GameClass = load_verified_game_class(game_id, game_path, expected_sha256)
    except PermissionError as e:
        print(f"[ERRO DE SEGURANÇA]: {e}")
        return
    except Exception as e:
        print(f"[ERRO AO CARREGAR JOGO]: {e}")
        return

    if GameClass is None:
        print(f"Aviso: Classe 'Game' não encontrada em {game_path}.")
        return

    # Executar jogo com Pygame
    try:
        import pygame
        pygame.init()
        width, height = 800, 600
        screen = pygame.display.set_mode((width, height))
        pygame.display.set_caption(f"Arcade Hub — {game_id.replace('_', ' ').title()}")
        clock = pygame.time.Clock()
        font = pygame.font.SysFont("Arial", 18, bold=True)

        game_instance = GameClass(width, height)
        running = True
        while running:
            clock.tick(60)
            events = pygame.event.get()
            for event in events:
                if event.type == pygame.QUIT or (event.type == pygame.KEYDOWN and event.key == pygame.K_ESCAPE):
                    running = False

            keys = pygame.key.get_pressed()
            game_instance.update(keys, events)
            game_instance.draw(screen, font)
            pygame.display.flip()

        pygame.quit()
    except Exception as e:
        print(f"Erro na execução do jogo: {e}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Security Runner do Arcade Hub")
    parser.add_argument("--game-id", type=str, default=None, help="ID do jogo a ser executado")
    parser.add_argument("game_pos", nargs="?", default=None, help="ID posicional do jogo")
    args = parser.parse_args()

    target_game = args.game_id or args.game_pos
    if target_game:
        run_standalone_game(target_game)
    else:
        print("Uso: python app/security_runner.py --game-id <id>")
