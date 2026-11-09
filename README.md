<div align="center">

# Arcade Hub — Rust Native Edition

### Estação Desktop Offline & Launcher de Jogos Retrô 100% em Rust Puro

[![Rust](https://img.shields.io/badge/Rust-2024_Edition-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Macroquad](https://img.shields.io/badge/Engine-Macroquad_0.4-black.svg?style=flat-square)](https://macroquad.rs/)
[![FPS](https://img.shields.io/badge/Performance-60+_FPS_V--Sync-emerald.svg?style=flat-square)](#-arquitetura-e-performance)
[![Offline](https://img.shields.io/badge/Rede-100%25_Offline_(Zero_Portas)-purple.svg?style=flat-square)](#)
[![License: MIT](https://img.shields.io/badge/Licen%C3%A7a-MIT-yellow.svg?style=flat-square)](LICENSE)

*Galeria e estação desktop de jogos retrô clássicos construída 100% em Rust puro (zero Python, zero WebView/Electron), com inicialização instantânea (<50ms), renderização acelerada por GPU na mesma janela e zero dependências de rede.*

[Jogos Inclusos](#-catálogo-de-jogos) • [Arquitetura & Performance](#-arquitetura-e-performance) • [Como Executar](#-como-executar) • [Estrutura do Projeto](#-estrutura-do-projeto) • [Atalhos](#️-atalhos-rápidos)

</div>

---

## 🕹️ Visão Geral

O **Arcade Hub** é uma estação desktop offline escrita inteiramente em **Rust**, projetada para entregar uma experiência arcade autêntica, responsiva e sem camadas intermediárias.

Diferente de launchers híbridos baseados em Python, Electron, Chromium ou wrappers de WebView que consomem centenas de megabytes de RAM e dependem de servidores locais, o Arcade Hub roda diretamente sobre um motor gráfico nativo (**Macroquad / OpenGL**):

- **100% Rust Nativo**: Código-fonte unificado em Rust (`Edition 2024`), sem interpretadores externos, sem scripts Python e sem navegador embutido.
- **Executável Único & Portátil**: Binário autossuficiente (~6.0 MB compilado com fontes e todos os 11 jogos integrados).
- **Palco Integrado na Mesma Janela**: Todos os jogos e a galeria interativa compartilham o mesmo contexto gráfico acelerado por GPU, sem abrir janelas extras ou terminais.
- **Latência Zero & 60+ FPS**: Física determinística, sistemas de partículas e fundo espacial reativo ao cursor sincronizados via V-Sync.
- **Zero Portas de Rede**: 100% offline, sem servidores HTTP locais, sem WebSockets e sem alertas de firewall.

---

## ⚡ Arquitetura e Performance

Comparação direta entre a arquitetura nativa em Rust do Arcade Hub e alternativas baseadas em interpretadores ou WebView:

| Critério | Arcade Hub (100% Rust Puro) | Soluções Web / Electron / Híbridas |
| :--- | :--- | :--- |
| **Inicialização** | **< 50 ms (Instantânea)** | 3 a 8 segundos |
| **Consumo de Memória** | **~25 MB RAM** | 350 MB a 800 MB RAM |
| **Taxa de Quadros** | **60+ FPS fluidos (V-Sync por Hardware)** | Variável (sujeito a stuttering e GC) |
| **Portas de Rede** | **Zero portas (100% isolado)** | Portas locais abertas (`localhost:XXXX`) |
| **Processos no SO** | **1 único processo nativo** | 3 a 7 processos em segundo plano |
| **Tamanho em Disco** | **~6.0 MB (Executável único)** | 150 MB a 300 MB |

---

## 🎮 Catálogo de Jogos

O hub integra **11 clássicos completos** programados nativamente em Rust com física dedicada, efeitos de partículas, seletor de dificuldades e persistência local de recordes (`scores.json`):

| Jogo | Categoria | Mecânica Principal | Controles Recomendados |
| :--- | :--- | :--- | :--- |
| **Space Odyssey** | Ação & Reflexos | Combate espacial *Apex Overhaul* com inércia vetorial, esquiva *Dash*, arsenal evolutivo em 5 níveis, super-arma *Quantum Nova EMP*, mecânica de *Graze* e chefe *Flagship Dreadnought* | `WASD` / `Setas` + `Shift` (Dash) + `Espaço` (Tiro) + `X` (EMP) |
| **Breakout Prism** | Arcade Clássico | Campanha em 5 fases prismáticas com 3 modos de dificuldade, blocos de titânio, bombas em cadeia, canhões laser e multi-bolas | `Setas` / `A, D` / `Mouse` + `Espaço` (Laser) + `1, 2, 3` (Dificuldade) |
| **Flappy Jetpack Ultra** | Ação & Reflexos | Voo acrobático por 4 biomas futuristas em *parallax*, coleta de moedas, escudos de força, *turbo boost*, canos oscilantes e 3 dificuldades | `Espaço` / `Seta Cima` / `Clique` + `1, 2, 3` (Dificuldade) |
| **Snake Cyber Neon** | Arcade Clássico | Evolução cyberpunk da serpente com 5 labirintos dinâmicos, portais quânticos, frutas douradas, poder fantasma e 3 dificuldades | `Setas` / `WASD` + `1, 2, 3` (Dificuldade) |
| **Pong Arcade** | Arcade Clássico | Duelo solo contra inteligência artificial responsiva com física de ricochete angular | `W` / `S` ou `Setas Cima/Baixo` |
| **Tron Cyber Cycles Neo** | Ação & Reflexos | Torneio Melhor de 5 na grade de luz com *Turbo Boost*, pulso EMP, salto quântico, portais e IA tática em 4 arenas | `Setas` / `WASD` + `Espaço` (Turbo) + `E` (EMP) + `J` (Salto) |
| **Cannon Ballistics** | Puzzle & Raciocínio | Artilharia balística com ajuste angular e potência parabólica contra alvos dinâmicos em movimento | `Setas Cima/Baixo` + `Espaço` / `Clique` |
| **Memory Tiles** | Puzzle & Raciocínio | Quebra-cabeça clássico de 15 peças deslizantes com embaralhamento matematicamente solucionável | `Setas` ou `Clique na peça adjacente` |
| **Minesweeper Grid** | Estratégia & Tabuleiro | Campo minado estratégico 10x10 com primeiro clique 100% seguro, expansão em cascata e bandeiras | `Clique esq.` (revelar) / `Clique dir.` (bandeira) |
| **Four in a Row** | Estratégia & Tabuleiro | Conexão tática de 4 fichas alinhadas na vertical, horizontal ou diagonal contra inteligência artificial | `Setas Esq./Dir.` / `Teclas 1 a 7` + `Espaço` / `Clique` |
| **Pac-Man Classic** | Arcade Clássico | Navegação em labirinto retrô com coleta de pílulas, animação de *chomp* e fantasmas com IA de perseguição | `Setas direcionais` ou `WASD` |

---

## 🚀 Como Executar

### 1. Executável Compilado (Recomendado)
Execute diretamente o binário portátil na raiz do projeto:
```powershell
.\ArcadeHub.exe
```
*(ou execute a partir de `arcade_hub_rust/ArcadeHub.exe`)*

---

### 2. Compilação a partir do Código-Fonte
Requisitos: [Rust & Cargo](https://rustup.rs/) instalados.

```bash
# Entre na pasta do motor Rust
cd arcade_hub_rust

# Execute diretamente em modo otimizado pelo Cargo
cargo run --release

# Ou gere apenas o binário final
cargo build --release
```
O executável final será gerado em `arcade_hub_rust/target/release/ArcadeHub.exe`.

---

## 📁 Estrutura do Projeto

```text
arcade-hub/
├── arcade_hub_rust/             # Motor Nativo Desktop em Rust Puro (Macroquad)
│   ├── Cargo.toml               # Manifesto e dependências (macroquad, serde, serde_json)
│   ├── Cargo.lock               # Lockfile reproduzível de dependências
│   ├── assets/                  # Tipografias vetoriais embutidas (.ttf)
│   ├── scores.json              # Banco de recordes locais persistidos
│   ├── ArcadeHub.exe            # Binário compilado do motor Rust
│   └── src/
│       ├── main.rs              # Loop principal, estado da aplicação e roteamento de telas
│       ├── theme.rs             # Design tokens (Paleta Safira), tipografia e ícones vetoriais
│       ├── models.rs            # Catálogo de jogos, metadados e persistência de recordes
│       ├── ui/                  # Interface gráfica nativa (Header, Hero, Galeria, Drawer, Palco, Fundo)
│       └── games/               # Implementação dos 11 jogos nativos em Rust
│           ├── space_odyssey.rs
│           ├── breakout.rs
│           ├── flappy.rs
│           ├── snake.rs
│           ├── pong.rs
│           ├── tron.rs
│           ├── cannon.rs
│           ├── tiles.rs
│           ├── minesweeper.rs
│           ├── connect4.rs
│           └── pacman.rs
├── ArcadeHub.exe                # Executável portátil na raiz
├── PROJECT_CHECKLIST.md         # Histórico de engenharia e evolução dos jogos
├── SECTIONS.md                  # Especificação arquitetural de interface
├── LICENSE                      # Licença MIT
└── README.md                    # Documentação principal do repositório
```

---

## ⌨️ Atalhos Rápidos

| Atalho | Ação |
| :--- | :--- |
| `Ctrl + K` | Ativar busca instantânea de jogos na galeria |
| `Esc` | Fechar gaveta/modal ou retornar do jogo para a galeria (salvando recorde) |
| `P` | Pausar ou retomar a partida em andamento |
| `R` | Reiniciar a partida instantaneamente |
| `1`, `2`, `3` | Alternar nível de dificuldade nos jogos compatíveis (*Breakout Prism*, *Flappy Jetpack Ultra*) |

---

## 🎖️ Créditos e Inspirações

- **Motor & Engenharia em Rust** — Todos os 11 jogos, sistemas de partículas, física 2D e interface vetorial foram integralmente reescritos em **Rust Puro** utilizando [Macroquad](https://macroquad.rs/).
- **Grant Jenks** e **Al Sweigart** — Inspiração conceitual nas regras clássicas de fliperama e tabuleiros lógicos de código aberto.

---

## 📄 Licença

Distribuído sob a licença [MIT](LICENSE).
