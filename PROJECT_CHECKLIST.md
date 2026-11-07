# Checklist de Acompanhamento do Projeto — Arcade Hub (Galeria Local de Jogos Python)

## Fase 1: Diagnóstico & Briefing do Produto
- [x] Entidade do Produto definida (Launcher & Galeria Desktop Offline para Execução de Jogos Python na Mesma Janela).
- [x] Público-alvo e Proposta de Valor central (Acesso sem fricção a centenas de jogos locais em Python com estatísticas, capas e palco integrado).
- [x] Purificação total de telemetria artificial e AI Slop em `SECTIONS.md` e `app/index.html`.
- [x] Paleta de Cores Oficial estabelecida (Safira Escuro `#090C14` + Slate Naval `#101522` + Azul Cobalto Gamer `#2563EB` + Pontos Estelares no grid).
- [x] Seções Especiais de Repositórios Públicos integradas com créditos explícitos, licença e links no GitHub.

## Fase 2: Curadoria de Referências & DNA Estético
- [x] DNA Estético alinhado: Geometria e linhas nítidas de 1px da Opção 2 com paleta Safira Escuro e fundo de coordenadas com pontos de cor.
- [x] Sistema Tipográfico selecionado: Plus Jakarta Sans (Display) + Inter (Interface) + Fira Code (Monospace).
- [x] Efeitos e materiais: Cantos estruturados de 6-8px, micro-grid industrial com pontos de cor, botões táteis sem halos de neon.

## Fase 3: Planejamento de Seções (`SECTIONS.md`)
- [x] Estrutura de seções aprovada e purificada em `SECTIONS.md`.
- [x] Seções de Coleções Comunitárias e Ficha Técnica com Créditos de Autores adicionadas.
- [x] Wireframe da primeira dobra (Header + Hero flanqueado + Cards de comandos e placar) validado.

## Fase 4: Infraestrutura, Executável Único (`ArcadeHub.exe`) & Palco na Mesma Janela
- [x] Compilação de executável único (`ArcadeHub.exe`) na raiz do projeto sem necessidade de arquivos `.bat` ou `.pyw` extras.
- [x] Ponto de entrada nativo (`app/main_desktop.py`) abrindo janela dedicada em modo App sem janelas pretas de terminal.
- [x] Motor gráfico em memória (`app/game_engine.py` — `InWindowGameRunner` com `SDL_VIDEODRIVER="dummy"`) renderizando todos os 11 jogos e jogos comunitários diretamente no `<canvas id="arcade-canvas">` dentro da mesma janela do Hub.
- [x] Catálogo de jogos configurado em `games/` com manifestos individuais (`manifest.json`), hashes SHA-256 e registro central (`catalog.json`).

## Fase 5: Header + Hero Section (Exclusivo)
- [x] Header com logo vetorial, busca universal (`Ctrl+K`), botão `+ Adicionar Jogo` e navegação limpa.
- [x] Hero dentro da primeira dobra com jogo em destaque (`Space Odyssey`), nave espacial modular em SVG e botão de execução direta `Jogar Agora`.
- [x] Flanqueamento lateral completo: Card de Comandos Rápidos à esquerda e Placar Local de Recordes à direita.

## Fase 6: Verificação Visual Real (Chrome DevTools MCP)
- [x] Screenshots capturados e auditados criticamente em alta resolução:
  - Dobra Superior (Header + Hero): `screenshot_app_hero.png`
  - Página Completa (Grid + Coleções): `screenshot_app_full.png`
  - Gaveta de Detalhes & Créditos: `screenshot_app_drawer.png`
  - Execução de Jogos na Mesma Janela (`Space Odyssey` / `Snake Arcade`): `screenshot_inwindow_gameplay.png`
  - Estúdio Padronizado de Criação e Validação AST (`+ Adicionar Jogo`): `screenshot_add_game_studio.png`
- [x] Contraste, alinhamentos e ausência de distorções visuais verificados e validados.

## Fase 7: Cascata Modular & Estúdio Padronizado de Adição de Jogos
- [x] Seção de Coleções em Destaque (Grant Jenks, Al Sweigart, Comunidade Pygame) com biografia, licença e links diretos para o GitHub.
- [x] Grade de jogos com filtros rápidos por categoria (Todos, Arcade, Ação, Puzzle, Estratégia) e capas vetoriais ou customizadas.
- [x] Modal padronizado **"+ Adicionar Jogo"** (`#add-game-modal`) com Nome, Autor, Categoria, Descrição, Upload de Imagem de Capa, Código Python (`main.py` com template `class Game` e importação de `.py`) e Validação de Segurança AST em tempo real.
- [x] Auto-descoberta automática de novas pastas adicionadas em `games/<id>/`.

## Fase 8: Arquitetura de Segurança & Sandbox Comunitária (`SECURITY_ARCHITECTURE.md`)
- [x] Especificação da arquitetura de Defesa em Profundidade documentada em `SECURITY_ARCHITECTURE.md`.
- [x] Camada 1 — Scanner Estático de AST (`app/ast_validator.py`): Allowlist de módulos seguros e bloqueio de imports de sistema/rede (`os`, `sys`, `subprocess`, `socket`, `eval`, `exec`, `open`, `__import__`).
- [x] Camada 2 — Guardião de Runtime (`app/security_runner.py`): Python Audit Hooks (`sys.addaudithook` / PEP 578) e namespace restrito `safe_builtins` bloqueando rede, terminal e DLLs.
- [x] Camada 3 — SDK Oficial de Jogos (`app/hub_sdk.py`): API controlada para salvar recordes e estado do jogo (JSON sanitizado até 64 KB) sem acesso direto ao disco.
- [x] Camada 4 — Verificação de Hash (`SHA-256`) em todos os 11 jogos do catálogo e em novos jogos criados no Hub.

## Fase 9: Evolução Contínua dos Jogos — O Top dos Top da Modalidade
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
  - [x] Paridade e sincronização total entre o motor nativo em Rust (`ArcadeHub.exe`) e o script Python em `games/space_odyssey/main.py` sob a Sandbox AST.

## Fase 10: Atmosfera & Interatividade Reativa de Fundo (Rust Nativo)
- [x] Motor de Fundo Dinâmico (`arcade_hub_rust/src/ui/background.rs`):
  - [x] Grade de coordenadas táticas com spotlight reativo que ilumina eixos e desenha micro-cruzes de miramento na posição do cursor.
  - [x] Constelação de 52 nós estelares com física de repulsão vetorial ao aproximar o mouse.
  - [x] Feixes translúcidos de conexão entre estrelas próximas e traçado de mira direta até o ponteiro do mouse.
  - [x] Executável nativo compilado em modo `--release` (`ArcadeHub.exe`) com 60+ FPS cravados em GPU.

