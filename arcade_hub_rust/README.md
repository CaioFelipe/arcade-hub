# Arcade Hub — Rust Puro (Native Desktop Engine)

Galeria e launcher de jogos arcade offline 100% em **Rust Puro**, sem Chromium, sem WebView, sem servidor HTTP, sem portas de rede e sem dependências externas.

---

## ⚡ Por que Rust Puro?

| Critério | Versão Anterior (Python + Web + HTTP) | Nova Versão (Rust Puro - `arcade_hub_rust`) |
| :--- | :--- | :--- |
| **Arquitetura** | Servidor HTTP em localhost + Edge/Chromium | **Binário Nativo com Aceleração por Hardware** |
| **Rede / Portas** | Ocupava porta TCP `8090` (conflitos, firewall, `ERR_CONNECTION_REFUSED`) | **Zero portas de rede**, 100% offline nativo |
| **Processos** | 3 a 5 subprocessos (Python, msedge, background daemon) | **1 único processo nativo** |
| **Tamanho** | ~17 MB + perfil de navegador de dezenas de MBs | **Apenas 4.1 MB** (executável único) |
| **Inicialização** | ~3 a 5 segundos (com risco de timeout) | **Instantânea (< 50 milissegundos)** |
| **Desempenho** | Streaming de frames base64 via HTTP (~30 FPS) | **60+ FPS fluidos** com física direta e resposta imediata |
| **Mesma Janela** | Modal web com canvas emulado | **Renderização nativa dentro da mesma janela** |

---

## 🎮 Jogos Inclusos (Na Mesma Janela)

1. **Space Odyssey**: Combate espacial com propulsão a jato, disparos laser duplos, chuva de asteroides e explosões de partículas.
2. **Snake Arcade**: Serpente ágil com grade de coordenadas, maçãs nutritivas e aumento dinâmico de velocidade.
3. **Pong Arcade**: Duelo de tênis de mesa clássico contra inteligência artificial responsiva e física de rebote por ângulo.
4. **Breakout Prism**: Demolição de tijolos prismáticos coloridos com rebote acelerado e contagem de vidas.
5. **Flappy Jetpack**: Impulso vertical rítmico entre colunas de obstáculos com física gravitacional suave.
6. **Tron Lightcycles**: Duelo de motos de luz na arena neon contra IA rival (evite cruzar rastros).
7. **Cannon Ballistics**: Artilharia com ajuste de ângulo e potência parabólica contra alvos voadores dinâmicos.
8. **Memory Tiles**: Quebra-cabeça de 15 números deslizantes gerado por embaralhamento matematicamente resolvível.
9. **Minesweeper Grid**: Campo minado 10x10 com primeiro clique 100% seguro, expansão em cascata e marcação de bandeiras.
10. **Four in a Row**: Conecte quatro fichas da mesma cor na vertical, horizontal ou diagonal contra a IA.
11. **Pac-Man Classic**: Labirinto retrô com pílulas, animação de chomp e fantasmas coloridos com IA de perseguição.

---

## 🚀 Como Executar

Basta dar dois cliques no executável:
```
arcade_hub_rust/ArcadeHub.exe
```

Para compilar novamente caso queira fazer modificações:
```bash
cd arcade_hub_rust
cargo build --release
```
O executável otimizado será gerado em `target/release/ArcadeHub.exe`.

---

## 🎨 Design & Interface

- **Paleta Oficial**: Safira Escuro (`#090C14`), Slate Naval (`#101522`), Azul Cobalto Gamer (`#2563EB`), Azul Gelo (`#60A5FA`), Amarelo Dourado (`#FACC15`) e Coral (`#F43F5E`).
- **Zero AI Slop / Zero Emojis**: Ícones vetoriais desenhados por código geométrico puro (Play, Voltar, Pausar, Reiniciar, Plus).
- **Controles Rápidos**:
  - `ESC`: Retorna à galeria ou fecha gavetas a qualquer momento.
  - `P`: Pausar / Retomar jogo ativo.
  - `R`: Reiniciar partida instantaneamente.
  - `Ctrl + K`: Foco rápido no campo de busca.
  - `+ Adicionar Jogo`: Cadastro e personalização de novos jogos diretamente no Hub.
