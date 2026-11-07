# Arcade Hub — Galeria & Launcher Desktop de Jogos Clássicos

Estação desktop offline e launcher de jogos clássicos, oferecendo execução instantânea na mesma janela, alto desempenho gráfico e arquitetura modular. O projeto conta com duas vertentes complementares de engenharia: uma implementação nativa de ultra-baixo consumo em **Rust Puro**, e um ambiente extensível com sandbox de segurança em **Python & Web**.

---

## Destaques da Arquitetura

### 1. Motor Nativo em Rust Puro (`arcade_hub_rust/`)
- **Zero Dependências Externas**: Sem Chromium, sem Electron, sem WebView e sem necessidade de servidores HTTP ou portas de rede locais.
- **Aceleração Gráfica por Hardware**: Renderização direta em 60+ FPS com física e resposta imediata de controles via Macroquad.
- **Execução na Mesma Janela**: Os 11 jogos rodam de forma nativa e integrada sem pop-ups ou múltiplas janelas.
- **Startup Instantâneo**: Inicialização em menos de 50 milissegundos e consumo mínimo de memória RAM.

### 2. Hub Extensível com Defesa em Profundidade (`app/` & `games/`)
- **Camada 1 — Scanner Estático de AST (`app/ast_validator.py`)**: Allowlist de módulos seguros e bloqueio de chamadas perigosas (`os`, `sys`, `subprocess`, `socket`, `eval`, `exec`).
- **Camada 2 — Guardião de Runtime (`app/security_runner.py`)**: Python Audit Hooks (`sys.addaudithook` / PEP 578) bloqueando acesso não autorizado ao sistema de arquivos e rede.
- **Camada 3 — SDK Oficial de Jogos (`app/hub_sdk.py`)**: Interface padronizada para registro seguro de recordes e persistência local sanitizada.
- **Estúdio "+ Adicionar Jogo"**: Interface padronizada para importar novos jogos da comunidade com validação de código em tempo real.

---

## Jogos Integrados

1. **Space Odyssey**: Combate espacial com propulsão a jato, disparos laser duplos, chuva de asteroides e explosões de partículas.
2. **Snake Arcade**: Serpente ágil com grade de coordenadas, maçãs nutritivas e aumento dinâmico de velocidade.
3. **Pong Arcade**: Duelo clássico contra inteligência artificial responsiva e física de rebote por ângulo.
4. **Breakout Prism**: Demolição de tijolos prismáticos coloridos com rebote acelerado e contagem de vidas.
5. **Flappy Jetpack**: Impulso vertical rítmico entre colunas de obstáculos com física gravitacional suave.
6. **Tron Lightcycles**: Duelo de motos de luz na arena neon contra IA rival (evite cruzar rastros).
7. **Cannon Ballistics**: Artilharia com ajuste de ângulo e potência parabólica contra alvos dinâmicos.
8. **Memory Tiles**: Quebra-cabeça de 15 números deslizantes gerado por embaralhamento matematicamente resolvível.
9. **Minesweeper Grid**: Campo minado 10x10 com primeiro clique seguro, expansão em cascata e marcação de bandeiras.
10. **Four in a Row**: Conecte quatro fichas da mesma cor na vertical, horizontal ou diagonal contra a IA.
11. **Pac-Man Classic**: Labirinto retrô com pílulas, animação clássica de chomp e fantasmas com IA de perseguição.

---

## Como Executar

### Opção A: Executável Nativo Compilado
Basta executar o binário nativo:
```powershell
.\ArcadeHub.exe
```
ou dentro da pasta do motor Rust:
```powershell
cd arcade_hub_rust
.\ArcadeHub.exe
```

### Opção B: Compilação do Código Rust
Requisitos: Toolchain Rust / Cargo instalados.
```powershell
cd arcade_hub_rust
cargo run --release
```

### Opção C: Execução do Hub Python
Requisitos: Python 3.10+
```powershell
python app/main_desktop.py
```
*(ou inicie o servidor local através de `python app/server.py` e acesse via navegador).*

---

## Estrutura do Repositório

```
poject1/
├── arcade_hub_rust/         # Motor nativo em Rust (Macroquad)
│   ├── Cargo.toml           # Manifesto de dependências Rust
│   ├── assets/              # Tipografia e fontes vetoriais
│   ├── src/                 # Código-fonte (main, tema, modelos)
│   │   └── games/           # Implementação dos 11 jogos em Rust
│   └── scores.json          # Recordes locais persistidos
├── app/                     # Hub desktop e motor de segurança Python
│   ├── main_desktop.py      # Ponto de entrada da aplicação desktop
│   ├── game_engine.py       # Renderizador de jogos no canvas integrado
│   ├── ast_validator.py     # Scanner estático de segurança AST
│   ├── security_runner.py   # Sandbox com audit hooks (PEP 578)
│   ├── hub_sdk.py           # SDK seguro para jogos da comunidade
│   ├── server.py            # Servidor local de entrega de ativos
│   └── index.html           # Interface do Hub
├── games/                   # Catálogo de jogos em Python da comunidade
├── data/                    # Dados locais e placares
├── ArcadeHub.exe            # Executável compilado
├── PROJECT_CHECKLIST.md     # Rastreamento de requisitos e etapas
├── SECURITY_ARCHITECTURE.md # Documentação da arquitetura de segurança
└── SECTIONS.md              # Especificação de arquitetura de UI
```

---

## Créditos e Reconhecimentos

- **Grant Jenks** (*Free Python Games*) — Lógica clássica de jogos 2D (Snake, Pacman, Pong, Tron, Cannon, Tiles).
- **Al Sweigart** (*Invent with Python*) — Conceitos e algoritmos de simulações e quebra-cabeças.
- **Comunidade Rust & Macroquad** — Motor gráfico 2D minimalista e de alta performance.

---

## Licença

Distribuído sob a licença [MIT](LICENSE).
