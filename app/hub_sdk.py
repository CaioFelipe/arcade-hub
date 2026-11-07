"""
hub_sdk.py - SDK Oficial de Persistência e Placar Seguro do Arcade Hub
Oferece API controlada para scripts de jogos salvarem pontuações e estados
com validação de cotas (max 64 KB) e tipos seguros.
"""
import os
import json
from datetime import datetime
from typing import Any, Dict

import sys

# Localização de armazenamento no projeto (compatível com PyInstaller frozen)
if getattr(sys, 'frozen', False):
    BASE_DIR = os.path.dirname(sys.executable)
else:
    BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DATA_DIR = os.path.join(BASE_DIR, "data")
SAVES_DIR = os.path.join(DATA_DIR, "saves")
SCORES_FILE = os.path.join(DATA_DIR, "scores.json")

os.makedirs(SAVES_DIR, exist_ok=True)

# Identificador do jogo atualmente em execução
_CURRENT_GAME_ID = "default"

MAX_STATE_BYTES = 65536  # 64 KB


def set_current_game(game_id: str) -> None:
    """Define o identificador do jogo ativo para isolamento de saves e scores."""
    global _CURRENT_GAME_ID
    if game_id and isinstance(game_id, str):
        # Sanitizar nome para evitar path traversal
        clean_id = "".join(c for c in game_id if c.isalnum() or c in ("-", "_")).strip()
        _CURRENT_GAME_ID = clean_id if clean_id else "default"
    else:
        _CURRENT_GAME_ID = "default"


def _is_safe_primitive(value: Any) -> bool:
    """Verifica recursivamente se a estrutura contém apenas tipos primitivos seguros."""
    if value is None or isinstance(value, (str, int, float, bool)):
        return True
    if isinstance(value, list):
        return all(_is_safe_primitive(item) for item in value)
    if isinstance(value, dict):
        return all(isinstance(k, str) and _is_safe_primitive(v) for k, v in value.items())
    return False


def submit_score(score: int, level: int = 1, player_name: str = "Jogador Local") -> bool:
    """
    Valida e registra a pontuação do jogo atual em data/scores.json.
    Mantém apenas o top 10 ordenado de forma decrescente.
    """
    # Validações estritas de tipo
    if not isinstance(score, int) or isinstance(score, bool):
        return False
    if not isinstance(level, int) or isinstance(level, bool):
        return False

    name_str = str(player_name).strip()[:32] if player_name else "Jogador Local"
    date_str = datetime.now().strftime("%d/%m/%Y %H:%M")

    record = {
        "name": name_str,
        "score": int(score),
        "level": int(level),
        "date": date_str
    }

    try:
        scores: Dict[str, Any] = {}
        if os.path.exists(SCORES_FILE):
            with open(SCORES_FILE, "r", encoding="utf-8") as f:
                try:
                    scores = json.load(f)
                    if not isinstance(scores, dict):
                        scores = {}
                except Exception:
                    scores = {}

        if _CURRENT_GAME_ID not in scores or not isinstance(scores[_CURRENT_GAME_ID], list):
            scores[_CURRENT_GAME_ID] = []

        scores[_CURRENT_GAME_ID].append(record)
        scores[_CURRENT_GAME_ID].sort(key=lambda x: x.get("score", 0), reverse=True)
        scores[_CURRENT_GAME_ID] = scores[_CURRENT_GAME_ID][:10]

        temp_file = SCORES_FILE + ".tmp"
        with open(temp_file, "w", encoding="utf-8") as f:
            json.dump(scores, f, indent=2, ensure_ascii=False)
        os.replace(temp_file, SCORES_FILE)
        return True
    except Exception:
        return False


def save_state(state: dict) -> bool:
    """
    Salva o estado do jogo em data/saves/<game_id>.json.
    Exige serialização em JSON com no máximo 64 KB e tipos primitivos seguros.
    """
    if not isinstance(state, dict):
        return False

    if not _is_safe_primitive(state):
        return False

    try:
        raw_json = json.dumps(state, ensure_ascii=False)
        if len(raw_json.encode("utf-8")) > MAX_STATE_BYTES:
            return False

        save_path = os.path.join(SAVES_DIR, f"{_CURRENT_GAME_ID}.json")
        temp_path = save_path + ".tmp"
        with open(temp_path, "w", encoding="utf-8") as f:
            f.write(raw_json)
        os.replace(temp_path, save_path)
        return True
    except Exception:
        return False


def load_state() -> dict:
    """
    Carrega o estado salvo de data/saves/<game_id>.json ou retorna dicionário vazio.
    """
    save_path = os.path.join(SAVES_DIR, f"{_CURRENT_GAME_ID}.json")
    if not os.path.exists(save_path):
        return {}

    try:
        with open(save_path, "r", encoding="utf-8") as f:
            data = json.load(f)
            if isinstance(data, dict):
                return data
    except Exception:
        pass
    return {}
