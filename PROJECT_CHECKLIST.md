# Checklist de Acompanhamento do Projeto — Arcade Hub (Estação Desktop 100% em Rust Puro)

## Fase 1: Diagnóstico & Identidade do Produto
- [x] Entidade do Produto definida (Launcher & Galeria Desktop Offline para Execução Nativa de Jogos Clássicos em Rust na Mesma Janela).
- [x] Proposta de Valor central (Inicialização instantânea `<50ms`, zero portas de rede, zero dependências externas e 60+ FPS via GPU).
- [x] Paleta de Cores Oficial estabelecida (Safira Escuro `#090C14` + Slate Naval `#101522` + Azul Cobalto Gamer `#2563EB` + Azul Gelo `#60A5FA` + Amarelo Dourado `#FACC15` + Coral `#F43F5E`).
- [x] Seções de Coleções Open-Source integradas com créditos explícitos, licença e links diretos para o GitHub.

## Fase 2: DNA Estético & Interface Vetorial (`arcade_hub_rust/src/theme.rs`)
- [x] Geometria de precisão com linhas nítidas de 1px, cantos estruturados e contraste elevado.
- [x] Sistema Tipográfico embutido em binário (`assets/font.ttf` e `assets/font_bold.ttf`).
- [x] Ícones vetoriais desenhados diretamente por código geométrico em GPU (Play, Pause, Restart, Close, Search, GitHub, External Link).

## Fase 3: Arquitetura de Telas & Navegação (`SECTIONS.md`)
- [x] Header fixo (`ui/header.rs`) com busca instantânea (`Ctrl+K`) e acesso ao modal `+ Adicionar Jogo`.
- [x] Hero Section (`ui/hero.rs`) destacando **Space Odyssey** com arte vetorial animada e painel tático.
- [x] Galeria responsiva (`ui/gallery.rs`) com filtros por coleção open-source e por categoria de jogo.
- [x] Gaveta lateral de detalhes (`ui/drawer.rs`) e modal de cadastro (`ui/add_modal.rs`).

## Fase 4: Motor Nativo em Rust Puro & Palco Integrado (`ArcadeHub.exe`)
- [x] Consolidação do repositório exclusivamente em **Rust Puro** (`Edition 2024` + `Macroquad 0.4`), eliminando a antiga implementação legada em HTML/Python.
- [x] Compilação de executável único (`ArcadeHub.exe`) portátil na raiz do projeto sem abrir janelas de terminal (`windows_subsystem = "windows"`).
- [x] Palco integrado (`ui/stage.rs`) executando todos os 11 jogos nativamente dentro da mesma janela com controles de pausa (`P`), reinício (`R`) e retorno (`ESC`).
- [x] Persistência local de recordes em `scores.json` (`models.rs`).

## Fase 5: Catálogo de 11 Jogos Nativos em Rust (`arcade_hub_rust/src/games/`)
- [x] **Space Odyssey** (`space_odyssey.rs`)
- [x] **Breakout Prism** (`breakout.rs`)
- [x] **Flappy Jetpack Ultra** (`flappy.rs`)
- [x] **Snake Arcade** (`snake.rs`)
- [x] **Pong Arcade** (`pong.rs`)
- [x] **Tron Lightcycles** (`tron.rs`)
- [x] **Cannon Ballistics** (`cannon.rs`)
- [x] **Memory Tiles** (`tiles.rs`)
- [x] **Minesweeper Grid** (`minesweeper.rs`)
- [x] **Four in a Row** (`connect4.rs`)
- [x] **Pac-Man Classic** (`pacman.rs`)

## Fase 6: Evolução Contínua dos Jogos — O Top dos Top da Modalidade
- [x] **Space Odyssey (The Apex Overhaul)**:
  - [x] Física 2D vetorial com aceleração, amortecimento suave e inclinação dinâmica (tilt) da fuselagem.
  - [x] Manobra tática de esquiva (Dash / Barrel Roll com `Shift`) com invulnerabilidade e afterimages holográficos.
  - [x] Arsenal evolutivo em 5 níveis (Dual Blaster -> Triple Vulcan -> Quintuple Scatter -> Photon Lance -> Hyper Swarm com micro-mísseis teleguiados).
  - [x] Super-Arma Quantum Nova EMP (`X` ou `Clique Direito`) com onda de choque expansiva que vaporiza projéteis inimigos e causa dano massivo de área.
  - [x] Escudo defletor regenerativo e integridade estrutural do casco (recarga automática após 4s fora de combate).
  - [x] Mecânica de Graze (raspão tático) que acelera recargas e multiplica pontuações de `1.0x` até `5.0x`.
  - [x] Fragmentação balística orgânica de asteroides (Titânicos -> Médios -> Detritos rápidos) e rochas de minério radioativo com veios de neon.
  - [x] Frota alienígena invasora com caças batedores ágeis em curva senoidal e lançadores de minas de proximidade em fragmentação radial.
  - [x] Chefe colossal de setor: Flagship Dreadnought com canhões laterais destrutíveis, fase de sobrecarga e núcleo exposto.
  - [x] Cápsulas táticas e cristais atraídos magneticamente por raio trator (*Tractor Beam*).
- [x] **Breakout Prism (Campaign & Power-Ups Overhaul)**:
  - [x] Campanha estruturada em 5 fases prismáticas com layouts distintos e 3 seletor de dificuldades (`1`, `2`, `3`).
  - [x] Blocos especiais de titânio, bombas explosivas em cadeia, canhões laser duplos e multi-bolas.
- [x] **Flappy Jetpack Ultra (Biomes & Power-Ups Overhaul)**:
  - [x] 4 biomas futuristas dinâmicos com múltiplas camadas de *parallax*.
  - [x] Coleta de moedas, escudos de força, *turbo boost*, canos oscilantes e 3 modos de dificuldade (`1`, `2`, `3`).
- [x] **Snake Cyber Neon (Mazes & Portals Overhaul)**:
  - [x] 5 labirintos dinâmicos, portais quânticos, frutas especiais, poder fantasma e 3 modos de dificuldade (`1`, `2`, `3`).
- [x] **Tron Cyber Cycles Neo (Tournament & Tactical AI Overhaul)**:
  - [x] Torneio Melhor de 5 em 4 arenas, *Turbo Boost* (`Espaço`), pulso EMP (`E`), salto quântico (`J`), portais e IA tática.

## Fase 7: Atmosfera & Interatividade Reativa de Fundo (`ui/background.rs`)
- [x] Grade de coordenadas táticas com spotlight reativo que ilumina eixos e desenha micro-cruzes de miramento na posição do cursor.
- [x] Constelação de 52 nós estelares com física de repulsão vetorial ao aproximar o mouse.
- [x] Feixes translúcidos de conexão entre estrelas próximas e traçado de mira direta até o ponteiro do mouse.
- [x] Executável nativo compilado em modo `--release` (`ArcadeHub.exe`) com 60+ FPS cravados em GPU.

