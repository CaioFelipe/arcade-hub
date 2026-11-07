use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

const COLS: usize = 21;
const ROWS: usize = 21;
const TILE_SIZE: f32 = 24.0;

// 1: Parede, 0: Pílula, 2: Energizer (Pílula Grande), 3: Vazio/Casa dos Fantasmas, 4: Portão/Túnel
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
    [1, 1, 1, 1, 0, 1, 3, 1, 1, 3, 3, 3, 1, 1, 3, 1, 0, 1, 1, 1, 1],
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

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_offset(self) -> (i32, i32) {
        match self {
            Direction::None => (0, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn to_angle(self) -> f32 {
        match self {
            Direction::Right => 0.0,
            Direction::Down => std::f32::consts::FRAC_PI_2,
            Direction::Left => std::f32::consts::PI,
            Direction::Up => std::f32::consts::PI * 1.5,
            Direction::None => 0.0,
        }
    }
}

struct Ghost {
    x: f32,
    y: f32,
    dir: Direction,
    color: Color,
    home_x: f32,
    home_y: f32,
    frightened: bool,
}

pub struct PacmanGame {
    pac_x: f32,
    pac_y: f32,
    dir: Direction,
    next_dir: Direction,
    maze: [[u8; COLS]; ROWS],
    ghosts: Vec<Ghost>,
    score: u32,
    lives: i32,
    game_over: bool,
    chomp_angle: f32,
    frightened_timer: f32,
    pellets_left: usize,
    score_popups: Vec<(f32, f32, String, f32)>,
}

impl PacmanGame {
    pub fn new() -> Self {
        let mut pellets = 0;
        for r in 0..ROWS {
            for c in 0..COLS {
                if MAZE_MAP[r][c] == 0 || MAZE_MAP[r][c] == 2 {
                    pellets += 1;
                }
            }
        }

        let ghosts = vec![
            // Blinky (Vermelho)
            Ghost {
                x: 10.0 * TILE_SIZE,
                y: 8.0 * TILE_SIZE,
                dir: Direction::Left,
                color: Color::new(0.95, 0.20, 0.20, 1.0),
                home_x: 10.0 * TILE_SIZE,
                home_y: 8.0 * TILE_SIZE,
                frightened: false,
            },
            // Pinky (Rosa)
            Ghost {
                x: 9.0 * TILE_SIZE,
                y: 10.0 * TILE_SIZE,
                dir: Direction::Up,
                color: Color::new(0.98, 0.60, 0.75, 1.0),
                home_x: 9.0 * TILE_SIZE,
                home_y: 10.0 * TILE_SIZE,
                frightened: false,
            },
            // Inky (Ciano)
            Ghost {
                x: 10.0 * TILE_SIZE,
                y: 10.0 * TILE_SIZE,
                dir: Direction::Up,
                color: Color::new(0.30, 0.85, 0.95, 1.0),
                home_x: 10.0 * TILE_SIZE,
                home_y: 10.0 * TILE_SIZE,
                frightened: false,
            },
            // Clyde (Laranja)
            Ghost {
                x: 11.0 * TILE_SIZE,
                y: 10.0 * TILE_SIZE,
                dir: Direction::Right,
                color: Color::new(0.98, 0.65, 0.15, 1.0),
                home_x: 11.0 * TILE_SIZE,
                home_y: 10.0 * TILE_SIZE,
                frightened: false,
            },
        ];

        Self {
            pac_x: 10.0 * TILE_SIZE,
            pac_y: 16.0 * TILE_SIZE,
            dir: Direction::None,
            next_dir: Direction::None,
            maze: MAZE_MAP,
            ghosts,
            score: 0,
            lives: 3,
            game_over: false,
            chomp_angle: 0.0,
            frightened_timer: 0.0,
            pellets_left: pellets,
            score_popups: Vec::new(),
        }
    }

    fn is_wall(&self, col: i32, row: i32) -> bool {
        if col < 0 || col >= COLS as i32 || row < 0 || row >= ROWS as i32 {
            return false; // Permite túnel lateral
        }
        self.maze[row as usize][col as usize] == 1
    }

    fn can_move_to(&self, x: f32, y: f32, dir: Direction) -> bool {
        let (dx, dy) = dir.to_offset();
        let target_x = x + (dx as f32) * 12.0;
        let target_y = y + (dy as f32) * 12.0;

        let col = (target_x / TILE_SIZE).floor() as i32;
        let row = (target_y / TILE_SIZE).floor() as i32;

        !self.is_wall(col, row)
    }
}

impl ArcadeGame for PacmanGame {
    fn update(&mut self) {
        if self.game_over {
            return;
        }

        let dt = get_frame_time();

        // Temporizador de Fantasmas Assustados (Modo Energizer)
        if self.frightened_timer > 0.0 {
            self.frightened_timer -= dt;
            if self.frightened_timer <= 0.0 {
                for g in &mut self.ghosts {
                    g.frightened = false;
                }
            }
        }

        // Popups de pontuação
        self.score_popups.retain_mut(|p| {
            p.1 -= 0.6;
            p.3 -= dt;
            p.3 > 0.0
        });

        // Entrada do Jogador com Buffer Inteligente de Próxima Direção
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.next_dir = Direction::Up;
        } else if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.next_dir = Direction::Down;
        } else if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.next_dir = Direction::Left;
        } else if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.next_dir = Direction::Right;
        }

        // Alinhamento ao centro do ladrilho para giros precisos
        let current_col = ((self.pac_x + TILE_SIZE * 0.5) / TILE_SIZE).floor() as usize;
        let current_row = ((self.pac_y + TILE_SIZE * 0.5) / TILE_SIZE).floor() as usize;
        let tile_center_x = (current_col as f32) * TILE_SIZE;
        let tile_center_y = (current_row as f32) * TILE_SIZE;

        let dist_to_center = (self.pac_x - tile_center_x).abs() + (self.pac_y - tile_center_y).abs();

        if self.next_dir != Direction::None && (dist_to_center < 6.0 || self.dir == Direction::None) {
            if self.can_move_to(tile_center_x, tile_center_y, self.next_dir) {
                self.dir = self.next_dir;
                self.pac_x = tile_center_x;
                self.pac_y = tile_center_y;
            }
        }

        // Movimento do Pacman
        let speed = 2.8;
        if self.dir != Direction::None && self.can_move_to(self.pac_x, self.pac_y, self.dir) {
            let (dx, dy) = self.dir.to_offset();
            self.pac_x += (dx as f32) * speed;
            self.pac_y += (dy as f32) * speed;
            self.chomp_angle += 0.28;
        }

        // Túnel lateral infinito
        if self.pac_x < -TILE_SIZE * 0.5 {
            self.pac_x = (COLS as f32 - 0.5) * TILE_SIZE;
        } else if self.pac_x > (COLS as f32 - 0.5) * TILE_SIZE {
            self.pac_x = -TILE_SIZE * 0.5;
        }

        // Coleta de pílulas e energizers
        if current_row < ROWS && current_col < COLS {
            match self.maze[current_row][current_col] {
                0 => {
                    self.maze[current_row][current_col] = 3;
                    self.score += 10;
                    self.pellets_left = self.pellets_left.saturating_sub(1);
                }
                2 => {
                    self.maze[current_row][current_col] = 3;
                    self.score += 50;
                    self.pellets_left = self.pellets_left.saturating_sub(1);
                    self.frightened_timer = 7.0; // 7 segundos de fantasmas azuis
                    for g in &mut self.ghosts {
                        g.frightened = true;
                    }
                }
                _ => {}
            }
        }

        // Vitória se comeu todas as pílulas
        if self.pellets_left == 0 {
            self.score += 2000;
            self.game_over = true;
            return;
        }

        // Inteligência Artificial dos 4 Fantasmas
        let ghost_speed = if self.frightened_timer > 0.0 { 1.5 } else { 2.2 };
        let dirs = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

        for g in &mut self.ghosts {
            let g_col = ((g.x + TILE_SIZE * 0.5) / TILE_SIZE).floor() as usize;
            let g_row = ((g.y + TILE_SIZE * 0.5) / TILE_SIZE).floor() as usize;
            let g_center_x = (g_col as f32) * TILE_SIZE;
            let g_center_y = (g_row as f32) * TILE_SIZE;

            let g_dist = (g.x - g_center_x).abs() + (g.y - g_center_y).abs();

            if g_dist < 4.0 {
                // Checa direções possíveis sem dar meia-volta imediata
                let mut valid_dirs = Vec::new();
                for &d in &dirs {
                    let is_opposite = match (g.dir, d) {
                        (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => true,
                        (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => true,
                        _ => false,
                    };

                    let (dx, dy) = d.to_offset();
                    let nc = g_col as i32 + dx;
                    let nr = g_row as i32 + dy;

                    if nc >= 0 && nc < COLS as i32 && nr >= 0 && nr < ROWS as i32 {
                        if MAZE_MAP[nr as usize][nc as usize] != 1 && !is_opposite {
                            valid_dirs.push(d);
                        }
                    }
                }

                if !valid_dirs.is_empty() {
                    if g.frightened {
                        // Fantasma assustado anda aleatório
                        g.dir = valid_dirs[rand::gen_range(0, valid_dirs.len())];
                    } else {
                        // Fantasma persegue Pacman
                        let target_x = self.pac_x;
                        let target_y = self.pac_y;

                        valid_dirs.sort_by(|&a, &b| {
                            let (dxa, dya) = a.to_offset();
                            let (dxb, dyb) = b.to_offset();
                            let d1 = (g.x + dxa as f32 * TILE_SIZE - target_x).powi(2) + (g.y + dya as f32 * TILE_SIZE - target_y).powi(2);
                            let d2 = (g.x + dxb as f32 * TILE_SIZE - target_x).powi(2) + (g.y + dyb as f32 * TILE_SIZE - target_y).powi(2);
                            d1.partial_cmp(&d2).unwrap()
                        });

                        g.dir = valid_dirs[0];
                    }
                }
            }

            let (gdx, gdy) = g.dir.to_offset();
            g.x += (gdx as f32) * ghost_speed;
            g.y += (gdy as f32) * ghost_speed;

            // Colisão com Pacman
            let dist = ((g.x - self.pac_x).powi(2) + (g.y - self.pac_y).powi(2)).sqrt();
            if dist < 18.0 {
                if g.frightened {
                    // Come fantasma
                    self.score += 200;
                    self.score_popups.push((g.x, g.y, "+200".into(), 1.0));
                    g.x = g.home_x;
                    g.y = g.home_y;
                    g.frightened = false;
                } else {
                    // Pacman perde vida
                    self.lives -= 1;
                    self.pac_x = 10.0 * TILE_SIZE;
                    self.pac_y = 16.0 * TILE_SIZE;
                    self.dir = Direction::None;
                    self.next_dir = Direction::None;

                    if self.lives <= 0 {
                        self.game_over = true;
                    }
                }
            }
        }
    }

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let ox = (screen_w - (COLS as f32 * TILE_SIZE)) * 0.5;
        let oy = (screen_h - (ROWS as f32 * TILE_SIZE)) * 0.5 + 32.0;

        // Fundo do palco
        draw_rectangle(ox - 8.0, oy - 8.0, (COLS as f32 * TILE_SIZE) + 16.0, (ROWS as f32 * TILE_SIZE) + 16.0, BG_COLOR);
        draw_rectangle_lines(ox - 8.0, oy - 8.0, (COLS as f32 * TILE_SIZE) + 16.0, (ROWS as f32 * TILE_SIZE) + 16.0, 1.5, BORDER_COLOR);

        // 1. Labirinto Autêntico Estilo Neon Arcade
        for r in 0..ROWS {
            for c in 0..COLS {
                let tx = ox + (c as f32) * TILE_SIZE;
                let ty = oy + (r as f32) * TILE_SIZE;

                match self.maze[r][c] {
                    1 => {
                        // Parede com brilho duplo neon
                        draw_rectangle(tx + 1.0, ty + 1.0, TILE_SIZE - 2.0, TILE_SIZE - 2.0, Color::new(0.06, 0.12, 0.30, 1.0));
                        draw_rectangle_lines(tx + 2.0, ty + 2.0, TILE_SIZE - 4.0, TILE_SIZE - 4.0, 1.5, COBALT_ACCENT);
                        draw_rectangle_lines(tx + 5.0, ty + 5.0, TILE_SIZE - 10.0, TILE_SIZE - 10.0, 1.0, ICE_BLUE);
                    }
                    0 => {
                        // Pílula comum
                        draw_circle(tx + TILE_SIZE * 0.5, ty + TILE_SIZE * 0.5, 3.0, Color::new(0.98, 0.90, 0.70, 0.9));
                    }
                    2 => {
                        // Energizer pulsante
                        let pulse = (get_time() * 6.0).sin().abs() as f32 * 2.0;
                        draw_circle(tx + TILE_SIZE * 0.5, ty + TILE_SIZE * 0.5, 6.5 + pulse, YELLOW_ACCENT);
                        draw_circle(tx + TILE_SIZE * 0.5, ty + TILE_SIZE * 0.5, 3.0, TEXT_PRIMARY);
                    }
                    _ => {}
                }
            }
        }

        // 2. Renderização do Pac-Man com Rotação e Animação de Chomp
        let px = ox + self.pac_x + TILE_SIZE * 0.5;
        let py = oy + self.pac_y + TILE_SIZE * 0.5;
        let mouth = (self.chomp_angle.sin().abs()) * 0.45;
        let angle = self.dir.to_angle();

        draw_circle(px, py, 11.0, YELLOW_ACCENT);

        // Boca cortada no ângulo de movimento
        if mouth > 0.05 {
            let mouth_len = 14.0;
            draw_triangle(
                Vec2::new(px, py),
                Vec2::new(px + (angle - mouth).cos() * mouth_len, py + (angle - mouth).sin() * mouth_len),
                Vec2::new(px + (angle + mouth).cos() * mouth_len, py + (angle + mouth).sin() * mouth_len),
                BG_COLOR,
            );
        }

        // 3. Renderização dos Fantasmas Autênticos (Corpo em Cúpula + Tentáculos + Olhos)
        for g in &self.ghosts {
            let gx = ox + g.x + TILE_SIZE * 0.5;
            let gy = oy + g.y + TILE_SIZE * 0.5;

            let ghost_color = if g.frightened {
                if self.frightened_timer < 2.0 && ((self.frightened_timer * 8.0) as i32 % 2 == 0) {
                    TEXT_PRIMARY // Pisca branco quando o tempo estiver acabando
                } else {
                    Color::new(0.18, 0.35, 0.95, 1.0) // Azul assustado
                }
            } else {
                g.color
            };

            // Cabeça semicircular
            draw_circle(gx, gy - 2.0, 10.0, ghost_color);
            // Saia / Corpo
            draw_rectangle(gx - 10.0, gy - 2.0, 20.0, 9.0, ghost_color);

            // 3 Tentáculos ondulados na base
            draw_triangle(Vec2::new(gx - 10.0, gy + 7.0), Vec2::new(gx - 6.0, gy + 7.0), Vec2::new(gx - 8.0, gy + 11.0), ghost_color);
            draw_triangle(Vec2::new(gx - 4.0, gy + 7.0), Vec2::new(gx + 0.0, gy + 7.0), Vec2::new(gx - 2.0, gy + 11.0), ghost_color);
            draw_triangle(Vec2::new(gx + 2.0, gy + 7.0), Vec2::new(gx + 6.0, gy + 7.0), Vec2::new(gx + 4.0, gy + 11.0), ghost_color);
            draw_triangle(Vec2::new(gx + 6.0, gy + 7.0), Vec2::new(gx + 10.0, gy + 7.0), Vec2::new(gx + 8.0, gy + 11.0), ghost_color);

            // Olhos
            if !g.frightened {
                let (edx, edy) = g.dir.to_offset();
                let ox_eye = (edx as f32) * 2.0;
                let oy_eye = (edy as f32) * 2.0;

                // Esclera branca
                draw_circle(gx - 4.0, gy - 3.0, 3.5, TEXT_PRIMARY);
                draw_circle(gx + 4.0, gy - 3.0, 3.5, TEXT_PRIMARY);

                // Pupila azul olhando na direção
                draw_circle(gx - 4.0 + ox_eye, gy - 3.0 + oy_eye, 1.8, Color::new(0.08, 0.15, 0.45, 1.0));
                draw_circle(gx + 4.0 + ox_eye, gy - 3.0 + oy_eye, 1.8, Color::new(0.08, 0.15, 0.45, 1.0));
            } else {
                // Olhos e boca de medo
                draw_circle(gx - 4.0, gy - 3.0, 1.8, YELLOW_ACCENT);
                draw_circle(gx + 4.0, gy - 3.0, 1.8, YELLOW_ACCENT);
                draw_line(gx - 5.0, gy + 3.0, gx + 5.0, gy + 3.0, 1.5, YELLOW_ACCENT);
            }
        }

        // 4. Popups de pontuação flutuantes
        for (sx, sy, text, _) in &self.score_popups {
            draw_text(text, ox + sx, oy + sy, 18.0, ICE_BLUE);
        }

        // 5. Vidas HUD
        for i in 0..3 {
            let col = if i < self.lives { YELLOW_ACCENT } else { Color::new(0.2, 0.2, 0.3, 0.5) };
            draw_circle(ox + 20.0 + (i as f32) * 24.0, oy + (ROWS as f32 * TILE_SIZE) + 20.0, 7.0, col);
        }

        let hud_info = format!("PÍLULAS: {}  |  FANTASMAS ASSUSTADOS: {:.1}s", self.pellets_left, self.frightened_timer.max(0.0));
        draw_text(&hud_info, ox + 120.0, oy + (ROWS as f32 * TILE_SIZE) + 24.0, 14.0, TEXT_MUTED);
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn get_score(&self) -> u32 {
        self.score
    }

    fn is_game_over(&self) -> bool {
        self.game_over
    }

    fn get_title(&self) -> &str {
        "Pac-Man Classic"
    }
}
