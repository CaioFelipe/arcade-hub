# Estrutura de Seções & Anatomia de Interface — PyArcade Hub

Este documento define a arquitetura de informação e a composição visual do **PyArcade Hub**, a estação local e offline de jogos desenvolvidos em Python.

---

## 1. Top Bar & Header de Comando
- **Objetivo**: Navegação rápida, alternância de filtros e acesso universal à busca.
- **Elementos**:
  - Logo vetorial da marca (`Arcade Hub`).
  - Barra de busca instantânea com atalho `Ctrl + K` (filtro em tempo real por título ou gênero).
  - Links limpos de navegação (`Explorar`, `Minha Coleção`, `Favoritos`).
  - Seletor de visualização (Grade / Lista).
- **Padrão de UX**: Limpo, sem ruídos de telemetria, sem badges decorativos vazios e sem emojis.

---

## 2. Hero Section: Destaque da Semana & Partida Rápida
- **Objetivo**: Chamar a atenção imediata na primeira dobra (*above the fold*), permitindo abrir o jogo principal com 1 clique.
- **Hierarquia Espacial**:
  - **Centro**: Card cinematográfico de destaque com arte vetorial em SVG animada do jogo em foco.
  - **Flanqueamento Lateral Esquerdo**: Card de atalhos rápidos de teclado/controles recomendados.
  - **Flanqueamento Lateral Direito**: Mini-painel com recorde local pessoal (*High Score*) e estatísticas da última sessão.
  - **Ação Principal**: Botão de alto contraste `Jogar Agora` (aciona subprocesso Python nativo).

---

## 3. Navegação de Categorias & Coleções de Criadores
- **Objetivo**: Segmentar centenas de jogos por estilo de jogo e por coleções oficiais de desenvolvedores da comunidade.
- **Categorias & Coleções Planejadas**:
  - `Todos os Jogos`
  - `Coleção Grant Jenks (Free Python Games)` — Snake, Pacman, Pong, Flappy Bird, Tron, Cannon, Tiles
  - `Coleção Al Sweigart (Invent with Python)` — Tetromino, Slide Puzzle, Memory, Simulações
  - `Coleção Pygame Arcade` — Clássicos da comunidade Pygame com som e física
  - `Arcade Clássico` (Fliperamas e Retrô)
  - `Puzzle & Raciocínio` (Lógica e Tabuleiro)
  - `Favoritos & Recentes`

---

## 4. Grid da Galeria de Jogos (Catálogo Modular)
- **Objetivo**: Apresentação visual limpa dos jogos disponíveis, com posters vetoriais estilizados e créditos explícitos.
- **Card do Jogo**:
  - Capa temática com arte vetorial em SVG.
  - Nome do Jogo e categoria principal.
  - Crédito do autor (ex: `Grant Jenks`, `Al Sweigart`, `Comunidade`).
  - Botão de ação rápida `Jogar`.
  - Acesso à ficha técnica completa.

---

## 5. Modal / Gaveta Lateral de Detalhes, Controles & Créditos
- **Objetivo**: Exibir instruções claras antes de jogar e dar visibilidade total aos criadores do código.
- **Conteúdo**:
  - Bloco de Créditos do Desenvolvedor: Nome do autor, licença de código aberto (MIT, Apache 2.0) e link direto para o repositório no GitHub.
  - Diagrama visual de teclas (ex: `WASD` / `Setas Direcionais` + `Espaço` para atirar/pular).
  - Regras do jogo e pontuação.
  - Tabela de recordes locais salvos no disco (`scores.json`).
  - Botão `Abrir Pasta do Código` (para inspecionar ou modificar o script Python do jogo).

---

## 6. Central de Gerenciamento, Segurança & Adição de Jogos
- **Objetivo**: Facilitar a inclusão de novos jogos feitos pela comunidade ou criados pelo próprio usuário com garantia de execução segura (detalhada em `SECURITY_ARCHITECTURE.md`).
- **Recursos Técnicos de Bastidores**:
  - Detecção automática de novos scripts na pasta `/games/`.
  - Validador de `manifest.json` e verificação de integridade `SHA-256`.
  - Validação estática via AST (`ast_validator.py`) + execução isolada via Python Audit Hooks (`security_runner.py`), bloqueando acesso a rede, terminal e arquivos externos.
  - SDK padrão (`hub_sdk.py`) para persistência segura de recordes e progresso com 1 arquivo `main.py`.
