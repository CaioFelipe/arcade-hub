<div align="center">

# Arcade Hub

### Estação Desktop Offline & Launcher de Jogos Retrô em Rust Puro

[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![Macroquad](https://img.shields.io/badge/Engine-Macroquad_0.4-black.svg?style=flat-square)](https://macroquad.rs/)
[![FPS](https://img.shields.io/badge/Performance-60+_FPS_V--Sync-emerald.svg?style=flat-square)](#-arquitetura-e-performance)
[![Offline](https://img.shields.io/badge/Rede-100%25_Offline_(Zero_Portas)-purple.svg?style=flat-square)](#)
[![License: MIT](https://img.shields.io/badge/Licen%C3%A7a-MIT-yellow.svg?style=flat-square)](LICENSE)

*Galeria e launcher desktop de jogos retrô clássicos em Rust puro, com inicialização instantânea (<50ms), renderização 100% acelerada por hardware na mesma janela e zero dependências de rede.*

[Jogos Inclusos](#-catálogo-de-jogos) • [Arquitetura & Performance](#-arquitetura-e-performance) • [Como Executar](#-como-executar) • [Estrutura do Projeto](#-estrutura-do-projeto) • [Atalhos](#️-atalhos-rápidos)

</div>

---

## 🕹️ Visão Geral

O **Arcade Hub** é uma estação desktop offline focada em oferecer uma experiência retrô autêntica, fluida e direta ao ponto. 

Diferente de launchers modernos baseados em Electron, Chromium ou wrappers de WebView que consomem gigabytes de memória e abrem dezenas de processos em segundo plano, o Arcade Hub foi construído em **Rust Puro**:

- **Executável Único & Leve**: Apenas ~4.1 MB compilado.
- **Palco Integrado**: Todos os jogos rodam nativamente dentro da mesma janela da galeria, sem abrir janelas extras, pop-ups ou terminais pretos.
- **Latência Zero & 60+ FPS**: Física direta e renderização gráfica acelerada por GPU sincronizada com a taxa de atualização do monitor.
- **Zero Portas de Rede**: 100% offline, sem servidores HTTP locais, sem WebSockets e sem riscos de bloqueio por firewall ou antivírus.

---

## ⚡ Arquitetura e Performance

Comparação direta entre a arquitetura em Rust do Arcade Hub e alternativas baseadas em web/navegador:

| Critério | Arcade Hub (Rust Puro) | Soluções Web / Electron Convencionais |
| :--- | :--- | :--- |
| **Inicialização** | **< 50 ms (Instantânea)** | 4 a 8 segundos |
| **Consumo de Memória** | **~25 MB RAM** | 350 MB a 800 MB RAM |
| **Taxa de Quadros** | **60+ FPS fluidos (V-Sync por Hardware)** | Variável (sujeito a stuttering do browser) |
| **Portas de Rede** | **Zero portas (100% isolado)** | Portas locais abertas (`localhost:XXXX`) |
| **Processos no SO** | **1 único processo nativo** | 3 a 7 processos em segundo plano |
| **Tamanho em Disco** | **~4.1 MB (Executável)** | 150 MB a 300 MB |

---

## 🎮 Catálogo de Jogos

O hub integra **11 clássicos completos** implementados diretamente em Rust com física dedicada, efeitos visuais e controle de recordes locais:

| Jogo | Categoria | Mecânica Principal | Controles Recomendados |
| :--- | :--- | :--- | :--- |
| **Space Odyssey** | Ação Espacial | Propulsão vetorial com física de inércia, tiros laser duplos, chuva contínua de asteroides e explosões de partículas | `Setas` / `WASD` + `Espaço` |
| **Snake Arcade** | Arcade Retrô | Serpente ágil em grade de coordenadas, maçãs nutritivas e aumento dinâmico de velocidade | `Setas direcionais` |
| **Pong Arcade** | Esporte Clássico | Duelo solo contra inteligência artificial responsiva com física de ricochete angular | `W` / `S` ou `Setas Cima/Baixo` |
| **Breakout Prism** | Destruição / Reflexos | Demolição de grades de blocos prismáticos coloridos com rebote acelerado e contagem de vidas | `Setas Esquerda/Direita` |
| **Flappy Jetpack** | Habilidade / Timing | Impulso gravitacional rítmico entre colunas de obstáculos com progressão dinâmica | `Espaço` ou `Clique esquerdo` |
| **Tron Lightcycles** | Duelo Tático | Disputa em arena fechada neon contra IA rival evitando cruzamento de trilhas de luz | `Setas direcionais` |
| **Cannon Ballistics** | Estratégia / Balística | Ajuste angular e potência de tiro parabólico contra alvos dinâmicos em movimento | `Setas` + `Enter` |
| **Memory Tiles** | Raciocínio / Quebra-Cabeça | Jogo clássico de 15 peças deslizantes com gerador de embaralhamento 100% solucionável | `Clique no bloco` |
| **Minesweeper Grid** | Tabuleiro de Lógica | Campo minado clássico 10x10 com primeiro clique seguro, expansão em cascata e marcadores | `Clique esq.` (revelar) / `dir.` (bandeira) |
| **Four in a Row** | Tabuleiro Tático | Conexão de 4 fichas alinhadas em grade vertical contra inteligência artificial | `Colunas 1 a 7` / `Clique` |
| **Pac-Man Classic** | Arcade de Labirinto | Navegação em labirinto retrô com coleta de pastilhas, animação de chomp e IA de fantasmas | `Setas direcionais` |

---

## 🚀 Como Executar

### 1. Executável Compilado (Recomendado)
Execute diretamente o binário na raiz do projeto:
```powershell
.\ArcadeHub.exe
```
*(ou execute a partir de `arcade_hub_rust/ArcadeHub.exe`)*

---

### 2. Compilação a partir do Código-Fonte
Requisitos: [Rust & Cargo](https://rustup.rs/) instalados.

```bash
# Entre na pasta do projeto Rust
cd arcade_hub_rust

# Execute diretamente pelo Cargo
cargo run --release

# Ou apenas gere o executável otimizado
cargo build --release
```
O executável final será gerado em `arcade_hub_rust/target/release/ArcadeHub.exe`.

---

## 📁 Estrutura do Projeto

```
arcade-hub/
├── arcade_hub_rust/             # Motor Nativo Desktop em Rust (Macroquad)
│   ├── Cargo.toml               # Dependências do projeto (macroquad, serde, etc.)
│   ├── Cargo.lock               # Lockfile reproduzível de dependências
│   ├── assets/                  # Tipografias vetoriais embutidas (.ttf)
│   ├── scores.json              # Banco de recordes locais persistidos
│   ├── ArcadeHub.exe            # Binário compilado do motor Rust
│   └── src/
│       ├── main.rs              # Loop principal da aplicação e roteamento de telas
│       ├── theme.rs             # Design tokens, paleta de cores e render de componentes
│       ├── models.rs            # Modelos de dados, catálogo de jogos e scores
│       ├── ui/                  # Telas e layouts (Header, Hero, Galeria, Palco)
│       └── games/               # Implementação dos 11 jogos nativos em Rust
│           ├── space_odyssey.rs
│           ├── snake.rs
│           ├── pong.rs
│           ├── breakout.rs
│           ├── flappy.rs
│           ├── tron.rs
│           ├── cannon.rs
│           ├── tiles.rs
│           ├── minesweeper.rs
│           ├── connect4.rs
│           └── pacman.rs
├── ArcadeHub.exe                # Executável portátil na raiz
├── PROJECT_CHECKLIST.md         # Acompanhamento de engenharia e UI
├── SECTIONS.md                  # Especificação arquitetural de interfaces
├── LICENSE                      # Licença MIT
└── README.md                    # Documentação oficial do repositório
```

---

## ⌨️ Atalhos Rápidos

| Atalho | Ação |
| :--- | :--- |
| `Ctrl + K` | Abrir busca instantânea de jogos na galeria |
| `Esc` | Pausar partida em andamento ou retornar para a galeria |
| `Espaço` / `Enter` | Iniciar jogo em destaque ou confirmar ação |
| `F11` | Alternar entre janela normal e tela cheia |

---

## 🎖️ Créditos e Reconhecimentos

- **Grant Jenks** (*Free Python Games*) — Lógica clássica de regras dos jogos retrô (Snake, Pacman, Pong, Tron, Cannon, Tiles).
- **Al Sweigart** (*Invent with Python*) — Algoritmos didáticos de simulação e lógica de tabuleiro.
- **Equipe Macroquad & Comunidade Rust** — Motor gráfico 2D ultraleve e performático para Rust.

---

## 📄 Licença

Distribuído sob a licença [MIT](LICENSE).
