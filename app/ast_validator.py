"""
ast_validator.py - Validador Estático de Código para Arcade Hub
Inspeciona a Árvore Sintática Abstrata (AST) para garantir que jogos da comunidade
cumprem rigorosamente as diretrizes de segurança antes da execução.
"""
import ast
import hashlib
from typing import Dict, List, Any

# Módulos estritamente permitidos para scripts de jogos
ALLOWLIST = {
    "pygame", "math", "cmath", "random", "statistics", "fractions", "decimal",
    "time", "datetime", "collections", "itertools", "functools", "dataclasses",
    "enum", "typing", "copy", "string", "re", "json", "heapq", "bisect",
    "array", "hub_sdk", "turtle", "freegames"
}

# Módulos explicitamente bloqueados (qualquer outro fora da ALLOWLIST também é bloqueado)
BLOCKLIST = {
    "os", "sys", "subprocess", "shutil", "pathlib", "glob", "tempfile",
    "signal", "pty", "fcntl", "msvcrt", "winreg", "socket", "ssl", "http",
    "urllib", "requests", "ftplib", "smtplib", "poplib", "imaplib", "telnetlib",
    "xmlrpc", "asyncio", "ctypes", "cffi", "pickle", "cPickle", "marshal",
    "shelve", "mmap", "gc", "inspect", "code", "importlib", "runpy",
    "zipimport", "pkgutil"
}

# Funções built-in proibidas que possibilitam execução arbitrária ou manipulação de estado
FORBIDDEN_BUILTINS = {
    "eval", "exec", "compile", "__import__", "open", "breakpoint",
    "globals", "locals", "vars", "getattr", "setattr", "delattr"
}

# Atributos especiais (dunders) proibidos para impedir escapes de sandbox
FORBIDDEN_DUNDERS = {
    "__builtins__", "__subclasses__", "__bases__", "__mro__",
    "__globals__", "__code__", "__closure__", "__loader__", "__spec__"
}


class GameSecurityVisitor(ast.NodeVisitor):
    def __init__(self):
        self.errors: List[str] = []

    def visit_Import(self, node: ast.Import):
        for alias in node.names:
            root_module = alias.name.split('.')[0]
            if root_module not in ALLOWLIST or root_module in BLOCKLIST:
                self.errors.append(
                    f"Linha {node.lineno}: Módulo '{alias.name}' não permitido. Permitidos apenas módulos da allowlist."
                )
        self.generic_visit(node)

    def visit_ImportFrom(self, node: ast.ImportFrom):
        if node.module:
            root_module = node.module.split('.')[0]
            if root_module not in ALLOWLIST or root_module in BLOCKLIST:
                self.errors.append(
                    f"Linha {node.lineno}: Importação do módulo '{node.module}' não permitida."
                )
        else:
            self.errors.append(
                f"Linha {node.lineno}: Importações relativas não são permitidas."
            )
        self.generic_visit(node)

    def visit_Call(self, node: ast.Call):
        # Bloquear chamadas a atributos que sejam built-ins proibidos (ex: builtins.eval)
        if isinstance(node.func, ast.Attribute):
            if node.func.attr in FORBIDDEN_BUILTINS:
                self.errors.append(
                    f"Linha {node.lineno}: Chamada proibida ao método/built-in '{node.func.attr}'."
                )
        self.generic_visit(node)

    def visit_Name(self, node: ast.Name):
        # Bloquear acesso a dunders perigosos ou referências a built-ins proibidos
        if node.id in FORBIDDEN_DUNDERS:
            self.errors.append(
                f"Linha {node.lineno}: Acesso proibido ao identificador restrito '{node.id}'."
            )
        elif node.id in FORBIDDEN_BUILTINS:
            self.errors.append(
                f"Linha {node.lineno}: Uso proibido da função built-in '{node.id}'."
            )
        self.generic_visit(node)

    def visit_Attribute(self, node: ast.Attribute):
        if node.attr in FORBIDDEN_DUNDERS:
            self.errors.append(
                f"Linha {node.lineno}: Acesso proibido ao atributo restrito '{node.attr}'."
            )
        self.generic_visit(node)


def calculate_sha256(source_code: str) -> str:
    """Calcula o hash SHA-256 do código-fonte em formato hexadecimal."""
    return hashlib.sha256(source_code.encode("utf-8")).hexdigest()


def validate_game_code(source_code: str, filename: str = "main.py") -> Dict[str, Any]:
    """
    Analisa estaticamente o código de um jogo.
    Retorna:
        {"valid": bool, "errors": list[str], "sha256": str}
    """
    sha256_hash = calculate_sha256(source_code)
    try:
        tree = ast.parse(source_code, filename=filename)
    except SyntaxError as e:
        return {
            "valid": False,
            "errors": [f"Erro de sintaxe na linha {e.lineno or 0}: {e.msg}"],
            "sha256": sha256_hash
        }
    except Exception as e:
        return {
            "valid": False,
            "errors": [f"Erro ao processar código: {str(e)}"],
            "sha256": sha256_hash
        }

    visitor = GameSecurityVisitor()
    visitor.visit(tree)

    return {
        "valid": len(visitor.errors) == 0,
        "errors": visitor.errors,
        "sha256": sha256_hash
    }


def validate_game_file(filepath: str) -> Dict[str, Any]:
    """
    Lê um arquivo de código e valida seu conteúdo estático.
    """
    try:
        with open(filepath, "r", encoding="utf-8") as f:
            content = f.read()
    except Exception as e:
        return {
            "valid": False,
            "errors": [f"Não foi possível ler o arquivo '{filepath}': {str(e)}"],
            "sha256": ""
        }
    return validate_game_code(content, filename=filepath)
