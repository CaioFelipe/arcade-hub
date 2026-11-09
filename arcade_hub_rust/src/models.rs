use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameMeta {
    pub id: String,
    pub title: String,
    pub author: String,
    pub category: String,
    pub description: String,
    pub controls_move: String,
    pub controls_action: String,
    pub collection: String,
    pub license: String,
    pub repo_url: String,
    pub featured: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoreRecord {
    pub name: String,
    pub score: u32,
    pub date: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct ScoresDB {
    pub scores: HashMap<String, Vec<ScoreRecord>>,
}

impl ScoresDB {
    pub fn load() -> Self {
        if let Ok(data) = fs::read_to_string("scores.json") {
            if let Ok(db) = serde_json::from_str(&data) {
                return db;
            }
        }
        let mut db = ScoresDB::default();
        db.scores.insert(
            "space_odyssey".to_string(),
            vec![
                ScoreRecord { name: "Piloto Safira".into(), score: 14200, date: "Hoje".into() },
                ScoreRecord { name: "Comandante".into(), score: 9800, date: "Ontem".into() },
            ],
        );
        db
    }

    pub fn save(&self) {
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write("scores.json", data);
        }
    }

    pub fn add_score(&mut self, game_id: &str, score: u32) {
        let entry = self.scores.entry(game_id.to_string()).or_default();
        entry.push(ScoreRecord {
            name: "Jogador Local".into(),
            score,
            date: "Hoje".into(),
        });
        entry.sort_by(|a, b| b.score.cmp(&a.score));
        entry.truncate(10);
        self.save();
    }

    pub fn get_high_score(&self, game_id: &str) -> u32 {
        self.scores
            .get(game_id)
            .and_then(|list| list.first().map(|r| r.score))
            .unwrap_or(0)
    }
}

pub fn get_default_catalog() -> Vec<GameMeta> {
    vec![
        GameMeta {
            id: "space_odyssey".into(),
            title: "Space Odyssey".into(),
            author: "Comunidade Pygame / Rust Apex Engine".into(),
            category: "Ação & Reflexos".into(),
            description: "Combate espacial de elite com física 2D, arsenal evolutivo em 5 níveis, ogivas Nova EMP, fragmentação balística, caças invasores e o colossal chefe Dreadnought.".into(),
            controls_move: "WASD / Setas (Movimento 2D) | Shift (Esquiva Dash)".into(),
            controls_action: "Espaço/Clique (Canhões) | X/Clique Dir (Nova EMP)".into(),
            collection: "Comunidade Arcade".into(),
            license: "MIT".into(),
            repo_url: "https://github.com/pygame/pygame".into(),
            featured: true,
        },
        GameMeta {
            id: "snake".into(),
            title: "Snake Cyber Neon".into(),
            author: "Grant Jenks / Free Games".into(),
            category: "Arcade Clássico".into(),
            description: "Evolução cyberpunk da serpente com 5 labirintos, portais quânticos, frutas douradas, poder fantasma e 3 dificuldades.".into(),
            controls_move: "Setas ou W, A, S, D".into(),
            controls_action: "Espaço (Iniciar) | 1, 2, 3 (Dificuldade)".into(),
            collection: "Grant Jenks Collection".into(),
            license: "Apache 2.0".into(),
            repo_url: "https://github.com/grantjenks/free-python-games".into(),
            featured: false,
        },
        GameMeta {
            id: "pong".into(),
            title: "Pong Arcade".into(),
            author: "Comunidade Pygame".into(),
            category: "Arcade Clássico".into(),
            description: "Duelo de tênis de mesa clássico contra inteligência artificial responsiva e física de rebote.".into(),
            controls_move: "Setas Cima / Baixo ou W / S".into(),
            controls_action: "Ângulo de interceptação da raquete".into(),
            collection: "Comunidade Arcade".into(),
            license: "MIT".into(),
            repo_url: "https://github.com/pygame/pygame".into(),
            featured: false,
        },
        GameMeta {
            id: "breakout".into(),
            title: "Breakout Prism".into(),
            author: "Al Sweigart / Invent Games".into(),
            category: "Arcade Clássico".into(),
            description: "Campanha em 5 fases prismáticas com 3 dificuldades, blocos de titânio, bombas em cadeia, canhões laser e multi-bolas.".into(),
            controls_move: "Setas / A, D ou Mouse".into(),
            controls_action: "Espaço/Clique (Lançar / Laser) | 1, 2, 3 (Dificuldade)".into(),
            collection: "Al Sweigart Collection".into(),
            license: "Simplified BSD".into(),
            repo_url: "https://github.com/asweigart/inventwithpython".into(),
            featured: false,
        },
        GameMeta {
            id: "flappy".into(),
            title: "Flappy Jetpack Ultra".into(),
            author: "Comunidade Arcade".into(),
            category: "Ação & Reflexos".into(),
            description: "Acrobacias aéreas por 4 biomas futuristas com parallax, moedas, escudos de força, turbo boost e canos oscilantes.".into(),
            controls_move: "Barra de Espaço, Seta Cima ou Clique".into(),
            controls_action: "Propulsão a jato | 1, 2, 3 (Dificuldade)".into(),
            collection: "Comunidade Arcade".into(),
            license: "MIT".into(),
            repo_url: "https://github.com/pygame/pygame".into(),
            featured: false,
        },
        GameMeta {
            id: "tron".into(),
            title: "Tron Cyber Cycles Neo".into(),
            author: "Grant Jenks / Free Games".into(),
            category: "Ação & Reflexos".into(),
            description: "Torneio Melhor de 5 na grade de luz com Turbo Boost, pulso EMP, salto quântico e IA tática em 4 arenas.".into(),
            controls_move: "Setas ou W, A, S, D".into(),
            controls_action: "Espaço (Turbo) | E (EMP) | J (Salto) | 1, 2, 3 (Dificuldade)".into(),
            collection: "Grant Jenks Collection".into(),
            license: "Apache 2.0".into(),
            repo_url: "https://github.com/grantjenks/free-python-games".into(),
            featured: false,
        },
        GameMeta {
            id: "cannon".into(),
            title: "Cannon Ballistics".into(),
            author: "Grant Jenks / Free Games".into(),
            category: "Puzzle & Raciocínio".into(),
            description: "Artilharia balística com cálculo parabólico contra alvos voadores dinâmicos.".into(),
            controls_move: "Setas Cima / Baixo (Ângulo e Força)".into(),
            controls_action: "Espaço ou Clique para Disparo".into(),
            collection: "Grant Jenks Collection".into(),
            license: "Apache 2.0".into(),
            repo_url: "https://github.com/grantjenks/free-python-games".into(),
            featured: false,
        },
        GameMeta {
            id: "tiles".into(),
            title: "Memory Tiles".into(),
            author: "Grant Jenks / Free Games".into(),
            category: "Puzzle & Raciocínio".into(),
            description: "Quebra-cabeça de 15 números deslizantes para ordenação sequencial no menor número de passos.".into(),
            controls_move: "Setas ou Clique nas peças adjacentes".into(),
            controls_action: "Deslize sequencial para a lacuna".into(),
            collection: "Grant Jenks Collection".into(),
            license: "Apache 2.0".into(),
            repo_url: "https://github.com/grantjenks/free-python-games".into(),
            featured: false,
        },
        GameMeta {
            id: "minesweeper".into(),
            title: "Minesweeper Grid".into(),
            author: "Comunidade Arcade".into(),
            category: "Estratégia & Tabuleiro".into(),
            description: "Campo minado estratégico 10x10 com primeiro clique seguro, expansão em cascata e bandeiras.".into(),
            controls_move: "Mouse para posicionar cursor".into(),
            controls_action: "Clique Esquerdo Revela, Direito Bandeira".into(),
            collection: "Comunidade Arcade".into(),
            license: "MIT".into(),
            repo_url: "https://github.com/pygame/pygame".into(),
            featured: false,
        },
        GameMeta {
            id: "connect4".into(),
            title: "Four in a Row".into(),
            author: "Al Sweigart / Invent Games".into(),
            category: "Estratégia & Tabuleiro".into(),
            description: "Conecte quatro fichas da mesma cor na vertical, horizontal ou diagonal antes do rival da IA.".into(),
            controls_move: "Setas Esquerda / Direita ou Teclas 1 a 7".into(),
            controls_action: "Espaço ou Clique para Soltar Ficha".into(),
            collection: "Al Sweigart Collection".into(),
            license: "Simplified BSD".into(),
            repo_url: "https://github.com/asweigart/inventwithpython".into(),
            featured: false,
        },
        GameMeta {
            id: "pacman".into(),
            title: "Pac-Man Classic".into(),
            author: "Grant Jenks / Free Games".into(),
            category: "Arcade Clássico".into(),
            description: "Navegação por labirinto retrô consumindo pílulas e fugindo dos fantasmas coloridos.".into(),
            controls_move: "Setas Direcionais ou W, A, S, D".into(),
            controls_action: "Mudança suave nos corredores".into(),
            collection: "Grant Jenks Collection".into(),
            license: "Apache 2.0".into(),
            repo_url: "https://github.com/grantjenks/free-python-games".into(),
            featured: false,
        },
    ]
}
