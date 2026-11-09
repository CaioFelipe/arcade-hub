use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

// =========================================================================
// 1. CONSTANTES DO LABIRINTO E ENUMS
// =========================================================================

pub const COLS: usize = 21;
pub const ROWS: usize = 21;
pub const TILE_SIZE: f32 = 24.0;

// Mapa 21x21 Autêntico Estilo Neo-Arcade
// 1: Parede, 0: Pílula, 2: Energizer (Pílula Grande), 3: Vazio/Corredor, 4: Túnel Lateral, 5: Portão Fantasma
const MAZE_MAP: [[u8; COLS]; ROWS] = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1],
    [1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1],
    [1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1],
    [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 0, 1, 1, 1, 3, 0, 1, 0, 3, 1, 1, 1, 0, 1, 1, 1, 1],
    [3, 3, 3, 1, 0, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 0, 1, 3, 3, 3],
    [1, 1, 1, 1, 0, 1, 3, 1, 1, 5, 5, 5, 1, 1, 3, 1, 0, 1, 1, 1, 1],
    [4, 0, 0, 0, 0, 3, 3, 1, 3, 3, 3, 3, 3, 1, 3, 3, 0, 0, 0, 0, 4],
    [1, 1, 1, 1, 0, 1, 3, 1, 1, 1, 1, 1, 1, 1, 3, 1, 0, 1, 1, 1, 1],
    [3, 3, 3, 1, 0, 1, 3, 3, 3, 3, 3, 3, 3, 3, 3, 1, 0, 1, 3, 3, 3],
    [1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1],
    [1, 2, 0, 1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 1, 0, 2, 1],
    [1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1],
    [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Difficulty {
    Casual,
    Normal,
    Hardcore,
}

impl Difficulty {
    pub fn name(self) -> &'static str {
        match self {
            Difficulty::Casual => "CASUAL",
            Difficulty::Normal => "NORMAL",
            Difficulty::Hardcore => "HARDCORE",
        }
    }

    pub fn pacman_speed(self) -> f32 {
        match self {
            Difficulty::Casual => 2.5,
            Difficulty::Normal => 2.8,
            Difficulty::Hardcore => 3.2,
        }
    }

    pub fn ghost_speed(self) -> f32 {
        match self {
            Difficulty::Casual => 1.8,
            Difficulty::Normal => 2.2,
            Difficulty::Hardcore => 2.8,
        }
    }

    pub fn frightened_time(self) -> f32 {
        match self {
            Difficulty::Casual => 9.0,
            Difficulty::Normal => 6.5,
            Difficulty::Hardcore => 3.5,
        }
    }

    pub fn starting_lives(self) -> i32 {
        match self {
            Difficulty::Casual => 4,
            Difficulty::Normal => 3,
            Difficulty::Hardcore => 3,
        }
    }

    pub fn score_multiplier(self) -> f32 {
        match self {
            Difficulty::Casual => 1.0,
            Difficulty::Normal => 1.5,
            Difficulty::Hardcore => 2.5,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_offset(self) -> (i32, i32) {
        match self {
            Direction::None => (0, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn is_opposite(self, other: Direction) -> bool {
        match (self, other) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => true,
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => true,
            _ => false,
        }
    }

    pub fn to_angle(self) -> f32 {
        match self {
            Direction::Right => 0.0,
            Direction::Down => std::f32::consts::FRAC_PI_2,
            Direction::Left => std::f32::consts::PI,
            Direction::Up => std::f32::consts::PI * 1.5,
            Direction::None => 0.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GhostType {
    Blinky, // Shadow (Vermelho) - Perseguidor direto
    Pinky,  // Speedy (Rosa) - Emboscador 4 passos à frente
    Inky,   // Bashful (Ciano) - Flanqueador com Blinky
    Clyde,  // Pokey (Laranja) - Covarde / Patrulheiro
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GhostState {
    InHouse,
    ExitingHouse,
    Chase,
    Scatter,
    Frightened,
    EatenEyes,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FruitKind {
    Cherry,     // Cereja (+100)
    Strawberry, // Morango (+300)
    Orange,     // Laranja (+500)
    Apple,      // Maçã (+700)
    Key,        // Chave Cyber (+5000)
}

impl FruitKind {
    pub fn for_stage(stage: u32) -> Self {
        match stage {
            1 => FruitKind::Cherry,
            2 => FruitKind::Strawberry,
            3 => FruitKind::Orange,
            4 => FruitKind::Apple,
            _ => FruitKind::Key,
        }
    }

    pub fn points(self) -> u32 {
        match self {
            FruitKind::Cherry => 100,
            FruitKind::Strawberry => 300,
            FruitKind::Orange => 500,
            FruitKind::Apple => 700,
            FruitKind::Key => 5000,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            FruitKind::Cherry => "CEREJA",
            FruitKind::Strawberry => "MORANGO",
            FruitKind::Orange => "LARANJA",
            FruitKind::Apple => "MAÇÃ",
            FruitKind::Key => "CHAVE CYBER",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GamePhase {
    ReadyCountdown,
    Playing,
    PacmanDying,
    StageClear,
    GameOver,
}

// =========================================================================
// 2. ESTRUTURAS AUXILIARES (PARTÍCULAS, POPUPS, GHOST)
// =========================================================================

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
    radius: f32,
    life: f32,
    max_life: f32,
}

struct ScorePopup {
    x: f32,
    y: f32,
    text: String,
    color: Color,
    life: f32,
}

struct Ghost {
    ghost_type: GhostType,
    x: f32,
    y: f32,
    dir: Direction,
    state: GhostState,
    color: Color,
    scatter_target: (i32, i32),
    house_timer: f32,
}

// =========================================================================
// 3. JOGO PRINCIPAL
// =========================================================================

pub struct PacmanGame {
    difficulty: Difficulty,
    phase: GamePhase,
    phase_timer: f32,

    // Pacman
    pac_x: f32,
    pac_y: f32,
    dir: Direction,
    next_dir: Direction,
    chomp_angle: f32,

    // Labirinto e Itens
    maze: [[u8; COLS]; ROWS],
    pellets_total: usize,
    pellets_left: usize,
    pellets_eaten: usize,

    // Fantasmas
    ghosts: Vec<Ghost>,
    frightened_timer: f32,
    ghost_multiplier: u32,
    wave_timer: f32,
    is_wave_chase: bool,

    // Fruta Bônus
    active_fruit: Option<FruitKind>,
    fruit_timer: f32,
    fruit_pos: (f32, f32),
    fruit_spawned_count: u32,
    collected_fruits: Vec<FruitKind>,

    // Estatísticas e Estado
    score: u32,
    high_score: u32,
    lives: i32,
    stage: u32,
    ghosts_eaten_total: u32,

    // Efeitos Visuais
    particles: Vec<Particle>,
    popups: Vec<ScorePopup>,
    wall_flash_timer: f32,
}

impl PacmanGame {
    pub fn new() -> Self {
        let mut game = Self {
            difficulty: Difficulty::Normal,
            phase: GamePhase::ReadyCountdown,
            phase_timer: 1.8,

            pac_x: Self::tile_cx(10),
            pac_y: Self::tile_cy(16),
            dir: Direction::None,
            next_dir: Direction::None,
            chomp_angle: 0.0,

            maze: MAZE_MAP,
            pellets_total: 0,
            pellets_left: 0,
            pellets_eaten: 0,

            ghosts: Vec::new(),
            frightened_timer: 0.0,
            ghost_multiplier: 1,
            wave_timer: 7.0, // Começa com 7s de dispersão clássica
            is_wave_chase: false,

            active_fruit: None,
            fruit_timer: 0.0,
            fruit_pos: (Self::tile_cx(10), Self::tile_cy(12)),
            fruit_spawned_count: 0,
            collected_fruits: Vec::new(),

            score: 0,
            high_score: 12500,
            lives: 3,
            stage: 1,
            ghosts_eaten_total: 0,

            particles: Vec::new(),
            popups: Vec::new(),
            wall_flash_timer: 0.0,
        };

        game.setup_stage(1, true);
        game
    }

    #[inline]
    fn tile_cx(col: i32) -> f32 {
        (col as f32 + 0.5) * TILE_SIZE
    }

    #[inline]
    fn tile_cy(row: i32) -> f32 {
        (row as f32 + 0.5) * TILE_SIZE
    }

    pub fn set_difficulty(&mut self, diff: Difficulty) {
        if self.difficulty != diff {
            self.difficulty = diff;
            self.lives = diff.starting_lives();
            self.popups.push(ScorePopup {
                x: Self::tile_cx(10),
                y: Self::tile_cy(10),
                text: format!("MODO: {}", diff.name()),
                color: ICE_BLUE,
                life: 1.5,
            });
        }
    }

    fn setup_stage(&mut self, stage: u32, reset_score_and_lives: bool) {
        self.stage = stage;
        self.maze = MAZE_MAP;

        let mut pellets = 0;
        for r in 0..ROWS {
            for c in 0..COLS {
                if self.maze[r][c] == 0 || self.maze[r][c] == 2 {
                    pellets += 1;
                }
            }
        }
        self.pellets_total = pellets;
        self.pellets_left = pellets;
        self.pellets_eaten = 0;
        self.active_fruit = None;
        self.fruit_timer = 0.0;
        self.fruit_spawned_count = 0;

        if reset_score_and_lives {
            self.score = 0;
            self.lives = self.difficulty.starting_lives();
            self.collected_fruits.clear();
            self.ghosts_eaten_total = 0;
        }

        self.reset_positions_for_round();
    }

    fn reset_positions_for_round(&mut self) {
        self.pac_x = Self::tile_cx(10);
        self.pac_y = Self::tile_cy(16);
        self.dir = Direction::None;
        self.next_dir = Direction::None;
        self.chomp_angle = 0.0;

        self.frightened_timer = 0.0;
        self.ghost_multiplier = 1;
        self.wave_timer = 7.0;
        self.is_wave_chase = false;

        self.ghosts = vec![
            // Blinky (Vermelho) - Começa no corredor acima da casa
            Ghost {
                ghost_type: GhostType::Blinky,
                x: Self::tile_cx(10),
                y: Self::tile_cy(8),
                dir: Direction::Left,
                state: GhostState::Scatter,
                color: Color::new(0.96, 0.22, 0.22, 1.0),
                scatter_target: (COLS as i32 - 2, 0),
                house_timer: 0.0,
            },
            // Pinky (Rosa) - Dentro da casa, sai após 1s
            Ghost {
                ghost_type: GhostType::Pinky,
                x: Self::tile_cx(10),
                y: Self::tile_cy(10),
                dir: Direction::Up,
                state: GhostState::InHouse,
                color: Color::new(0.98, 0.60, 0.78, 1.0),
                scatter_target: (1, 0),
                house_timer: 1.2,
            },
            // Inky (Ciano) - Dentro da casa, sai após 4s
            Ghost {
                ghost_type: GhostType::Inky,
                x: Self::tile_cx(9),
                y: Self::tile_cy(10),
                dir: Direction::Up,
                state: GhostState::InHouse,
                color: Color::new(0.0, 0.90, 0.95, 1.0),
                scatter_target: (COLS as i32 - 1, ROWS as i32 - 1),
                house_timer: 4.0,
            },
            // Clyde (Laranja) - Dentro da casa, sai após 7s
            Ghost {
                ghost_type: GhostType::Clyde,
                x: Self::tile_cx(11),
                y: Self::tile_cy(10),
                dir: Direction::Up,
                state: GhostState::InHouse,
                color: Color::new(0.98, 0.62, 0.12, 1.0),
                scatter_target: (0, ROWS as i32 - 1),
                house_timer: 7.0,
            },
        ];

        self.phase = GamePhase::ReadyCountdown;
        self.phase_timer = 1.8;
    }

    fn is_pacman_walkable(&self, col: i32, row: i32) -> bool {
        // Túnel horizontal lateral
        if row == 10 && (col < 0 || col >= COLS as i32) {
            return true;
        }
        if col < 0 || col >= COLS as i32 || row < 0 || row >= ROWS as i32 {
            return false;
        }
        let cell = self.maze[row as usize][col as usize];
        // 1: Parede, 5: Portão da casa dos fantasmas (proibido para Pacman)
        cell != 1 && cell != 5
    }

    fn is_ghost_walkable(maze: &[[u8; COLS]; ROWS], col: i32, row: i32, is_eyes: bool) -> bool {
        // Túnel horizontal lateral
        if row == 10 && (col < 0 || col >= COLS as i32) {
            return true;
        }
        if col < 0 || col >= COLS as i32 || row < 0 || row >= ROWS as i32 {
            return false;
        }
        let cell = maze[row as usize][col as usize];
        if is_eyes {
            // Olhos atravessam portão para renascer
            cell != 1
        } else {
            // Fantasmas normais não entram no portão livremente a menos que em saída
            cell != 1 && cell != 5
        }
    }

    // =========================================================================
    // 4. ATUALIZAÇÃO DA FÍSICA E MOVIMENTAÇÃO (CORREÇÃO DE BUGS)
    // =========================================================================

    fn update_pacman_movement(&mut self, dt: f32) {
        let speed = self.difficulty.pacman_speed();
        let move_dist = speed * (dt * 60.0).clamp(0.5, 2.0);

        let col = (self.pac_x / TILE_SIZE).floor() as i32;
        let row = (self.pac_y / TILE_SIZE).floor() as i32;
        let cx = Self::tile_cx(col);
        let cy = Self::tile_cy(row);

        // 1. Meia-volta imediata (180 graus) em qualquer ponto do corredor
        if self.next_dir.is_opposite(self.dir) {
            self.dir = self.next_dir;
        }

        // 2. Curvas perpendiculares com Cornering Assist (janela de 9px)
        if self.next_dir != Direction::None && self.next_dir != self.dir {
            match self.dir {
                Direction::Left | Direction::Right => {
                    let dist = (self.pac_x - cx).abs();
                    if dist <= 9.0 {
                        let (_, ndy) = self.next_dir.to_offset();
                        if self.is_pacman_walkable(col, row + ndy) {
                            self.pac_x = cx; // Alinha ao eixo vertical
                            self.dir = self.next_dir;
                        }
                    }
                }
                Direction::Up | Direction::Down => {
                    let dist = (self.pac_y - cy).abs();
                    if dist <= 9.0 {
                        let (ndx, _) = self.next_dir.to_offset();
                        if self.is_pacman_walkable(col + ndx, row) {
                            self.pac_y = cy; // Alinha ao eixo horizontal
                            self.dir = self.next_dir;
                        }
                    }
                }
                Direction::None => {
                    let (ndx, ndy) = self.next_dir.to_offset();
                    if self.is_pacman_walkable(col + ndx, row + ndy) {
                        self.dir = self.next_dir;
                    }
                }
            }
        }

        // 3. Avanço contínuo no sentido atual com parada suave na parede
        match self.dir {
            Direction::Right => {
                self.pac_y = cy;
                if !self.is_pacman_walkable(col + 1, row) {
                    if self.pac_x < cx {
                        self.pac_x = (self.pac_x + move_dist).min(cx);
                    } else {
                        self.pac_x = cx;
                    }
                } else {
                    self.pac_x += move_dist;
                    self.chomp_angle += 0.32;
                }
            }
            Direction::Left => {
                self.pac_y = cy;
                if !self.is_pacman_walkable(col - 1, row) {
                    if self.pac_x > cx {
                        self.pac_x = (self.pac_x - move_dist).max(cx);
                    } else {
                        self.pac_x = cx;
                    }
                } else {
                    self.pac_x -= move_dist;
                    self.chomp_angle += 0.32;
                }
            }
            Direction::Down => {
                self.pac_x = cx;
                if !self.is_pacman_walkable(col, row + 1) {
                    if self.pac_y < cy {
                        self.pac_y = (self.pac_y + move_dist).min(cy);
                    } else {
                        self.pac_y = cy;
                    }
                } else {
                    self.pac_y += move_dist;
                    self.chomp_angle += 0.32;
                }
            }
            Direction::Up => {
                self.pac_x = cx;
                if !self.is_pacman_walkable(col, row - 1) {
                    if self.pac_y > cy {
                        self.pac_y = (self.pac_y - move_dist).max(cy);
                    } else {
                        self.pac_y = cy;
                    }
                } else {
                    self.pac_y -= move_dist;
                    self.chomp_angle += 0.32;
                }
            }
            Direction::None => {}
        }

        // 4. Túnel lateral sem limites (Linha 10)
        let total_w = COLS as f32 * TILE_SIZE;
        if self.pac_x < -TILE_SIZE * 0.5 {
            self.pac_x += total_w + TILE_SIZE;
        } else if self.pac_x > total_w + TILE_SIZE * 0.5 {
            self.pac_x -= total_w + TILE_SIZE;
        }

        // 5. Coleta de Pílulas e Energizers
        let cur_c = (self.pac_x / TILE_SIZE).floor() as i32;
        let cur_r = (self.pac_y / TILE_SIZE).floor() as i32;
        let c_cx = Self::tile_cx(cur_c);
        let c_cy = Self::tile_cy(cur_r);

        if (self.pac_x - c_cx).abs() < 10.0 && (self.pac_y - c_cy).abs() < 10.0 {
            if cur_c >= 0 && cur_c < COLS as i32 && cur_r >= 0 && cur_r < ROWS as i32 {
                let r = cur_r as usize;
                let c = cur_c as usize;
                match self.maze[r][c] {
                    0 => {
                        self.maze[r][c] = 3;
                        self.pellets_left = self.pellets_left.saturating_sub(1);
                        self.pellets_eaten += 1;
                        let pts = (10.0 * self.difficulty.score_multiplier()) as u32;
                        self.score += pts;

                        // Rastro de migalhas
                        self.particles.push(Particle {
                            x: self.pac_x,
                            y: self.pac_y,
                            vx: rand::gen_range(-1.2, 1.2),
                            vy: rand::gen_range(-1.2, 1.2),
                            color: Color::new(0.98, 0.90, 0.60, 0.8),
                            radius: 2.0,
                            life: 0.25,
                            max_life: 0.25,
                        });

                        self.check_fruit_spawn();
                    }
                    2 => {
                        self.maze[r][c] = 3;
                        self.pellets_left = self.pellets_left.saturating_sub(1);
                        self.pellets_eaten += 1;
                        let pts = (50.0 * self.difficulty.score_multiplier()) as u32;
                        self.score += pts;
                        self.frightened_timer = self.difficulty.frightened_time();
                        self.ghost_multiplier = 1;

                        // Todos os fantasmas ativos ficam assustados e dão meia-volta
                        for g in &mut self.ghosts {
                            if g.state == GhostState::Chase || g.state == GhostState::Scatter {
                                g.state = GhostState::Frightened;
                                g.dir = match g.dir {
                                    Direction::Up => Direction::Down,
                                    Direction::Down => Direction::Up,
                                    Direction::Left => Direction::Right,
                                    Direction::Right => Direction::Left,
                                    Direction::None => Direction::None,
                                };
                            }
                        }

                        // Ondas de choque do Energizer
                        for _ in 0..16 {
                            let ang = rand::gen_range(0.0, std::f32::consts::TAU);
                            let spd = rand::gen_range(2.0, 5.0);
                            self.particles.push(Particle {
                                x: self.pac_x,
                                y: self.pac_y,
                                vx: ang.cos() * spd,
                                vy: ang.sin() * spd,
                                color: YELLOW_ACCENT,
                                radius: 3.5,
                                life: 0.45,
                                max_life: 0.45,
                            });
                        }

                        self.popups.push(ScorePopup {
                            x: self.pac_x,
                            y: self.pac_y - 12.0,
                            text: "+50 ENERGIZER!".into(),
                            color: YELLOW_ACCENT,
                            life: 1.0,
                        });

                        self.check_fruit_spawn();
                    }
                    _ => {}
                }
            }
        }

        // 6. Checagem de Conclusão do Labirinto
        if self.pellets_left == 0 {
            self.phase = GamePhase::StageClear;
            self.phase_timer = 2.4;
            self.wall_flash_timer = 2.4;
            let bonus = 1000 * self.stage;
            self.score += (bonus as f32 * self.difficulty.score_multiplier()) as u32;
            self.popups.push(ScorePopup {
                x: Self::tile_cx(10),
                y: Self::tile_cy(10),
                text: format!("FASE LIMPA! +{} BÔNUS", bonus),
                color: EMERALD,
                life: 2.2,
            });
        }
    }

    fn check_fruit_spawn(&mut self) {
        if self.active_fruit.is_none() {
            if (self.fruit_spawned_count == 0 && self.pellets_eaten >= 70)
                || (self.fruit_spawned_count == 1 && self.pellets_eaten >= 160)
            {
                let fruit = FruitKind::for_stage(self.stage);
                self.active_fruit = Some(fruit);
                self.fruit_timer = 10.0;
                self.fruit_spawned_count += 1;
                self.popups.push(ScorePopup {
                    x: self.fruit_pos.0,
                    y: self.fruit_pos.1 - 16.0,
                    text: format!("BÔNUS: {}!", fruit.label()),
                    color: YELLOW_ACCENT,
                    life: 1.8,
                });
            }
        }
    }

    fn update_fruits(&mut self, dt: f32) {
        if let Some(fruit) = self.active_fruit {
            self.fruit_timer -= dt;
            if self.fruit_timer <= 0.0 {
                self.active_fruit = None;
            } else {
                // Checa colisão com Pacman
                let dist = ((self.pac_x - self.fruit_pos.0).powi(2) + (self.pac_y - self.fruit_pos.1).powi(2)).sqrt();
                if dist < 16.0 {
                    let pts = (fruit.points() as f32 * self.difficulty.score_multiplier()) as u32;
                    self.score += pts;
                    self.collected_fruits.push(fruit);
                    self.active_fruit = None;

                    self.popups.push(ScorePopup {
                        x: self.fruit_pos.0,
                        y: self.fruit_pos.1 - 14.0,
                        text: format!("+{} {}", pts, fruit.label()),
                        color: YELLOW_ACCENT,
                        life: 1.5,
                    });

                    for _ in 0..12 {
                        let ang = rand::gen_range(0.0, std::f32::consts::TAU);
                        let spd = rand::gen_range(1.5, 4.0);
                        self.particles.push(Particle {
                            x: self.fruit_pos.0,
                            y: self.fruit_pos.1,
                            vx: ang.cos() * spd,
                            vy: ang.sin() * spd,
                            color: CORAL,
                            radius: 3.0,
                            life: 0.4,
                            max_life: 0.4,
                        });
                    }
                }
            }
        }
    }

    // =========================================================================
    // 5. INTELIGÊNCIA ARTIFICIAL DOS 4 FANTASMAS
    // =========================================================================

    fn update_ghosts(&mut self, dt: f32) {
        let base_speed = self.difficulty.ghost_speed();
        let pac_col = (self.pac_x / TILE_SIZE).floor() as i32;
        let pac_row = (self.pac_y / TILE_SIZE).floor() as i32;
        let (pac_dx, pac_dy) = self.dir.to_offset();
        let maze = self.maze;
        let pellets_left = self.pellets_left;
        let frightened_timer = self.frightened_timer;
        let is_wave_chase = self.is_wave_chase;

        // Localiza posição de Blinky para o cálculo tático de Inky
        let (blinky_col, blinky_row) = self
            .ghosts
            .iter()
            .find(|g| g.ghost_type == GhostType::Blinky)
            .map(|g| ((g.x / TILE_SIZE).floor() as i32, (g.y / TILE_SIZE).floor() as i32))
            .unwrap_or((pac_col, pac_row));

        for g in &mut self.ghosts {
            // 1. Gestão da Casa dos Fantasmas
            if g.state == GhostState::InHouse {
                g.house_timer -= dt;
                // Animação de flutuação vertical dentro da casa
                g.y += (get_time() * 5.0).sin() as f32 * 0.35;
                if g.house_timer <= 0.0 {
                    g.state = GhostState::ExitingHouse;
                }
                continue;
            }

            if g.state == GhostState::ExitingHouse {
                let target_x = Self::tile_cx(10);
                let target_y = Self::tile_cy(8);

                let dx = target_x - g.x;
                let dy = target_y - g.y;

                if dx.abs() > 1.0 {
                    g.x += dx.signum() * base_speed;
                    g.dir = if dx > 0.0 { Direction::Right } else { Direction::Left };
                } else if dy.abs() > 1.0 {
                    g.x = target_x;
                    g.y += dy.signum() * base_speed;
                    g.dir = Direction::Up;
                } else {
                    g.x = target_x;
                    g.y = target_y;
                    g.state = if frightened_timer > 0.0 {
                        GhostState::Frightened
                    } else if is_wave_chase {
                        GhostState::Chase
                    } else {
                        GhostState::Scatter
                    };
                    g.dir = Direction::Left;
                }
                continue;
            }

            // 2. Determinação da Velocidade Dinâmica
            let speed = match g.state {
                GhostState::EatenEyes => 4.4,
                GhostState::Frightened => base_speed * 0.60,
                GhostState::Chase | GhostState::Scatter => {
                    // Cruise Elroy: Blinky acelera quando restam poucas pílulas
                    if g.ghost_type == GhostType::Blinky {
                        if pellets_left < 15 {
                            base_speed * 1.25
                        } else if pellets_left < 30 {
                            base_speed * 1.12
                        } else {
                            base_speed
                        }
                    } else {
                        base_speed
                    }
                }
                _ => base_speed,
            };

            let move_dist = speed * (dt * 60.0).clamp(0.5, 2.0);

            // 3. Alinhamento de Grade e Escolha de Direção nas Interseções
            let col = (g.x / TILE_SIZE).floor() as i32;
            let row = (g.y / TILE_SIZE).floor() as i32;
            let cx = Self::tile_cx(col);
            let cy = Self::tile_cy(row);

            let is_near_center = match g.dir {
                Direction::Right => (cx - g.x) >= 0.0 && (cx - g.x) <= move_dist,
                Direction::Left => (g.x - cx) >= 0.0 && (g.x - cx) <= move_dist,
                Direction::Down => (cy - g.y) >= 0.0 && (cy - g.y) <= move_dist,
                Direction::Up => (g.y - cy) >= 0.0 && (g.y - cy) <= move_dist,
                Direction::None => true,
            };

            if is_near_center {
                g.x = cx;
                g.y = cy;

                // Destino dos Olhos Eaten: Portão da Casa (10, 8), depois centro da casa (10, 10)
                if g.state == GhostState::EatenEyes {
                    if col == 10 && row == 8 {
                        // Entra na casa
                        g.dir = Direction::Down;
                    } else if col == 10 && row == 10 {
                        // Renasceu!
                        g.state = GhostState::ExitingHouse;
                        continue;
                    }
                }

                // Cálculo do Alvo Autêntico conforme a Personalidade
                let target_tile: (i32, i32) = match g.state {
                    GhostState::EatenEyes => (10, 8),
                    GhostState::Scatter => g.scatter_target,
                    GhostState::Frightened => {
                        // Alvo aleatório / disperso quando assustado
                        (rand::gen_range(0, COLS as i32), rand::gen_range(0, ROWS as i32))
                    }
                    GhostState::Chase => match g.ghost_type {
                        GhostType::Blinky => (pac_col, pac_row),
                        GhostType::Pinky => (pac_col + 4 * pac_dx, pac_row + 4 * pac_dy),
                        GhostType::Inky => {
                            let pivot_c = pac_col + 2 * pac_dx;
                            let pivot_r = pac_row + 2 * pac_dy;
                            (pivot_c + (pivot_c - blinky_col), pivot_r + (pivot_r - blinky_row))
                        }
                        GhostType::Clyde => {
                            let dist = (((col - pac_col).pow(2) + (row - pac_row).pow(2)) as f32).sqrt();
                            if dist > 8.0 {
                                (pac_col, pac_row)
                            } else {
                                g.scatter_target
                            }
                        }
                    },
                    _ => (pac_col, pac_row),
                };

                // Avalia candidatos de direção (exceto oposta para evitar ping-pong)
                let all_dirs = [Direction::Up, Direction::Left, Direction::Down, Direction::Right];
                let is_eyes = g.state == GhostState::EatenEyes;

                let mut valid_dirs = Vec::new();
                for &d in &all_dirs {
                    if !d.is_opposite(g.dir) || g.dir == Direction::None {
                        let (dx, dy) = d.to_offset();
                        let nc = col + dx;
                        let nr = row + dy;
                        if Self::is_ghost_walkable(&maze, nc, nr, is_eyes) {
                            valid_dirs.push(d);
                        }
                    }
                }

                // Se encurralado em beco sem saída, permite dar meia-volta
                if valid_dirs.is_empty() {
                    let opposite = match g.dir {
                        Direction::Up => Direction::Down,
                        Direction::Down => Direction::Up,
                        Direction::Left => Direction::Right,
                        Direction::Right => Direction::Left,
                        Direction::None => Direction::None,
                    };
                    let (dx, dy) = opposite.to_offset();
                    if Self::is_ghost_walkable(&maze, col + dx, row + dy, is_eyes) {
                        valid_dirs.push(opposite);
                    }
                }

                if !valid_dirs.is_empty() {
                    if g.state == GhostState::Frightened {
                        let idx = rand::gen_range(0, valid_dirs.len());
                        g.dir = valid_dirs[idx];
                    } else {
                        // Ordena pelo menor quadrado da distância Euclidiana até o alvo
                        valid_dirs.sort_by(|&a, &b| {
                            let (dxa, dya) = a.to_offset();
                            let (dxb, dyb) = b.to_offset();
                            let d1 = (col + dxa - target_tile.0).pow(2) + (row + dya - target_tile.1).pow(2);
                            let d2 = (col + dxb - target_tile.0).pow(2) + (row + dyb - target_tile.1).pow(2);
                            d1.cmp(&d2)
                        });
                        g.dir = valid_dirs[0];
                    }
                }
            }

            // 4. Execução do Movimento
            let (gdx, gdy) = g.dir.to_offset();
            g.x += (gdx as f32) * move_dist;
            g.y += (gdy as f32) * move_dist;

            // Túnel lateral dos fantasmas
            let total_w = COLS as f32 * TILE_SIZE;
            if g.x < -TILE_SIZE * 0.5 {
                g.x += total_w + TILE_SIZE;
            } else if g.x > total_w + TILE_SIZE * 0.5 {
                g.x -= total_w + TILE_SIZE;
            }
        }
    }

    fn check_pacman_ghost_collisions(&mut self) {
        let mut caught = false;

        for g in &mut self.ghosts {
            if g.state == GhostState::InHouse || g.state == GhostState::ExitingHouse || g.state == GhostState::EatenEyes {
                continue;
            }

            let dist = ((self.pac_x - g.x).powi(2) + (self.pac_y - g.y).powi(2)).sqrt();
            if dist < 17.0 {
                if g.state == GhostState::Frightened {
                    // Come o fantasma!
                    let pts = 200 * self.ghost_multiplier;
                    self.ghost_multiplier *= 2;
                    let real_pts = (pts as f32 * self.difficulty.score_multiplier()) as u32;
                    self.score += real_pts;
                    self.ghosts_eaten_total += 1;

                    g.state = GhostState::EatenEyes;

                    self.popups.push(ScorePopup {
                        x: g.x,
                        y: g.y - 12.0,
                        text: format!("+{}", real_pts),
                        color: ICE_BLUE,
                        life: 1.2,
                    });

                    // Partículas de alma espectral
                    for _ in 0..14 {
                        let ang = rand::gen_range(0.0, std::f32::consts::TAU);
                        let spd = rand::gen_range(1.0, 3.5);
                        self.particles.push(Particle {
                            x: g.x,
                            y: g.y,
                            vx: ang.cos() * spd,
                            vy: ang.sin() * spd,
                            color: Color::new(0.3, 0.7, 1.0, 0.9),
                            radius: 3.0,
                            life: 0.35,
                            max_life: 0.35,
                        });
                    }
                } else if g.state == GhostState::Chase || g.state == GhostState::Scatter {
                    caught = true;
                    break;
                }
            }
        }

        if caught {
            self.lives -= 1;
            self.phase = GamePhase::PacmanDying;
            self.phase_timer = 1.6;

            // Explosão de morte do Pacman
            for _ in 0..20 {
                let ang = rand::gen_range(0.0, std::f32::consts::TAU);
                let spd = rand::gen_range(1.5, 5.0);
                self.particles.push(Particle {
                    x: self.pac_x,
                    y: self.pac_y,
                    vx: ang.cos() * spd,
                    vy: ang.sin() * spd,
                    color: YELLOW_ACCENT,
                    radius: 3.0,
                    life: 0.8,
                    max_life: 0.8,
                });
            }

            if self.lives <= 0 {
                if self.score > self.high_score {
                    self.high_score = self.score;
                }
            }
        }
    }
}

// =========================================================================
// 6. IMPLEMENTAÇÃO DO TRAIT ARCADE_GAME
// =========================================================================

impl ArcadeGame for PacmanGame {
    fn update(&mut self) {
        let dt = get_frame_time().min(0.05);

        // Atualização de partículas
        self.particles.retain_mut(|p| {
            p.x += p.vx;
            p.y += p.vy;
            p.vx *= 0.95;
            p.vy *= 0.95;
            p.life -= dt;
            p.life > 0.0
        });

        // Atualização de popups
        self.popups.retain_mut(|pop| {
            pop.y -= 0.6;
            pop.life -= dt;
            pop.life > 0.0
        });

        // Alternância de Dificuldade
        if is_key_pressed(KeyCode::Key1) {
            self.set_difficulty(Difficulty::Casual);
        } else if is_key_pressed(KeyCode::Key2) {
            self.set_difficulty(Difficulty::Normal);
        } else if is_key_pressed(KeyCode::Key3) {
            self.set_difficulty(Difficulty::Hardcore);
        }

        // Reiniciar
        if is_key_pressed(KeyCode::R) {
            self.reset();
            return;
        }

        match self.phase {
            GamePhase::ReadyCountdown => {
                self.phase_timer -= dt;
                if self.phase_timer <= 0.0 {
                    self.phase = GamePhase::Playing;
                }
            }
            GamePhase::Playing => {
                // Buffer de Teclas
                if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
                    self.next_dir = Direction::Up;
                } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
                    self.next_dir = Direction::Down;
                } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
                    self.next_dir = Direction::Left;
                } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
                    self.next_dir = Direction::Right;
                }

                // Temporizador de Energizer
                if self.frightened_timer > 0.0 {
                    self.frightened_timer -= dt;
                    if self.frightened_timer <= 0.0 {
                        for g in &mut self.ghosts {
                            if g.state == GhostState::Frightened {
                                g.state = if self.is_wave_chase {
                                    GhostState::Chase
                                } else {
                                    GhostState::Scatter
                                };
                            }
                        }
                    }
                } else {
                    // Ondas clássicas de Chase vs Scatter
                    self.wave_timer -= dt;
                    if self.wave_timer <= 0.0 {
                        self.is_wave_chase = !self.is_wave_chase;
                        self.wave_timer = if self.is_wave_chase { 20.0 } else { 7.0 };
                        for g in &mut self.ghosts {
                            if g.state == GhostState::Chase || g.state == GhostState::Scatter {
                                g.state = if self.is_wave_chase {
                                    GhostState::Chase
                                } else {
                                    GhostState::Scatter
                                };
                            }
                        }
                    }
                }

                self.update_pacman_movement(dt);
                self.update_fruits(dt);
                self.update_ghosts(dt);
                self.check_pacman_ghost_collisions();
            }
            GamePhase::PacmanDying => {
                self.phase_timer -= dt;
                if self.phase_timer <= 0.0 {
                    if self.lives <= 0 {
                        self.phase = GamePhase::GameOver;
                    } else {
                        self.reset_positions_for_round();
                    }
                }
            }
            GamePhase::StageClear => {
                self.wall_flash_timer -= dt;
                self.phase_timer -= dt;
                if self.phase_timer <= 0.0 {
                    self.setup_stage(self.stage + 1, false);
                }
            }
            GamePhase::GameOver => {
                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                    self.reset();
                }
            }
        }
    }

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let ox = (screen_w - (COLS as f32 * TILE_SIZE)) * 0.5;
        let oy = 56.0 + (screen_h - 56.0 - (ROWS as f32 * TILE_SIZE)) * 0.5;

        // Fundo e moldura do tabuleiro arcade
        draw_rectangle(
            ox - 10.0,
            oy - 10.0,
            (COLS as f32 * TILE_SIZE) + 20.0,
            (ROWS as f32 * TILE_SIZE) + 20.0,
            BG_COLOR,
        );
        draw_rectangle_lines(
            ox - 10.0,
            oy - 10.0,
            (COLS as f32 * TILE_SIZE) + 20.0,
            (ROWS as f32 * TILE_SIZE) + 20.0,
            2.0,
            BORDER_COLOR,
        );

        // =====================================================================
        // 1. RENDERIZAÇÃO DO LABIRINTO COM BRILHO NEON DUPLO
        // =====================================================================
        let is_flashing = self.wall_flash_timer > 0.0 && ((self.wall_flash_timer * 10.0) as i32 % 2 == 0);
        let wall_bg = if is_flashing { Color::new(0.2, 0.4, 0.9, 1.0) } else { Color::new(0.05, 0.08, 0.18, 1.0) };
        let wall_line = if is_flashing { TEXT_PRIMARY } else { COBALT_ACCENT };
        let wall_inner = if is_flashing { ICE_BLUE } else { Color::new(0.35, 0.65, 0.98, 0.8) };

        for r in 0..ROWS {
            for c in 0..COLS {
                let tx = ox + (c as f32) * TILE_SIZE;
                let ty = oy + (r as f32) * TILE_SIZE;

                match self.maze[r][c] {
                    1 => {
                        // Bloco de parede sólida com linhas de neon duplas
                        draw_rectangle(tx + 1.0, ty + 1.0, TILE_SIZE - 2.0, TILE_SIZE - 2.0, wall_bg);
                        draw_rectangle_lines(tx + 1.5, ty + 1.5, TILE_SIZE - 3.0, TILE_SIZE - 3.0, 1.5, wall_line);
                        draw_rectangle_lines(tx + 4.5, ty + 4.5, TILE_SIZE - 9.0, TILE_SIZE - 9.0, 1.0, wall_inner);
                    }
                    5 => {
                        // Portão da Casa dos Fantasmas (Linha Magenta Pulsante)
                        let pulse = (get_time() * 8.0).sin().abs() as f32 * 0.3;
                        draw_line(
                            tx,
                            ty + TILE_SIZE * 0.5,
                            tx + TILE_SIZE,
                            ty + TILE_SIZE * 0.5,
                            2.5,
                            Color::new(1.0, 0.25, 0.65, 0.7 + pulse),
                        );
                    }
                    0 => {
                        // Pílula comum suavemente brilhante
                        draw_circle(tx + TILE_SIZE * 0.5, ty + TILE_SIZE * 0.5, 3.2, Color::new(0.98, 0.92, 0.75, 0.95));
                    }
                    2 => {
                        // Energizer pulsante com anéis concêntricos
                        let pulse = (get_time() * 7.0).sin().abs() as f32 * 2.5;
                        draw_circle(tx + TILE_SIZE * 0.5, ty + TILE_SIZE * 0.5, 7.0 + pulse, Color::new(0.98, 0.80, 0.08, 0.3));
                        draw_circle(tx + TILE_SIZE * 0.5, ty + TILE_SIZE * 0.5, 6.0, YELLOW_ACCENT);
                        draw_circle(tx + TILE_SIZE * 0.5, ty + TILE_SIZE * 0.5, 2.5, TEXT_PRIMARY);
                    }
                    4 => {
                        // Setas indicativas nos túneis laterais
                        let arrow_col = Color::new(0.2, 0.4, 0.8, 0.4);
                        if c == 0 {
                            draw_triangle(
                                Vec2::new(tx + 4.0, ty + 6.0),
                                Vec2::new(tx + 4.0, ty + 18.0),
                                Vec2::new(tx + 12.0, ty + 12.0),
                                arrow_col,
                            );
                        } else {
                            draw_triangle(
                                Vec2::new(tx + 20.0, ty + 6.0),
                                Vec2::new(tx + 20.0, ty + 18.0),
                                Vec2::new(tx + 12.0, ty + 12.0),
                                arrow_col,
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        // =====================================================================
        // 2. FRUTA BÔNUS COM AURA E CONTAGEM
        // =====================================================================
        if let Some(fruit) = self.active_fruit {
            let fx = ox + self.fruit_pos.0;
            let fy = oy + self.fruit_pos.1;
            let fruit_pulse = (get_time() * 6.0).sin().abs() as f32 * 3.0;

            // Halo pulsante
            draw_circle(fx, fy, 12.0 + fruit_pulse, Color::new(0.98, 0.80, 0.08, 0.25));

            Self::draw_fruit_vector(fruit, fx, fy, 10.0);

            // Arco do temporizador restante
            let pct = (self.fruit_timer / 10.0).clamp(0.0, 1.0);
            draw_circle_lines(fx, fy, 14.0, 1.5, Color::new(0.98, 0.80, 0.08, 0.7 * pct));
        }

        // =====================================================================
        // 3. FANTASMAS (CORPO EM CÚPULA + TENTÁCULOS + OLHOS)
        // =====================================================================
        for g in &self.ghosts {
            let gx = ox + g.x;
            let gy = oy + g.y;

            if g.state == GhostState::EatenEyes {
                // Apenas olhos correndo velozes de volta à casa
                let (edx, edy) = g.dir.to_offset();
                let ox_eye = (edx as f32) * 2.2;
                let oy_eye = (edy as f32) * 2.2;

                draw_circle(gx - 4.5, gy - 2.0, 4.0, TEXT_PRIMARY);
                draw_circle(gx + 4.5, gy - 2.0, 4.0, TEXT_PRIMARY);
                draw_circle(gx - 4.5 + ox_eye, gy - 2.0 + oy_eye, 2.0, Color::new(0.08, 0.20, 0.65, 1.0));
                draw_circle(gx + 4.5 + ox_eye, gy - 2.0 + oy_eye, 2.0, Color::new(0.08, 0.20, 0.65, 1.0));
                continue;
            }

            let ghost_color = if g.state == GhostState::Frightened {
                if self.frightened_timer < 2.2 && ((self.frightened_timer * 8.0) as i32 % 2 == 0) {
                    TEXT_PRIMARY // Pisca em branco quando o tempo estiver acabando
                } else {
                    Color::new(0.18, 0.35, 0.95, 1.0) // Azul assustado clássico
                }
            } else {
                g.color
            };

            // Cúpula superior
            draw_circle(gx, gy - 2.0, 10.0, ghost_color);
            // Corpo central
            draw_rectangle(gx - 10.0, gy - 2.0, 20.0, 9.0, ghost_color);

            // 4 Pés ondulados na base que balançam
            let wave_time = (get_time() * 10.0).sin() as f32 * 1.5;
            draw_triangle(
                Vec2::new(gx - 10.0, gy + 7.0),
                Vec2::new(gx - 5.0, gy + 7.0),
                Vec2::new(gx - 7.5, gy + 11.0 + wave_time),
                ghost_color,
            );
            draw_triangle(
                Vec2::new(gx - 5.0, gy + 7.0),
                Vec2::new(gx, gy + 7.0),
                Vec2::new(gx - 2.5, gy + 11.0 - wave_time),
                ghost_color,
            );
            draw_triangle(
                Vec2::new(gx, gy + 7.0),
                Vec2::new(gx + 5.0, gy + 7.0),
                Vec2::new(gx + 2.5, gy + 11.0 + wave_time),
                ghost_color,
            );
            draw_triangle(
                Vec2::new(gx + 5.0, gy + 7.0),
                Vec2::new(gx + 10.0, gy + 7.0),
                Vec2::new(gx + 7.5, gy + 11.0 - wave_time),
                ghost_color,
            );

            // Olhos
            if g.state == GhostState::Frightened {
                draw_circle(gx - 4.0, gy - 3.0, 1.8, YELLOW_ACCENT);
                draw_circle(gx + 4.0, gy - 3.0, 1.8, YELLOW_ACCENT);
                // Boca trêmula
                draw_line(gx - 5.0, gy + 3.0, gx - 2.5, gy + 1.5, 1.5, YELLOW_ACCENT);
                draw_line(gx - 2.5, gy + 1.5, gx, gy + 3.0, 1.5, YELLOW_ACCENT);
                draw_line(gx, gy + 3.0, gx + 2.5, gy + 1.5, 1.5, YELLOW_ACCENT);
                draw_line(gx + 2.5, gy + 1.5, gx + 5.0, gy + 3.0, 1.5, YELLOW_ACCENT);
            } else {
                let (edx, edy) = g.dir.to_offset();
                let ox_eye = (edx as f32) * 2.2;
                let oy_eye = (edy as f32) * 2.2;

                draw_circle(gx - 4.5, gy - 3.0, 3.8, TEXT_PRIMARY);
                draw_circle(gx + 4.5, gy - 3.0, 3.8, TEXT_PRIMARY);

                draw_circle(gx - 4.5 + ox_eye, gy - 3.0 + oy_eye, 1.9, Color::new(0.08, 0.15, 0.45, 1.0));
                draw_circle(gx + 4.5 + ox_eye, gy - 3.0 + oy_eye, 1.9, Color::new(0.08, 0.15, 0.45, 1.0));
            }
        }

        // =====================================================================
        // 4. PAC-MAN COM ROTAÇÃO E ANIMAÇÃO DE CHOMP
        // =====================================================================
        if self.phase != GamePhase::PacmanDying || ((self.phase_timer * 12.0) as i32 % 2 == 0) {
            let px = ox + self.pac_x;
            let py = oy + self.pac_y;
            let mouth = (self.chomp_angle.sin().abs()) * 0.45;
            let angle = self.dir.to_angle();

            draw_circle(px, py, 11.0, YELLOW_ACCENT);
            draw_circle_lines(px, py, 11.0, 1.0, Color::new(1.0, 0.95, 0.5, 0.8));

            // Boca cortada no ângulo de movimento
            if mouth > 0.05 && self.dir != Direction::None {
                let mouth_len = 14.0;
                draw_triangle(
                    Vec2::new(px, py),
                    Vec2::new(px + (angle - mouth).cos() * mouth_len, py + (angle - mouth).sin() * mouth_len),
                    Vec2::new(px + (angle + mouth).cos() * mouth_len, py + (angle + mouth).sin() * mouth_len),
                    BG_COLOR,
                );
            }

            // Olho expressivo do Pacman
            if self.dir != Direction::None {
                let perp_angle = angle - std::f32::consts::FRAC_PI_2;
                let eye_x = px + angle.cos() * 2.5 + perp_angle.cos() * 5.0;
                let eye_y = py + angle.sin() * 2.5 + perp_angle.sin() * 5.0;
                draw_circle(eye_x, eye_y, 1.5, Color::new(0.1, 0.1, 0.1, 0.9));
                draw_circle(eye_x + 0.4, eye_y - 0.4, 0.6, TEXT_PRIMARY);
            }
        }

        // =====================================================================
        // 5. PARTÍCULAS E POPUPS FLUTUANTES
        // =====================================================================
        for p in &self.particles {
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            let mut col = p.color;
            col.a *= alpha;
            draw_circle(ox + p.x, oy + p.y, p.radius, col);
        }

        for pop in &self.popups {
            let alpha = (pop.life / 1.0).clamp(0.0, 1.0);
            let mut col = pop.color;
            col.a = alpha;
            let dim = measure_text(&pop.text, None, 18, 1.0);
            draw_text(&pop.text, ox + pop.x - dim.width * 0.5, oy + pop.y, 18.0, col);
        }

        // =====================================================================
        // 6. PAINEL LATERAL ESQUERDO (RADAR DOS FANTASMAS + DIFICULDADE)
        // =====================================================================
        let left_x = (ox - 330.0).max(18.0);
        let left_w = 310.0;
        let mut left_y = oy;

        // Card 1: Título e Dificuldade
        draw_card_rect(left_x, left_y, left_w, 150.0, SURFACE_COLOR, BORDER_COLOR);
        draw_text("PACMAN NEON ARCADE", left_x + 16.0, left_y + 26.0, 20.0, YELLOW_ACCENT);

        let stage_str = format!("FASE {:02}", self.stage);
        draw_rectangle(left_x + 16.0, left_y + 36.0, 80.0, 22.0, Color::new(0.1, 0.2, 0.4, 0.8));
        draw_rectangle_lines(left_x + 16.0, left_y + 36.0, 80.0, 22.0, 1.0, ICE_BLUE);
        draw_text(&stage_str, left_x + 24.0, left_y + 51.0, 14.0, TEXT_PRIMARY);

        draw_text("SELEÇÃO DE DIFICULDADE:", left_x + 16.0, left_y + 80.0, 13.0, TEXT_MUTED);

        let diffs = [
            ("[1] CASUAL", Difficulty::Casual),
            ("[2] NORMAL", Difficulty::Normal),
            ("[3] HARDCORE", Difficulty::Hardcore),
        ];
        for (i, (d_name, d_val)) in diffs.iter().enumerate() {
            let is_sel = self.difficulty == *d_val;
            let bx = left_x + 16.0 + (i as f32) * 92.0;
            let by = left_y + 94.0;
            let bg_c = if is_sel { COBALT_ACCENT } else { Color::new(0.08, 0.12, 0.20, 1.0) };
            let tx_c = if is_sel { TEXT_PRIMARY } else { TEXT_MUTED };

            draw_rectangle(bx, by, 86.0, 26.0, bg_c);
            draw_rectangle_lines(bx, by, 86.0, 26.0, 1.0, if is_sel { ICE_BLUE } else { BORDER_COLOR });
            draw_text(d_name, bx + 6.0, by + 17.0, 11.0, tx_c);
        }

        left_y += 166.0;

        // Card 2: Radar de IA dos 4 Fantasmas
        draw_card_rect(left_x, left_y, left_w, 320.0, SURFACE_COLOR, BORDER_COLOR);
        draw_text("RADAR TÁTICO DOS FANTASMAS", left_x + 16.0, left_y + 24.0, 15.0, TEXT_PRIMARY);

        let wave_label = if self.frightened_timer > 0.0 {
            format!("ALERTA ENERGIZER: {:.1}s", self.frightened_timer)
        } else if self.is_wave_chase {
            format!("ONDA: PERSEGUIÇÃO ({:.1}s)", self.wave_timer)
        } else {
            format!("ONDA: DISPERSÃO ({:.1}s)", self.wave_timer)
        };
        let wave_color = if self.frightened_timer > 0.0 {
            YELLOW_ACCENT
        } else if self.is_wave_chase {
            CORAL
        } else {
            EMERALD
        };
        draw_text(&wave_label, left_x + 16.0, left_y + 44.0, 13.0, wave_color);

        let ghost_info = [
            ("BLINKY", "Sombra", "Perseguição Direta", Color::new(0.96, 0.22, 0.22, 1.0)),
            ("PINKY", "Veloz", "Emboscada Antecipada", Color::new(0.98, 0.60, 0.78, 1.0)),
            ("INKY", "Tímido", "Flanqueamento Tático", Color::new(0.0, 0.90, 0.95, 1.0)),
            ("CLYDE", "Covarde", "Patrulha / Fuga", Color::new(0.98, 0.62, 0.12, 1.0)),
        ];

        for (i, (name, nickname, role, col)) in ghost_info.iter().enumerate() {
            let gy_pos = left_y + 66.0 + (i as f32) * 60.0;
            draw_rectangle(left_x + 12.0, gy_pos, left_w - 24.0, 52.0, Color::new(0.05, 0.07, 0.12, 0.8));
            draw_rectangle_lines(left_x + 12.0, gy_pos, left_w - 24.0, 52.0, 1.0, Color::new(0.15, 0.20, 0.30, 0.6));

            // Mini Avatar
            draw_circle(left_x + 30.0, gy_pos + 26.0, 10.0, *col);
            draw_circle(left_x + 27.0, gy_pos + 24.0, 3.0, TEXT_PRIMARY);
            draw_circle(left_x + 33.0, gy_pos + 24.0, 3.0, TEXT_PRIMARY);
            draw_circle(left_x + 28.0, gy_pos + 24.0, 1.4, Color::new(0.08, 0.15, 0.45, 1.0));
            draw_circle(left_x + 34.0, gy_pos + 24.0, 1.4, Color::new(0.08, 0.15, 0.45, 1.0));

            // Informações
            let title = format!("{} ({})", name, nickname);
            draw_text(&title, left_x + 50.0, gy_pos + 20.0, 14.0, *col);
            draw_text(role, left_x + 50.0, gy_pos + 36.0, 12.0, TEXT_MUTED);

            // Estado atual do fantasma
            let cur_state = self.ghosts.get(i).map(|g| g.state).unwrap_or(GhostState::Chase);
            let (st_text, st_col) = match cur_state {
                GhostState::InHouse => ("NA CASA", TEXT_MUTED),
                GhostState::ExitingHouse => ("SAINDO", ICE_BLUE),
                GhostState::Chase => ("CAÇANDO", CORAL),
                GhostState::Scatter => ("DISPERSO", EMERALD),
                GhostState::Frightened => ("VULNERÁVEL", YELLOW_ACCENT),
                GhostState::EatenEyes => ("OLHOS/BASE", ICE_BLUE),
            };
            draw_text(st_text, left_x + 215.0, gy_pos + 28.0, 11.0, st_col);
        }

        // =====================================================================
        // 7. PAINEL LATERAL DIREITO (PLACAR, METAS, FRUTAS E CONTROLES)
        // =====================================================================
        let right_x = ox + (COLS as f32 * TILE_SIZE) + 20.0;
        let right_w = 310.0;
        let mut right_y = oy;

        // Card 1: Placar e Recorde
        draw_card_rect(right_x, right_y, right_w, 130.0, SURFACE_COLOR, BORDER_COLOR);
        draw_text("PONTUAÇÃO ATUAL", right_x + 16.0, right_y + 24.0, 13.0, TEXT_MUTED);
        let score_txt = format!("{:06}", self.score);
        draw_text(&score_txt, right_x + 16.0, right_y + 58.0, 32.0, YELLOW_ACCENT);

        let mult_txt = format!("Multiplicador: x{:.1}", self.difficulty.score_multiplier());
        draw_text(&mult_txt, right_x + 175.0, right_y + 42.0, 12.0, ICE_BLUE);

        draw_text("RECORDE DA PLATAFORMA:", right_x + 16.0, right_y + 88.0, 12.0, TEXT_MUTED);
        let hi_txt = format!("{:06} PTS", self.high_score.max(self.score));
        draw_text(&hi_txt, right_x + 16.0, right_y + 110.0, 18.0, TEXT_PRIMARY);

        right_y += 144.0;

        // Card 2: Progresso do Labirinto
        draw_card_rect(right_x, right_y, right_w, 100.0, SURFACE_COLOR, BORDER_COLOR);
        draw_text("PROGRESSO DO LABIRINTO", right_x + 16.0, right_y + 24.0, 14.0, TEXT_PRIMARY);

        let pel_txt = format!("Pílulas: {} / {}", self.pellets_eaten, self.pellets_total);
        draw_text(&pel_txt, right_x + 16.0, right_y + 48.0, 13.0, TEXT_MUTED);

        // Barra de Progresso Neon
        let pct = if self.pellets_total > 0 {
            (self.pellets_eaten as f32 / self.pellets_total as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };
        draw_rectangle(right_x + 16.0, right_y + 60.0, right_w - 32.0, 12.0, Color::new(0.08, 0.12, 0.20, 1.0));
        draw_rectangle(right_x + 16.0, right_y + 60.0, (right_w - 32.0) * pct, 12.0, YELLOW_ACCENT);
        draw_rectangle_lines(right_x + 16.0, right_y + 60.0, right_w - 32.0, 12.0, 1.0, BORDER_COLOR);

        let pct_txt = format!("{:.0}%", pct * 100.0);
        draw_text(&pct_txt, right_x + 235.0, right_y + 48.0, 13.0, ICE_BLUE);

        right_y += 114.0;

        // Card 3: Galeria de Frutas e Combo
        draw_card_rect(right_x, right_y, right_w, 110.0, SURFACE_COLOR, BORDER_COLOR);
        draw_text("COLEÇÃO DE FRUTAS BÔNUS", right_x + 16.0, right_y + 24.0, 14.0, TEXT_PRIMARY);

        let fruit_icons = [
            FruitKind::Cherry,
            FruitKind::Strawberry,
            FruitKind::Orange,
            FruitKind::Apple,
            FruitKind::Key,
        ];

        for (i, f) in fruit_icons.iter().enumerate() {
            let fx = right_x + 30.0 + (i as f32) * 55.0;
            let fy = right_y + 55.0;
            let collected = self.collected_fruits.contains(f);

            if collected {
                Self::draw_fruit_vector(*f, fx, fy, 11.0);
                draw_circle_lines(fx, fy, 16.0, 1.5, YELLOW_ACCENT);
            } else {
                Self::draw_fruit_vector(*f, fx, fy, 10.0);
                // Silhueta escurecida de fruta não coletada
                draw_circle(fx, fy, 14.0, Color::new(0.0, 0.0, 0.0, 0.65));
                draw_circle_lines(fx, fy, 14.0, 1.0, BORDER_COLOR);
            }

            let pts_sub = format!("+{}", f.points());
            draw_text(&pts_sub, fx - 14.0, fy + 28.0, 10.0, if collected { TEXT_PRIMARY } else { TEXT_MUTED });
        }

        right_y += 124.0;

        // Card 4: Controles e Dicas
        draw_card_rect(right_x, right_y, right_w, 100.0, SURFACE_COLOR, BORDER_COLOR);
        draw_text("GUIA DE CONTROLES", right_x + 16.0, right_y + 22.0, 13.0, ICE_BLUE);
        draw_text("• [WASD / Setas]: Movimento fluido com Buffer", right_x + 16.0, right_y + 42.0, 12.0, TEXT_MUTED);
        draw_text("• [Meia-Volta]: Curva 180° instantânea no corredor", right_x + 16.0, right_y + 58.0, 12.0, TEXT_MUTED);
        draw_text("• [1, 2, 3]: Alternar Dificuldade | [R]: Reiniciar", right_x + 16.0, right_y + 74.0, 12.0, TEXT_MUTED);

        // =====================================================================
        // 8. RODAPÉ INFERIOR (VIDAS E FRUTA ATUAL DA FASE)
        // =====================================================================
        let hud_y = oy + (ROWS as f32 * TILE_SIZE) + 16.0;

        // Vidas
        draw_text("VIDAS:", ox, hud_y + 14.0, 15.0, TEXT_MUTED);
        for i in 0..self.difficulty.starting_lives() {
            let lx = ox + 60.0 + (i as f32) * 26.0;
            let col = if i < self.lives {
                YELLOW_ACCENT
            } else {
                Color::new(0.2, 0.2, 0.3, 0.4)
            };
            draw_circle(lx, hud_y + 10.0, 8.0, col);
            if i < self.lives {
                draw_triangle(
                    Vec2::new(lx, hud_y + 10.0),
                    Vec2::new(lx + 10.0, hud_y + 6.0),
                    Vec2::new(lx + 10.0, hud_y + 14.0),
                    BG_COLOR,
                );
            }
        }

        let stage_fruit = FruitKind::for_stage(self.stage);
        let sf_x = ox + (COLS as f32 * TILE_SIZE) - 30.0;
        Self::draw_fruit_vector(stage_fruit, sf_x, hud_y + 10.0, 10.0);
        let sf_lbl = format!("FRUTA DA FASE: {}", stage_fruit.label());
        draw_text(&sf_lbl, sf_x - 170.0, hud_y + 14.0, 13.0, TEXT_MUTED);

        // =====================================================================
        // 9. OVERLAYS DE FASE (READY, GAME OVER, STAGE CLEAR)
        // =====================================================================
        match self.phase {
            GamePhase::ReadyCountdown => {
                let box_w = 240.0;
                let box_h = 60.0;
                let bx = ox + ((COLS as f32 * TILE_SIZE) - box_w) * 0.5;
                let by = oy + Self::tile_cy(12) - box_h * 0.5;

                draw_card_rect(bx, by, box_w, box_h, SURFACE_COLOR, YELLOW_ACCENT);
                let txt = "READY!";
                let dim = measure_text(txt, None, 28, 1.0);
                draw_text(txt, bx + (box_w - dim.width) * 0.5, by + 40.0, 28.0, YELLOW_ACCENT);
            }
            GamePhase::StageClear => {
                let box_w = 320.0;
                let box_h = 80.0;
                let bx = ox + ((COLS as f32 * TILE_SIZE) - box_w) * 0.5;
                let by = oy + Self::tile_cy(10) - box_h * 0.5;

                draw_card_rect(bx, by, box_w, box_h, SURFACE_COLOR, EMERALD);
                let txt = "FASE CONCLUÍDA!";
                let dim = measure_text(txt, None, 24, 1.0);
                draw_text(txt, bx + (box_w - dim.width) * 0.5, by + 34.0, 24.0, EMERALD);

                let sub = format!("AVANÇANDO PARA A FASE {:02}...", self.stage + 1);
                let sdim = measure_text(&sub, None, 14, 1.0);
                draw_text(&sub, bx + (box_w - sdim.width) * 0.5, by + 60.0, 14.0, TEXT_PRIMARY);
            }
            GamePhase::GameOver => {
                draw_rectangle(ox - 10.0, oy - 10.0, (COLS as f32 * TILE_SIZE) + 20.0, (ROWS as f32 * TILE_SIZE) + 20.0, Color::new(0.02, 0.03, 0.06, 0.85));

                let box_w = 380.0;
                let box_h = 240.0;
                let bx = ox + ((COLS as f32 * TILE_SIZE) - box_w) * 0.5;
                let by = oy + ((ROWS as f32 * TILE_SIZE) - box_h) * 0.5;

                draw_card_rect(bx, by, box_w, box_h, SURFACE_COLOR, CORAL);

                let txt = "GAME OVER";
                let dim = measure_text(txt, None, 32, 1.0);
                draw_text(txt, bx + (box_w - dim.width) * 0.5, by + 44.0, 32.0, CORAL);

                let s_txt = format!("PONTUAÇÃO FINAL: {} PTS", self.score);
                let s_dim = measure_text(&s_txt, None, 18, 1.0);
                draw_text(&s_txt, bx + (box_w - s_dim.width) * 0.5, by + 86.0, 18.0, YELLOW_ACCENT);

                let stats1 = format!("Fase Alcançada: {}  |  Fantasmas Comidos: {}", self.stage, self.ghosts_eaten_total);
                let dim1 = measure_text(&stats1, None, 13, 1.0);
                draw_text(&stats1, bx + (box_w - dim1.width) * 0.5, by + 120.0, 13.0, TEXT_PRIMARY);

                let stats2 = format!("Pílulas Consumidas: {}  |  Frutas Coletadas: {}", self.pellets_eaten, self.collected_fruits.len());
                let dim2 = measure_text(&stats2, None, 13, 1.0);
                draw_text(&stats2, bx + (box_w - dim2.width) * 0.5, by + 144.0, 13.0, TEXT_MUTED);

                let r_txt = "Pressione [ESPAÇO] ou [R] para Jogar Novamente";
                let r_dim = measure_text(r_txt, None, 14, 1.0);
                draw_text(r_txt, bx + (box_w - r_dim.width) * 0.5, by + 196.0, 14.0, ICE_BLUE);
            }
            _ => {}
        }
    }

    fn reset(&mut self) {
        let diff = self.difficulty;
        *self = Self::new();
        self.set_difficulty(diff);
    }

    fn get_score(&self) -> u32 {
        self.score
    }

    fn is_game_over(&self) -> bool {
        self.phase == GamePhase::GameOver
    }

    fn get_title(&self) -> &str {
        "Pacman Neon Arcade"
    }
}

// =========================================================================
// 7. DESENHO VETORIAL DE FRUTAS BÔNUS RETRÔ
// =========================================================================

impl PacmanGame {
    fn draw_fruit_vector(fruit: FruitKind, x: f32, y: f32, r: f32) {
        match fruit {
            FruitKind::Cherry => {
                // Duas cerejas vermelhas com hastes verdes curvas unidas
                draw_circle(x - r * 0.5, y + r * 0.3, r * 0.65, Color::new(0.95, 0.15, 0.25, 1.0));
                draw_circle(x + r * 0.5, y + r * 0.2, r * 0.60, Color::new(0.95, 0.15, 0.25, 1.0));
                // Brilho
                draw_circle(x - r * 0.65, y + r * 0.15, r * 0.18, TEXT_PRIMARY);
                draw_circle(x + r * 0.35, y + r * 0.05, r * 0.16, TEXT_PRIMARY);
                // Hastes
                draw_line(x - r * 0.5, y - r * 0.2, x, y - r * 0.9, 1.6, EMERALD);
                draw_line(x + r * 0.5, y - r * 0.3, x, y - r * 0.9, 1.6, EMERALD);
                // Folha
                draw_circle(x + r * 0.3, y - r * 0.85, r * 0.25, EMERALD);
            }
            FruitKind::Strawberry => {
                // Morango em formato cônico vermelho com folhas e sementes
                let body_col = Color::new(0.96, 0.20, 0.35, 1.0);
                draw_triangle(
                    Vec2::new(x - r * 0.8, y - r * 0.3),
                    Vec2::new(x + r * 0.8, y - r * 0.3),
                    Vec2::new(x, y + r * 0.9),
                    body_col,
                );
                draw_circle(x, y - r * 0.2, r * 0.75, body_col);
                // Sementes douradas
                draw_circle(x - r * 0.3, y, 1.0, YELLOW_ACCENT);
                draw_circle(x + r * 0.3, y, 1.0, YELLOW_ACCENT);
                draw_circle(x, y + r * 0.4, 1.0, YELLOW_ACCENT);
                // Folhas verdes
                draw_circle(x - r * 0.4, y - r * 0.8, r * 0.25, EMERALD);
                draw_circle(x, y - r * 0.85, r * 0.25, EMERALD);
                draw_circle(x + r * 0.4, y - r * 0.8, r * 0.25, EMERALD);
            }
            FruitKind::Orange => {
                // Laranja redonda com folha superior
                draw_circle(x, y, r * 0.85, Color::new(0.98, 0.58, 0.10, 1.0));
                draw_circle(x - r * 0.25, y - r * 0.25, r * 0.2, Color::new(1.0, 0.8, 0.4, 0.7));
                draw_circle(x + r * 0.2, y - r * 0.75, r * 0.3, EMERALD);
            }
            FruitKind::Apple => {
                // Maçã vermelha com caule
                draw_circle(x - r * 0.35, y, r * 0.65, Color::new(0.92, 0.18, 0.22, 1.0));
                draw_circle(x + r * 0.35, y, r * 0.65, Color::new(0.92, 0.18, 0.22, 1.0));
                draw_line(x, y - r * 0.4, x, y - r * 0.9, 1.8, Color::new(0.5, 0.3, 0.15, 1.0));
                draw_circle(x + r * 0.3, y - r * 0.7, r * 0.22, EMERALD);
            }
            FruitKind::Key => {
                // Chave cibernética dourada reluzente
                let key_col = YELLOW_ACCENT;
                draw_circle_lines(x, y - r * 0.4, r * 0.45, 2.0, key_col);
                draw_line(x, y - r * 0.2, x, y + r * 0.8, 2.0, key_col);
                draw_line(x, y + r * 0.4, x + r * 0.4, y + r * 0.4, 2.0, key_col);
                draw_line(x, y + r * 0.7, x + r * 0.4, y + r * 0.7, 2.0, key_col);
            }
        }
    }
}

// =========================================================================
// 8. TESTES UNITÁRIOS AUTOMATIZADOS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_opposites_and_angles() {
        assert!(Direction::Up.is_opposite(Direction::Down));
        assert!(Direction::Down.is_opposite(Direction::Up));
        assert!(Direction::Left.is_opposite(Direction::Right));
        assert!(Direction::Right.is_opposite(Direction::Left));
        assert!(!Direction::Up.is_opposite(Direction::Left));
        assert!(!Direction::Right.is_opposite(Direction::Down));
    }

    #[test]
    fn test_difficulty_configurations() {
        let casual = Difficulty::Casual;
        let normal = Difficulty::Normal;
        let hard = Difficulty::Hardcore;

        assert_eq!(casual.starting_lives(), 4);
        assert_eq!(normal.starting_lives(), 3);
        assert_eq!(hard.starting_lives(), 3);

        assert!(casual.ghost_speed() < normal.ghost_speed());
        assert!(normal.ghost_speed() < hard.ghost_speed());

        assert!(casual.frightened_time() > normal.frightened_time());
        assert!(normal.frightened_time() > hard.frightened_time());

        assert_eq!(casual.score_multiplier(), 1.0);
        assert_eq!(normal.score_multiplier(), 1.5);
        assert_eq!(hard.score_multiplier(), 2.5);
    }

    #[test]
    fn test_instant_180_reversal() {
        let mut game = PacmanGame::new();
        game.phase = GamePhase::Playing;
        game.dir = Direction::Right;
        game.next_dir = Direction::Left;

        // Ao rodar a lógica de movimento, o Pacman inverte imediatamente sem precisar de alinhamento
        game.update_pacman_movement(0.016);
        assert_eq!(game.dir, Direction::Left);
    }

    #[test]
    fn test_wall_stopping_at_tile_center() {
        let mut game = PacmanGame::new();
        game.phase = GamePhase::Playing;
        // Posiciona Pacman na coluna 1, linha 1 (onde (0, 1) é parede)
        let cx = PacmanGame::tile_cx(1);
        let cy = PacmanGame::tile_cy(1);
        game.pac_x = cx;
        game.pac_y = cy;
        game.dir = Direction::Left; // Indo em direção à parede esquerda
        game.next_dir = Direction::Left;

        // Avança vários passos
        for _ in 0..10 {
            game.update_pacman_movement(0.016);
        }

        // Deve parar perfeitamente no centro cx, sem ultrapassar para dentro da parede
        assert_eq!(game.pac_x, cx);
    }

    #[test]
    fn test_tunnel_horizontal_wrap() {
        let mut game = PacmanGame::new();
        game.phase = GamePhase::Playing;
        game.pac_y = PacmanGame::tile_cy(10);
        game.pac_x = -TILE_SIZE * 0.6; // Fora do limite esquerdo do túnel
        game.dir = Direction::Left;
        game.next_dir = Direction::Left;

        game.update_pacman_movement(0.016);
        assert!(game.pac_x > 0.0);
    }

    #[test]
    fn test_fruit_progression() {
        assert_eq!(FruitKind::for_stage(1), FruitKind::Cherry);
        assert_eq!(FruitKind::for_stage(1).points(), 100);

        assert_eq!(FruitKind::for_stage(2), FruitKind::Strawberry);
        assert_eq!(FruitKind::for_stage(2).points(), 300);

        assert_eq!(FruitKind::for_stage(3), FruitKind::Orange);
        assert_eq!(FruitKind::for_stage(3).points(), 500);

        assert_eq!(FruitKind::for_stage(4), FruitKind::Apple);
        assert_eq!(FruitKind::for_stage(4).points(), 700);

        assert_eq!(FruitKind::for_stage(5), FruitKind::Key);
        assert_eq!(FruitKind::for_stage(5).points(), 5000);
    }

    #[test]
    fn test_ghost_initialization() {
        let game = PacmanGame::new();
        assert_eq!(game.ghosts.len(), 4);

        assert_eq!(game.ghosts[0].ghost_type, GhostType::Blinky);
        assert_eq!(game.ghosts[1].ghost_type, GhostType::Pinky);
        assert_eq!(game.ghosts[2].ghost_type, GhostType::Inky);
        assert_eq!(game.ghosts[3].ghost_type, GhostType::Clyde);

        // Blinky começa fora da casa
        assert_eq!(game.ghosts[0].state, GhostState::Scatter);
        // Os demais começam dentro da casa
        assert_eq!(game.ghosts[1].state, GhostState::InHouse);
        assert_eq!(game.ghosts[2].state, GhostState::InHouse);
        assert_eq!(game.ghosts[3].state, GhostState::InHouse);
    }
}

