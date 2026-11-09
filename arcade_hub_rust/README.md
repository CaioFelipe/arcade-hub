# Arcade Hub — Rust Native Engine (`arcade_hub_rust`)

Motor desktop nativo do **Arcade Hub**, construído 100% em **Rust Puro** (`Edition 2024`) com **Macroquad**, sem interpretadores externos, sem Chromium/WebView, sem servidor HTTP e sem portas de rede.

---

## ⚡ Destaques da Arquitetura em Rust

| Critério | Especificação Técnica (`arcade_hub_rust`) |
| :--- | :--- |
| **Arquitetura** | **Binário Nativo em Rust com Aceleração por Hardware (OpenGL / GPU)** |
| **Rede / Portas** | **Zero portas de rede**, 100% offline e isolado |
| **Processos** | **1 único processo nativo** no sistema operacional |
| **Tamanho** | **~6.0 MB** (executável único com fontes e 11 jogos embutidos) |
| **Inicialização** | **Instantânea (< 50 milissegundos)** |
| **Desempenho** | **60+ FPS fluidos (V-Sync)** com física direta e resposta imediata |
| **Palco Integrado** | **Renderização nativa dentro da mesma janela** da galeria |

---

## 🎮 Jogos Inclusos (100% Rust no Palco Integrado)

1. **Space Odyssey (The Apex Overhaul)**: Combate espacial com inércia vetorial, esquiva *Dash*, 5 níveis de armas evolutivas, super-arma *Quantum Nova EMP*, *Graze*, asteroides fragmentáveis, frota invasora e chefe *Flagship Dreadnought*.
2. **Breakout Prism**: Campanha de 5 fases prismáticas com 3 modos de dificuldade, blocos de titânio, bombas em cadeia, canhões laser e multi-bolas.
3. **Flappy Jetpack Ultra**: Acrobacias aéreas por 4 biomas futuristas em *parallax*, moedas, escudos de força, *turbo boost*, canos oscilantes e 3 dificuldades.
4. **Snake Cyber Neon**: Evolução cyberpunk da serpente com 5 labirintos dinâmicos, portais quânticos, frutas especiais, poder fantasma e 3 dificuldades.
5. **Tron Cyber Cycles Neo**: Torneio Melhor de 5 na grade de luz com *Turbo Boost*, pulso EMP, salto quântico, portais e IA tática em 4 arenas.
6. **Pong Arcade**: Duelo de tênis de mesa clássico contra inteligência artificial responsiva e física de rebote por ângulo.
7. **Cannon Ballistics**: Artilharia com ajuste de ângulo e potência parabólica contra alvos voadores dinâmicos.
8. **Memory Tiles**: Quebra-cabeça de 15 números deslizantes gerado por embaralhamento matematicamente resolvível.
9. **Minesweeper Grid**: Campo minado 10x10 com primeiro clique 100% seguro, expansão em cascata e marcação de bandeiras.
10. **Four in a Row**: Conecte quatro fichas da mesma cor na vertical, horizontal ou diagonal contra a IA.
11. **Pac-Man Classic**: Labirinto retrô com pílulas, animação de *chomp* e fantasmas coloridos com IA de perseguição.

---

## 🚀 Como Executar e Compilar

Basta executar o binário compilado:
```powershell
.\ArcadeHub.exe
```

Para compilar a partir do código-fonte:
```bash
cd arcade_hub_rust
cargo build --release
```
O executável otimizado será gerado em `target/release/ArcadeHub.exe`.

---

## 🎨 Design & Interface

- **Paleta Oficial**: Safira Escuro (`#090C14`), Slate Naval (`#101522`), Azul Cobalto Gamer (`#2563EB`), Azul Gelo (`#60A5FA`), Amarelo Dourado (`#FACC15`) e Coral (`#F43F5E`).
- **Fundo Reativo**: Grade de coordenadas táticas com holofote dinâmico e constelação de partículas que reagem ao cursor em tempo real.
- **Ícones Vetoriais**: Desenhados via primitivas geométricas em GPU (Play, Voltar, Pausar, Reiniciar, GitHub, Busca).
- **Controles Rápidos**:
  - `ESC`: Retorna à galeria ou fecha gavetas/modais a qualquer momento.
  - `P`: Pausar / Retomar jogo ativo.
  - `R`: Reiniciar partida instantaneamente.
  - `Ctrl + K`: Foco rápido no campo de busca.
  - `+ Adicionar Jogo`: Cadastro de novos jogos no catálogo local.
