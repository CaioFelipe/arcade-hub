# Arquitetura de Segurança & Sandbox Comunitária — Arcade Hub

Este documento especifica o modelo de **Defesa em Profundidade** do **Arcade Hub**, garantindo que usuários possam baixar o Hub e executar jogos criados pela comunidade com segurança, sem risco de execução de código malicioso (*malware*, roubo de arquivos, conexões de rede não autorizadas ou travamento do sistema).

---

## Visão Geral das 3 Camadas de Proteção

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                   PIPELINE DE SEGURANÇA DO ARCADE HUB                       │
└─────────────────────────────────────────────────────────────────────────────┘
  [1. Portão de Entrada (CI/CD + AST Scanner)]
       ├── Valida Pull Requests da comunidade no GitHub e antes da execução
       ├── Allowlist estrita de imports (proíbe os, subprocess, socket, etc.)
       └── Bloqueia execução dinâmica (eval, exec, __import__, compile)
                                   │
                                   ▼
  [2. Guardião em Tempo de Execução (app/security_runner.py)]
       ├── Verifica integridade do jogo via Hash SHA-256 no catalog.json
       ├── Injeta Python Audit Hooks (PEP 578 - sys.addaudithook)
       ├── Bloqueia rede, subprocessos e leitura fora de /games/<game_id>/
       └── Oferece API controlada (app/hub_sdk.py) para salvar recordes
                                   │
                                   ▼
  [3. Isolamento de Processo & Recursos (Watchdog do Servidor)]
       ├── Execução desacoplada com ambiente de variáveis limpo (env={})
       └── Monitoramento de tempo, CPU e limite de memória RAM
```

---

## Camada 1: Análise Estática de AST (`app/ast_validator.py`)

Todo script `.py` dentro de `games/<game_id>/` passa por uma inspeção da **Árvore Sintática Abstrata (`ast`)** em dois momentos:
1. **No GitHub Actions (CI/CD)** quando um desenvolvedor envia um *Pull Request* com um novo jogo.
2. **Localmente no Hub** milissegundos antes de o jogo ser iniciado pelo `server.py`.

### 1.1. Política de Módulos (Allowlist)
Apenas módulos voltados para lógica de jogos, matemática e gráficos locais são permitidos:

| Categoria | Módulos Permitidos (`ALLOWLIST`) |
| :--- | :--- |
| **Motores Gráficos / UI** | `pygame`, `turtle`, `tkinter`, `freegames` |
| **Matemática & Física** | `math`, `cmath`, `random`, `statistics`, `fractions`, `decimal` |
| **Estruturas & Utilitários** | `time`, `datetime`, `collections`, `itertools`, `functools`, `dataclasses`, `enum`, `typing`, `copy`, `string`, `re`, `json`, `heapq`, `bisect`, `array` |
| **SDK Oficial do Hub** | `hub_sdk` |

### 1.2. Módulos Terminantemente Bloqueados (`BLOCKLIST`)
Qualquer tentativa de `import` ou `from ... import` dos módulos abaixo reprova o jogo imediatamente:
- **Sistema & Processos:** `os`, `sys`, `subprocess`, `shutil`, `pathlib`, `glob`, `tempfile`, `signal`, `pty`, `fcntl`, `msvcrt`, `winreg`
- **Rede & Internet:** `socket`, `ssl`, `http`, `urllib`, `requests`, `ftplib`, `smtplib`, `poplib`, `imaplib`, `telnetlib`, `xmlrpc`, `asyncio`
- **Baixo Nível & Serialização Insegura:** `ctypes`, `cffi`, `pickle`, `cPickle`, `marshal`, `shelve`, `mmap`, `gc`, `inspect`, `code`, `importlib`, `runpy`, `zipimport`, `pkgutil`

### 1.3. Funções Built-in e Atributos Proibidos na AST
Para impedir que um atacante contorne a lista de imports usando reflexão ou código ofuscado, o scanner AST barra:
- **Chamadas diretas a built-ins críticos:** `eval()`, `exec()`, `compile()`, `__import__()`, `open()`, `input()` (em jogos gráficos), `breakpoint()`, `globals()`, `locals()`, `vars()`, `getattr()`, `setattr()`, `delattr()`.
- **Acesso a Dunder Attributes (Escape de Sandbox):** `__builtins__`, `__subclasses__`, `__bases__`, `__mro__`, `__globals__`, `__code__`, `__closure__`, `__loader__`, `__spec__`.

---

## Camada 2: Guardião em Tempo de Execução (`app/security_runner.py`)

Em vez de o servidor executar `python games/<id>/main.py` diretamente, ele executa:
```bash
python app/security_runner.py --game-id <id>
```

O `security_runner.py` prepara o ambiente seguro antes de carregar o código do jogo:

### 2.1. Verificação de Integridade (`SHA-256`)
- Cada jogo aprovado possui o hash `sha256` do seu `main.py` registrado no `games/catalog.json`.
- Antes da execução, o runner calcula o hash do arquivo local e compara com o catálogo. Se houver divergência (modificação não autorizada), a execução é abortada ou exige confirmação explícita de modo desenvolvedor.

### 2.2. Python Audit Hooks (`sys.addaudithook` — PEP 578)
O runner registra um gancho de auditoria nativo no núcleo do CPython que intercepta eventos de sistema em nível C, mesmo que alguma biblioteca interna tente acioná-los:

- **Bloqueio Total de Rede:**
  - Eventos interceptados e bloqueados (`PermissionError`): `socket.connect`, `socket.bind`, `socket.send`, `socket.getaddrinfo`, `urllib.Request`.
- **Bloqueio Total de Execução de Comandos:**
  - Eventos interceptados e bloqueados: `os.system`, `os.exec`, `os.spawn`, `subprocess.Popen`, `os.fork`, `os.kill`.
- **Bloqueio de Carregamento de DLLs / Código Nativo Arbitrário:**
  - Eventos interceptados e bloqueados: `ctypes.dlopen`, `ctypes.dlsym`.
- **Jaula de Sistema de Arquivos (Chroot Lógico no evento `open`):**
  - Se o modo de abertura contiver escrita (`"w"`, `"a"`, `"x"`, `"+"`), o evento é **bloqueado** imediatamente.
  - Se for apenas leitura (`"r"`, `"rb"`), o caminho real (`os.path.realpath`) deve obrigatoriamente estar contido dentro de `games/<game_id>/` (para carregar sprites, fontes ou sons do próprio jogo) ou nos diretórios das bibliotecas gráficas padrão (`pygame`, `tkinter`). Qualquer tentativa de ler pastas do usuário (`C:\Users\...`) levanta `PermissionError`.

---

## Camada 3: SDK Seguro para Desenvolvedores (`app/hub_sdk.py`)

Como os jogos não têm permissão para abrir arquivos (`open()`) nem fazer requisições de rede, o **Arcade Hub** fornece o módulo `hub_sdk` injetado automaticamente no ambiente do jogo para operações legítimas:

```python
import hub_sdk

# 1. Registrar pontuação no placar local do Hub (validado e limitado a inteiros)
hub_sdk.submit_score(score=12500, level=4)

# 2. Salvar progresso/configurações do jogo (JSON sanitizado, limite máximo de 64 KB)
hub_sdk.save_state({"unlocked_ships": [1, 2], "volume": 0.8})

# 3. Carregar progresso salvo anteriormente
state = hub_sdk.load_state()
```

### Garantias do `hub_sdk`:
- **Isolamento de Namespace:** O jogo `snake` só consegue ler e gravar a chave `snake` dentro de `data/saves/<game_id>.json`.
- **Cota de Disco (Quota Protection):** O payload serializado em JSON é rejeitado se ultrapassar `64 KB`, impedindo ataques de exaustão de disco.
- **Sanitização de Tipos:** Aceita apenas tipos primitivos seguros (`dict`, `list`, `str`, `int`, `float`, `bool`).

---

## Camada 4: Isolamento de Processo & Governança de Contribuições

### 4.1. Sanitização de Ambiente no `subprocess.Popen`
Quando o `server.py` inicia o `security_runner.py`:
- **Variáveis de Ambiente Limpas:** O subprocesso recebe apenas as variáveis estritamente necessárias para renderização gráfica do sistema operacional (`SYSTEMROOT`, `DISPLAY`, `PATH` mínimo do Python), removendo chaves sensíveis que o usuário possa ter no terminal (como `AWS_SECRET_ACCESS_KEY`, `GITHUB_TOKEN`, `OPENAI_API_KEY`, etc.).
- **Watchdog de Processo:** O servidor mantém a referência do PID iniciado, permitindo encerrar o processo caso exceda limites de memória ou caso o usuário feche o jogo pela interface do Hub.

### 4.2. Esteira de Contribuição para Novos Jogos (GitHub)
Para que outros desenvolvedores contribuam com facilidade e segurança:
1. O desenvolvedor cria uma pasta `games/<novo_jogo>/` contendo apenas `main.py`, `manifest.json` e pasta opcional `assets/` (apenas `.png`, `.wav`, `.ogg`, `.ttf`).
2. Não é permitido incluir `requirements.txt`, arquivos binários (`.exe`, `.dll`, `.so`, `.pyd`) ou scripts de shell (`.bat`, `.ps1`, `.sh`).
3. Ao abrir o *Pull Request*, a Action de Segurança executa o `ast_validator.py` em todos os arquivos `.py`, valida o schema do `manifest.json` e verifica se há binários ocultos.
