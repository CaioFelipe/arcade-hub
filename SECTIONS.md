# Estrutura de Seções & Anatomia de Interface — Arcade Hub (Rust Native Edition)

Este documento define a arquitetura de informação e a composição visual do **Arcade Hub**, a estação desktop offline de jogos clássicos construída 100% em **Rust Puro** com renderização nativa por GPU (**Macroquad**).

---

## 1. Top Bar & Header de Comando (`ui/header.rs`)
- **Objetivo**: Navegação rápida, acesso universal à busca e adição de jogos ao catálogo.
- **Elementos**:
  - Logo vetorial geométrico da marca (`ARCADE HUB`).
  - Barra de busca instantânea com atalho `Ctrl + K` (filtro em tempo real por título, autor ou categoria).
  - Botão de ação `+ Adicionar Jogo` para abertura do modal de cadastro.
- **Padrão de UX**: Limpo, sem ruídos de telemetria, sem badges decorativos vazios e com ícones desenhados via primitivas geométricas.

---

## 2. Hero Section: Destaque Principal & Partida Rápida (`ui/hero.rs`)
- **Objetivo**: Chamar a atenção imediata na primeira dobra (*above the fold*), permitindo iniciar o jogo principal com 1 clique na mesma janela.
- **Hierarquia Espacial**:
  - **Card Cinematográfico**: Destaque para **Space Odyssey (The Apex Overhaul)** com arte vetorial renderizada em GPU, descrição tática e botões `Jogar Agora` e `Ficha Técnica`.
  - **Painel Lateral de Status**: Especificações de performance nativa (`60+ FPS V-Sync`, `Engine Rust Puro`, `100% Offline`) e atalhos de combate.

---

## 3. Navegação de Categorias & Coleções de Inspiração (`ui/gallery.rs`)
- **Objetivo**: Segmentar o catálogo por gênero de jogo e pelas coleções open-source que inspiraram as recriações em Rust.
- **Categorias & Filtros**:
  - `Todos`
  - `Arcade Clássico` — Snake Arcade, Pong Arcade, Breakout Prism, Pac-Man Classic
  - `Ação & Reflexos` — Space Odyssey, Flappy Jetpack Ultra, Tron Lightcycles
  - `Puzzle & Raciocínio` — Cannon Ballistics, Memory Tiles
  - `Estratégia & Tabuleiro` — Minesweeper Grid, Four in a Row
- **Cards de Coleções no GitHub**:
  - `Grant Jenks Collection` — Clássicos recriados em Rust (Snake, Tron, Cannon, Tiles, Pac-Man)
  - `Al Sweigart Collection` — Clássicos recriados em Rust (Breakout Prism, Four in a Row)
  - `Comunidade Arcade Rust` — Motores expandidos em Rust (Space Odyssey, Pong, Flappy Jetpack Ultra, Minesweeper)

---

## 4. Grid da Galeria de Jogos (`ui/gallery.rs`)
- **Objetivo**: Apresentação visual estruturada em 3 colunas responsivas com feedback imediato de hover.
- **Card do Jogo**:
  - Ícone vetorial estilizado e título em destaque.
  - Tag de categoria em amarelo dourado (`#FACC15`) e crédito de autoria/coleção.
  - Resumo da mecânica principal do jogo.
  - Botões de ação rápida: `Jogar` (inicia imediatamente no palco integrado) e `Ficha` (abre a gaveta lateral de detalhes).

---

## 5. Gaveta Lateral de Detalhes, Controles & Créditos (`ui/drawer.rs`)
- **Objetivo**: Exibir instruções claras antes da partida e dar visibilidade às licenças e repositórios de referência.
- **Conteúdo**:
  - Ficha Técnica: Autor/Coleção, licença aberta (`MIT`, `Apache 2.0`, `Simplified BSD`) e assinatura do motor (`Arquitetura: 100% Rust Puro | Engine: Macroquad 60+ FPS`).
  - Botão de acesso externo para o repositório de referência no GitHub.
  - Descrição da mecânica e quadro detalhado de **Comandos de Controle** (Movimento e Ação/Disparo).
  - Botão principal `Iniciar Jogo no Palco`.

---

## 6. Palco Integrado & Persistência Local (`ui/stage.rs` & `models.rs`)
- **Objetivo**: Executar todos os 11 jogos nativamente dentro da mesma janela com HUD unificado e salvamento automático de recordes.
- **Recursos Técnicos**:
  - Barra superior do console (`ui/stage.rs`) exibindo título do jogo, pontuação em tempo real e controles rápidos (`Voltar`, `Pausar [P]`, `Reiniciar [R]`).
  - Persistência automática da maior pontuação em `scores.json` ao retornar para a galeria.
  - Fundo interativo (`ui/background.rs`) com grade tática de coordenadas, holofote reativo e constelação de partículas com repulsão vetorial ao mouse.
