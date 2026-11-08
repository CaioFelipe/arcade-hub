use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

// =========================================================================
// 1. ENUMS E CONFIGURAÇÕES DO JOGO
// =========================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    pub fn name(&self) -> &'static str {
        match self {
            Difficulty::Easy => "FÁCIL",
            Difficulty::Normal => "NORMAL",
            Difficulty::Hard => "DIFÍCIL",
        }
    }

    pub fn lives(&self) -> i32 {
        match self {
            Difficulty::Easy => 5,
            Difficulty::Normal => 3,
            Difficulty::Hard => 3,
        }
    }

    pub fn base_paddle_width(&self) -> f32 {
        match self {
            Difficulty::Easy => 125.0,
            Difficulty::Normal => 100.0,
            Difficulty::Hard => 80.0,
        }
    }

    pub fn base_ball_speed(&self) -> f32 {
        match self {
            Difficulty::Easy => 4.6,
            Difficulty::Normal => 5.6,
            Difficulty::Hard => 6.8,
        }
    }

    pub fn score_multiplier(&self) -> f32 {
        match self {
            Difficulty::Easy => 1.0,
            Difficulty::Normal => 1.5,
            Difficulty::Hard => 2.5,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BrickKind {
    Standard,
    Reinforced,
    Indestructible,
    Explosive,
    PowerUpSpawner,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PowerUpType {
    MultiBall,
    WidePaddle,
    LaserGun,
    FireBall,
    SafetyNet,
    SlowMo,
}

impl PowerUpType {
    pub fn label(&self) -> &'static str {
        match self {
            PowerUpType::MultiBall => "MULTI-BOLA",
            PowerUpType::WidePaddle => "RAQUETE LARGA",
            PowerUpType::LaserGun => "CANHÃO LASER",
            PowerUpType::FireBall => "BOLA DE FOGO",
            PowerUpType::SafetyNet => "ESCUDO DE FUNDO",
            PowerUpType::SlowMo => "CÂMERA LENTA",
        }
    }

    pub fn icon_letter(&self) -> &'static str {
        match self {
            PowerUpType::MultiBall => "M",
            PowerUpType::WidePaddle => "W",
            PowerUpType::LaserGun => "L",
            PowerUpType::FireBall => "F",
            PowerUpType::SafetyNet => "S",
            PowerUpType::SlowMo => "T",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            PowerUpType::MultiBall => YELLOW_ACCENT,
            PowerUpType::WidePaddle => ICE_BLUE,
            PowerUpType::LaserGun => CORAL,
            PowerUpType::FireBall => Color::new(1.0, 0.45, 0.1, 1.0),
            PowerUpType::SafetyNet => EMERALD,
            PowerUpType::SlowMo => Color::new(0.7, 0.4, 0.95, 1.0),
        }
    }
}

#[derive(PartialEq, Debug)]
enum PlayState {
    ReadyToServe,
    Playing,
    LevelClear { bonus: u32, timer: f32 },
    Victory { timer: f32 },
}

enum GameEvent {
    WallSparks { x: f32, y: f32, color: Color },
    PaddleHit { x: f32, y: f32 },
    BrickParticles { x: f32, y: f32, color: Color },
    FloatText { x: f32, y: f32, text: String, color: Color },
    TriggerBomb(usize),
    SpawnPowerUp { x: f32, y: f32 },
    ScreenShake(f32),
    AddScore(u32, f32, f32), // base_points, x, y
}

// =========================================================================
// 2. ESTRUTURAS DE ENTIDADES
// =========================================================================

#[derive(Clone)]
struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    radius: f32,
    trail: Vec<(f32, f32)>,
}

struct Brick {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    kind: BrickKind,
    color: Color,
    hp: i32,
    max_hp: i32,
    points: u32,
    alive: bool,
}

struct PowerUpCapsule {
    x: f32,
    y: f32,
    vy: f32,
    kind: PowerUpType,
    life: f32,
}

struct LaserBeam {
    x: f32,
    y: f32,
    vy: f32,
    alive: bool,
}

struct BreakoutParticle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
    life: f32,
    max_life: f32,
    size: f32,
}

struct FloatText {
    x: f32,
    y: f32,
    text: String,
    color: Color,
    alpha: f32,
}

// =========================================================================
// 3. ESTRUTURA PRINCIPAL DO JOGO
// =========================================================================

pub struct BreakoutGame {
    difficulty: Difficulty,
    state: PlayState,
    current_level: u32,
    paddle_x: f32,
    paddle_w: f32,
    paddle_h: f32,
    balls: Vec<Ball>,
    bricks: Vec<Brick>,
    powerups: Vec<PowerUpCapsule>,
    lasers: Vec<LaserBeam>,
    particles: Vec<BreakoutParticle>,
    float_texts: Vec<FloatText>,
    score: u32,
    lives: i32,
    combo: u32,
    game_over: bool,
    time: f32,
    screen_shake: f32,

    // Timers de Power-ups ativos
    wide_timer: f32,
    laser_timer: f32,
    laser_cooldown: f32,
    fireball_timer: f32,
    safetynet_active: bool,
    safetynet_timer: f32,
    slowmo_timer: f32,
}

impl BreakoutGame {
    pub fn new() -> Self {
        let diff = Difficulty::Normal;
        let mut game = Self {
            difficulty: diff,
            state: PlayState::ReadyToServe,
            current_level: 1,
            paddle_x: 350.0,
            paddle_w: diff.base_paddle_width(),
            paddle_h: 16.0,
            balls: Vec::new(),
            bricks: Vec::new(),
            powerups: Vec::new(),
            lasers: Vec::new(),
            particles: Vec::new(),
            float_texts: Vec::new(),
            score: 0,
            lives: diff.lives(),
            combo: 0,
            game_over: false,
            time: 0.0,
            screen_shake: 0.0,

            wide_timer: 0.0,
            laser_timer: 0.0,
            laser_cooldown: 0.0,
            fireball_timer: 0.0,
            safetynet_active: false,
            safetynet_timer: 0.0,
            slowmo_timer: 0.0,
        };

        game.load_level(1);
        game.attach_ball_to_paddle();
        game
    }

    fn attach_ball_to_paddle(&mut self) {
        self.balls.clear();
        let bx = self.paddle_x + self.paddle_w * 0.5;
        let by = 538.0 - 9.0;
        self.balls.push(Ball {
            x: bx,
            y: by,
            vx: 0.0,
            vy: 0.0,
            radius: 7.0,
            trail: Vec::new(),
        });
        self.state = PlayState::ReadyToServe;
    }

    fn serve_ball(&mut self) {
        if self.balls.is_empty() {
            return;
        }
        let speed = self.difficulty.base_ball_speed();
        // Leve oscilação angular para o saque inicial não ser estritamente reto
        let angle_offset = (self.time * 2.8).sin() * 0.40;
        let vx = speed * angle_offset.sin();
        let vy = -speed * angle_offset.cos();

        self.balls[0].vx = vx;
        self.balls[0].vy = if vy > -2.5 { -speed } else { vy };
        self.state = PlayState::Playing;

        // Efeito de partida
        for _ in 0..14 {
            self.particles.push(BreakoutParticle {
                x: self.balls[0].x,
                y: self.balls[0].y,
                vx: rand::gen_range(-2.5, 2.5),
                vy: rand::gen_range(-3.5, 0.5),
                color: ICE_BLUE,
                life: 0.5,
                max_life: 0.5,
                size: 3.5,
            });
        }
    }

    fn set_difficulty(&mut self, diff: Difficulty) {
        self.difficulty = diff;
        self.lives = diff.lives();
        self.paddle_w = diff.base_paddle_width();
        self.attach_ball_to_paddle();
        self.float_texts.push(FloatText {
            x: 400.0,
            y: 470.0,
            text: format!("DIFICULDADE: {}", diff.name()),
            color: YELLOW_ACCENT,
            alpha: 1.0,
        });
    }

    // =========================================================================
    // 4. GERAÇÃO DE FASES HANDCRAFTED
    // =========================================================================

    fn load_level(&mut self, level: u32) {
        self.current_level = level;
        self.bricks.clear();
        self.powerups.clear();
        self.lasers.clear();
        self.combo = 0;

        let bw = 66.0;
        let bh = 20.0;
        let pad_x = 7.0;
        let pad_y = 7.0;
        let ox = 42.0;
        let oy = 68.0;

        match level {
            1 => {
                // Fase 1: Prisma Inicial (Grade Clássica com introdução fluida)
                let row_data = [
                    (CORAL, 50),
                    (YELLOW_ACCENT, 40),
                    (EMERALD, 30),
                    (ICE_BLUE, 20),
                    (COBALT_ACCENT, 10),
                ];
                for r in 0..5 {
                    let (col, pts) = row_data[r];
                    for c in 0..10 {
                        let is_powerup = (r == 2 && (c == 3 || c == 6)) || (r == 0 && c == 4);
                        let kind = if is_powerup {
                            BrickKind::PowerUpSpawner
                        } else {
                            BrickKind::Standard
                        };

                        self.bricks.push(Brick {
                            x: ox + (c as f32) * (bw + pad_x),
                            y: oy + (r as f32) * (bh + pad_y),
                            w: bw,
                            h: bh,
                            kind,
                            color: if is_powerup { Color::new(0.9, 0.4, 0.95, 1.0) } else { col },
                            hp: 1,
                            max_hp: 1,
                            points: pts,
                            alive: true,
                        });
                    }
                }
            }
            2 => {
                // Fase 2: Pirâmide de Diamante (Geometria Escalonada com Núcleo Reforçado)
                let diamond_rows = [
                    (4..=5, CORAL, 60),
                    (3..=6, YELLOW_ACCENT, 50),
                    (2..=7, EMERALD, 40),
                    (1..=8, ICE_BLUE, 30),
                    (0..=9, COBALT_ACCENT, 20),
                    (1..=8, ICE_BLUE, 30),
                    (2..=7, EMERALD, 40),
                ];

                for (r, (cols, color, pts)) in diamond_rows.iter().enumerate() {
                    for c in cols.clone() {
                        let is_core = (r >= 2 && r <= 4) && (c >= 4 && c <= 5);
                        let is_powerup = (r == 0 && c == 4) || (r == 3 && (c == 1 || c == 8));

                        let (kind, hp, clr) = if is_core {
                            (BrickKind::Reinforced, 2, Color::new(0.98, 0.82, 0.2, 1.0))
                        } else if is_powerup {
                            (BrickKind::PowerUpSpawner, 1, Color::new(0.9, 0.4, 0.95, 1.0))
                        } else {
                            (BrickKind::Standard, 1, *color)
                        };

                        self.bricks.push(Brick {
                            x: ox + (c as f32) * (bw + pad_x),
                            y: oy + (r as f32) * (bh + pad_y),
                            w: bw,
                            h: bh,
                            kind,
                            color: clr,
                            hp,
                            max_hp: hp,
                            points: *pts,
                            alive: true,
                        });
                    }
                }
            }
            3 => {
                // Fase 3: Fortaleza Blindada (Colunas de Titânio + Bombas Neon Estratégicas)
                for r in 0..6 {
                    for c in 0..10 {
                        let is_pillar = (c == 0 || c == 9) && (r >= 1 && r <= 4);
                        let is_bomb = (r == 2 && (c == 3 || c == 6)) || (r == 4 && c == 4);
                        let is_reinforced = (r == 1 && (c == 4 || c == 5)) || (r == 3 && (c == 2 || c == 7));

                        let (kind, hp, color, pts) = if is_pillar {
                            (BrickKind::Indestructible, 999, Color::new(0.35, 0.42, 0.52, 1.0), 0)
                        } else if is_bomb {
                            (BrickKind::Explosive, 1, Color::new(0.95, 0.25, 0.15, 1.0), 80)
                        } else if is_reinforced {
                            (BrickKind::Reinforced, 2, ICE_BLUE, 70)
                        } else {
                            let col = match r % 4 {
                                0 => CORAL,
                                1 => EMERALD,
                                2 => YELLOW_ACCENT,
                                _ => COBALT_ACCENT,
                            };
                            (BrickKind::Standard, 1, col, 35)
                        };

                        self.bricks.push(Brick {
                            x: ox + (c as f32) * (bw + pad_x),
                            y: oy + (r as f32) * (bh + pad_y),
                            w: bw,
                            h: bh,
                            kind,
                            color,
                            hp,
                            max_hp: hp,
                            points: pts,
                            alive: true,
                        });
                    }
                }
            }
            4 => {
                // Fase 4: Ondas Cósmicas (Padrão Senoidal, Alta Densidade de Itens e Bombas)
                for r in 0..7 {
                    for c in 0..10 {
                        let wave_active = (c + r) % 2 == 0;
                        if !wave_active {
                            continue;
                        }

                        let is_bomb = (r == 3 && c == 5) || (r == 1 && c == 3);
                        let is_powerup = (r == 5 && (c == 1 || c == 9)) || (r == 1 && c == 7);
                        let is_reinforced = r == 0 || r == 6;

                        let (kind, hp, color, pts) = if is_bomb {
                            (BrickKind::Explosive, 1, Color::new(0.95, 0.25, 0.15, 1.0), 100)
                        } else if is_powerup {
                            (BrickKind::PowerUpSpawner, 1, Color::new(0.9, 0.4, 0.95, 1.0), 75)
                        } else if is_reinforced {
                            (BrickKind::Reinforced, 2, EMERALD, 80)
                        } else {
                            let col = match c % 3 {
                                0 => ICE_BLUE,
                                1 => YELLOW_ACCENT,
                                _ => CORAL,
                            };
                            (BrickKind::Standard, 1, col, 45)
                        };

                        self.bricks.push(Brick {
                            x: ox + (c as f32) * (bw + pad_x),
                            y: oy + (r as f32) * (bh + pad_y),
                            w: bw,
                            h: bh,
                            kind,
                            color,
                            hp,
                            max_hp: hp,
                            points: pts,
                            alive: true,
                        });
                    }
                }
            }
            _ => {
                // Fase 5: Núcleo Prismático Supremo (Boss Stage - Fortificação de Cristal)
                for r in 0..8 {
                    for c in 0..10 {
                        // Pilares de titânio nas quinas
                        let is_titanium = ((c == 1 || c == 8) && (r == 1 || r == 6))
                            || ((c == 0 || c == 9) && (r == 3 || r == 4));

                        // Núcleo pesado de 3 HP
                        let is_heavy_core = (r >= 3 && r <= 4) && (c >= 4 && c <= 5);

                        // Bombas no anel intermediário
                        let is_bomb = (r == 2 && c == 4) || (r == 5 && c == 5) || (r == 3 && c == 2) || (r == 4 && c == 7);

                        // Cristais de power-up
                        let is_powerup = (r == 0 && (c == 3 || c == 6)) || (r == 7 && (c == 2 || c == 7));

                        let (kind, hp, color, pts) = if is_titanium {
                            (BrickKind::Indestructible, 999, Color::new(0.35, 0.42, 0.52, 1.0), 0)
                        } else if is_heavy_core {
                            (BrickKind::Reinforced, 3, Color::new(0.98, 0.35, 0.45, 1.0), 150)
                        } else if is_bomb {
                            (BrickKind::Explosive, 1, Color::new(0.95, 0.25, 0.15, 1.0), 120)
                        } else if is_powerup {
                            (BrickKind::PowerUpSpawner, 1, Color::new(0.9, 0.4, 0.95, 1.0), 100)
                        } else {
                            let hp_val = if r % 2 == 1 { 2 } else { 1 };
                            let col = if hp_val == 2 { ICE_BLUE } else { YELLOW_ACCENT };
                            (BrickKind::Reinforced, hp_val, col, 60)
                        };

                        self.bricks.push(Brick {
                            x: ox + (c as f32) * (bw + pad_x),
                            y: oy + (r as f32) * (bh + pad_y),
                            w: bw,
                            h: bh,
                            kind,
                            color,
                            hp,
                            max_hp: hp,
                            points: pts,
                            alive: true,
                        });
                    }
                }
            }
        }
    }

    // =========================================================================
    // 5. ATIVAÇÃO DE POWER-UPS & ITENS
    // =========================================================================

    fn spawn_powerup_capsule(&mut self, x: f32, y: f32, forced_kind: Option<PowerUpType>) {
        let kind = if let Some(k) = forced_kind {
            k
        } else {
            let roll = rand::gen_range(0, 100);
            if roll < 22 {
                PowerUpType::MultiBall
            } else if roll < 42 {
                PowerUpType::WidePaddle
            } else if roll < 60 {
                PowerUpType::LaserGun
            } else if roll < 75 {
                PowerUpType::FireBall
            } else if roll < 88 {
                PowerUpType::SafetyNet
            } else {
                PowerUpType::SlowMo
            }
        };

        self.powerups.push(PowerUpCapsule {
            x,
            y,
            vy: 2.2,
            kind,
            life: 0.0,
        });
    }

    fn apply_powerup(&mut self, kind: PowerUpType) {
        match kind {
            PowerUpType::MultiBall => {
                let mut new_balls = Vec::new();
                for b in &self.balls {
                    new_balls.push(Ball {
                        x: b.x,
                        y: b.y,
                        vx: b.vx * 0.8 - b.vy * 0.4,
                        vy: b.vy * 0.8 + b.vx * 0.4,
                        radius: b.radius,
                        trail: Vec::new(),
                    });
                    new_balls.push(Ball {
                        x: b.x,
                        y: b.y,
                        vx: b.vx * 0.8 + b.vy * 0.4,
                        vy: b.vy * 0.8 - b.vx * 0.4,
                        radius: b.radius,
                        trail: Vec::new(),
                    });
                }
                for mut nb in new_balls {
                    let cur_spd = (nb.vx * nb.vx + nb.vy * nb.vy).sqrt();
                    let target_spd = self.difficulty.base_ball_speed();
                    if cur_spd > 0.001 {
                        nb.vx = (nb.vx / cur_spd) * target_spd;
                        nb.vy = (nb.vy / cur_spd) * target_spd;
                        self.balls.push(nb);
                    }
                }
                self.float_texts.push(FloatText {
                    x: self.paddle_x + self.paddle_w * 0.5,
                    y: 500.0,
                    text: "+MULTI-BOLA!".to_string(),
                    color: YELLOW_ACCENT,
                    alpha: 1.0,
                });
            }
            PowerUpType::WidePaddle => {
                self.wide_timer = 12.0;
                self.paddle_w = self.difficulty.base_paddle_width() + 45.0;
                self.float_texts.push(FloatText {
                    x: self.paddle_x + self.paddle_w * 0.5,
                    y: 500.0,
                    text: "+RAQUETE LARGA!".to_string(),
                    color: ICE_BLUE,
                    alpha: 1.0,
                });
            }
            PowerUpType::LaserGun => {
                self.laser_timer = 10.0;
                self.float_texts.push(FloatText {
                    x: self.paddle_x + self.paddle_w * 0.5,
                    y: 500.0,
                    text: "+CANHÃO LASER (Espaço/Clique)!".to_string(),
                    color: CORAL,
                    alpha: 1.0,
                });
            }
            PowerUpType::FireBall => {
                self.fireball_timer = 8.5;
                self.float_texts.push(FloatText {
                    x: self.paddle_x + self.paddle_w * 0.5,
                    y: 500.0,
                    text: "+BOLA DE FOGO PRISMÁTICA!".to_string(),
                    color: Color::new(1.0, 0.45, 0.1, 1.0),
                    alpha: 1.0,
                });
            }
            PowerUpType::SafetyNet => {
                self.safetynet_active = true;
                self.safetynet_timer = 16.0;
                self.float_texts.push(FloatText {
                    x: self.paddle_x + self.paddle_w * 0.5,
                    y: 500.0,
                    text: "+ESCUDO DE ENERGIA ATIVADO!".to_string(),
                    color: EMERALD,
                    alpha: 1.0,
                });
            }
            PowerUpType::SlowMo => {
                self.slowmo_timer = 8.0;
                for b in &mut self.balls {
                    b.vx *= 0.65;
                    b.vy *= 0.65;
                }
                self.float_texts.push(FloatText {
                    x: self.paddle_x + self.paddle_w * 0.5,
                    y: 500.0,
                    text: "+CÂMERA LENTA!".to_string(),
                    color: Color::new(0.7, 0.4, 0.95, 1.0),
                    alpha: 1.0,
                });
            }
        }
    }

    fn trigger_bomb_explosion(&mut self, initial_bomb: usize) {
        let mut bomb_queue = vec![initial_bomb];
        self.screen_shake = 8.0;

        while let Some(b_idx) = bomb_queue.pop() {
            if b_idx >= self.bricks.len() {
                continue;
            }
            let bx = self.bricks[b_idx].x + self.bricks[b_idx].w * 0.5;
            let by = self.bricks[b_idx].y + self.bricks[b_idx].h * 0.5;

            // Partículas da explosão
            for _ in 0..28 {
                let spd = rand::gen_range(2.0, 6.5);
                let ang: f32 = rand::gen_range(0.0, 6.28);
                self.particles.push(BreakoutParticle {
                    x: bx,
                    y: by,
                    vx: ang.cos() * spd,
                    vy: ang.sin() * spd,
                    color: if rand::gen_range(0, 2) == 0 { CORAL } else { YELLOW_ACCENT },
                    life: 0.75,
                    max_life: 0.75,
                    size: rand::gen_range(3.0, 6.0),
                });
            }

            let radius_sq = 95.0 * 95.0;
            let score_mult = self.difficulty.score_multiplier();

            for (i, b) in self.bricks.iter_mut().enumerate() {
                if !b.alive || i == b_idx {
                    continue;
                }
                if b.kind == BrickKind::Indestructible {
                    continue;
                }

                let cx = b.x + b.w * 0.5;
                let cy = b.y + b.h * 0.5;
                let dist_sq = (cx - bx) * (cx - bx) + (cy - by) * (cy - by);

                if dist_sq <= radius_sq {
                    if b.kind == BrickKind::Explosive {
                        b.alive = false;
                        bomb_queue.push(i);
                    } else {
                        b.hp -= 2;
                        if b.hp <= 0 {
                            b.alive = false;
                            self.score += (b.points as f32 * score_mult) as u32;
                        }
                    }
                }
            }
        }
    }

    fn fire_lasers(&mut self) {
        if self.laser_cooldown > 0.0 {
            return;
        }
        self.laser_cooldown = 0.18;
        let ly = 538.0 - 6.0;
        self.lasers.push(LaserBeam {
            x: self.paddle_x + 8.0,
            y: ly,
            vy: -15.0,
            alive: true,
        });
        self.lasers.push(LaserBeam {
            x: self.paddle_x + self.paddle_w - 8.0,
            y: ly,
            vy: -15.0,
            alive: true,
        });
    }

    fn spawn_wall_sparks(&mut self, x: f32, y: f32, col: Color) {
        for _ in 0..8 {
            self.particles.push(BreakoutParticle {
                x,
                y,
                vx: rand::gen_range(-2.8, 2.8),
                vy: rand::gen_range(-2.8, 2.8),
                color: col,
                life: 0.40,
                max_life: 0.40,
                size: 3.0,
            });
        }
    }
}

// =========================================================================
// 6. IMPLEMENTAÇÃO DO TRAIT ARCADETRAIT
// =========================================================================

impl ArcadeGame for BreakoutGame {
    fn update(&mut self) {
        let dt = get_frame_time();
        self.time += dt;

        // Amortecimento do tremor de tela
        if self.screen_shake > 0.0 {
            self.screen_shake = (self.screen_shake - dt * 16.0).max(0.0);
        }

        // Atualização dos temporizadores de power-ups
        if self.wide_timer > 0.0 {
            self.wide_timer -= dt;
            if self.wide_timer <= 0.0 {
                self.paddle_w = self.difficulty.base_paddle_width();
            }
        }
        if self.laser_timer > 0.0 {
            self.laser_timer -= dt;
        }
        if self.laser_cooldown > 0.0 {
            self.laser_cooldown -= dt;
        }
        if self.fireball_timer > 0.0 {
            self.fireball_timer -= dt;
        }
        if self.safetynet_active {
            self.safetynet_timer -= dt;
            if self.safetynet_timer <= 0.0 {
                self.safetynet_active = false;
            }
        }
        if self.slowmo_timer > 0.0 {
            self.slowmo_timer -= dt;
        }

        // Atualiza partículas
        self.particles.retain_mut(|p| {
            p.x += p.vx;
            p.y += p.vy;
            p.life -= dt;
            p.life > 0.0
        });

        // Atualiza textos flutuantes
        self.float_texts.retain_mut(|ft| {
            ft.y -= 0.8;
            ft.alpha -= dt * 1.3;
            ft.alpha > 0.0
        });

        // Atualiza cápsulas de itens caindo
        self.powerups.retain_mut(|pup| {
            pup.y += pup.vy;
            pup.life += dt;
            pup.y < 620.0
        });

        // Atualiza projéteis laser
        self.lasers.retain_mut(|l| {
            l.y += l.vy;
            l.alive && l.y > 25.0
        });

        // Tratamento dos estados do jogo
        match &mut self.state {
            PlayState::LevelClear { bonus: _, timer } => {
                *timer += dt;
                // Disparo de fogos de comemoração
                if rand::gen_range(0, 10) == 0 {
                    self.particles.push(BreakoutParticle {
                        x: rand::gen_range(100.0, 700.0),
                        y: rand::gen_range(100.0, 350.0),
                        vx: rand::gen_range(-2.0, 2.0),
                        vy: rand::gen_range(-2.0, 2.0),
                        color: YELLOW_ACCENT,
                        life: 0.6,
                        max_life: 0.6,
                        size: 4.0,
                    });
                }
                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) || is_mouse_button_pressed(MouseButton::Left) {
                    let next_lvl = self.current_level + 1;
                    if next_lvl <= 5 {
                        self.load_level(next_lvl);
                        self.attach_ball_to_paddle();
                    } else {
                        self.state = PlayState::Victory { timer: 0.0 };
                    }
                }
                return;
            }
            PlayState::Victory { timer } => {
                *timer += dt;
                // Fogos contínuos prismáticos
                if rand::gen_range(0, 6) == 0 {
                    let cols = [CORAL, YELLOW_ACCENT, EMERALD, ICE_BLUE, Color::new(0.9, 0.4, 0.95, 1.0)];
                    let c = cols[rand::gen_range(0, cols.len())];
                    self.particles.push(BreakoutParticle {
                        x: rand::gen_range(80.0, 720.0),
                        y: rand::gen_range(80.0, 400.0),
                        vx: rand::gen_range(-3.0, 3.0),
                        vy: rand::gen_range(-3.0, 3.0),
                        color: c,
                        life: 0.8,
                        max_life: 0.8,
                        size: 4.5,
                    });
                }
                if is_key_pressed(KeyCode::R) {
                    *self = Self::new();
                }
                return;
            }
            _ => {}
        }

        if self.game_over {
            return;
        }

        // =====================================================================
        // CONTROLE DA RAQUETE (Híbrido Teclado + Mouse Suave)
        // =====================================================================
        let speed = 9.8;
        let mut moved_keyboard = false;

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.paddle_x = (self.paddle_x - speed).max(42.0);
            moved_keyboard = true;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.paddle_x = (self.paddle_x + speed).min(758.0 - self.paddle_w);
            moved_keyboard = true;
        }

        // Se o mouse estiver ativo dentro da arena e o jogador não estiver usando teclado
        if !moved_keyboard {
            let (mx, my) = mouse_position();
            let screen_w = screen_width();
            let screen_h = screen_height();
            let ox = (screen_w - 800.0) * 0.5;
            let oy = (screen_h - 600.0) * 0.5 + 24.0;
            let local_mx = mx - ox;
            let local_my = my - oy;

            if local_mx >= 0.0 && local_mx <= 800.0 && local_my >= 40.0 && local_my <= 620.0 {
                let target_x = (local_mx - self.paddle_w * 0.5).clamp(42.0, 758.0 - self.paddle_w);
                self.paddle_x += (target_x - self.paddle_x) * 0.35;
            }
        }

        // Seletor de Dificuldade antes do primeiro saque
        if self.state == PlayState::ReadyToServe && self.current_level == 1 && self.score == 0 {
            if is_key_pressed(KeyCode::Key1) {
                self.set_difficulty(Difficulty::Easy);
            } else if is_key_pressed(KeyCode::Key2) {
                self.set_difficulty(Difficulty::Normal);
            } else if is_key_pressed(KeyCode::Key3) {
                self.set_difficulty(Difficulty::Hard);
            }
        }

        // Disparo do Canhão Laser
        if self.laser_timer > 0.0 {
            if is_key_down(KeyCode::Space) || is_mouse_button_down(MouseButton::Left) {
                self.fire_lasers();
            }
        }

        // Lançamento da bola no estado ReadyToServe
        if self.state == PlayState::ReadyToServe {
            if !self.balls.is_empty() {
                self.balls[0].x = self.paddle_x + self.paddle_w * 0.5;
                self.balls[0].y = 538.0 - 9.0;
            }

            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up) || is_mouse_button_pressed(MouseButton::Left) {
                self.serve_ball();
            }
            return;
        }

        // Buffer de eventos para desacoplar borrows
        let mut events = Vec::new();

        // =====================================================================
        // FÍSICA E COLISÕES DOS LASERS
        // =====================================================================
        for laser in &mut self.lasers {
            if !laser.alive {
                continue;
            }
            for (b_idx, brick) in self.bricks.iter_mut().enumerate() {
                if !brick.alive {
                    continue;
                }
                if laser.x >= brick.x && laser.x <= brick.x + brick.w &&
                   laser.y >= brick.y && laser.y <= brick.y + brick.h
                {
                    laser.alive = false;
                    if brick.kind == BrickKind::Indestructible {
                        events.push(GameEvent::WallSparks { x: laser.x, y: laser.y, color: ICE_BLUE });
                    } else if brick.kind == BrickKind::Explosive {
                        brick.alive = false;
                        events.push(GameEvent::TriggerBomb(b_idx));
                    } else {
                        brick.hp -= 1;
                        if brick.hp <= 0 {
                            brick.alive = false;
                            let bx = brick.x + brick.w * 0.5;
                            let by = brick.y + brick.h * 0.5;
                            let col = brick.color;
                            let kind = brick.kind;
                            let pts = brick.points;

                            events.push(GameEvent::AddScore(pts, bx, by));
                            events.push(GameEvent::BrickParticles { x: bx, y: by, color: col });
                            if kind == BrickKind::PowerUpSpawner || rand::gen_range(0, 100) < 15 {
                                events.push(GameEvent::SpawnPowerUp { x: bx, y: by });
                            }
                        }
                    }
                    break;
                }
            }
        }

        // =====================================================================
        // COLISÃO DAS CÁPSULAS DE POWER-UP COM A RAQUETE
        // =====================================================================
        let pw = self.paddle_w;
        let ph = self.paddle_h;
        let px = self.paddle_x;
        let py = 538.0;

        let mut collected_pups = Vec::new();
        for (idx, pup) in self.powerups.iter_mut().enumerate() {
            if pup.y >= py - 8.0 && pup.y <= py + ph + 8.0 &&
               pup.x >= px - 10.0 && pup.x <= px + pw + 10.0
            {
                collected_pups.push((idx, pup.kind));
            }
        }

        // Aplica itens coletados de trás para frente
        for (idx, kind) in collected_pups.into_iter().rev() {
            self.powerups.remove(idx);
            self.apply_powerup(kind);
            self.score += (50.0 * self.difficulty.score_multiplier()) as u32;
        }

        // =====================================================================
        // FÍSICA DAS BOLAS E COLISÕES CIRCULO-AABB
        // =====================================================================
        let is_fireball = self.fireball_timer > 0.0;
        let safetynet = self.safetynet_active;
        let mut dropped_balls = Vec::new();

        for (ball_idx, ball) in self.balls.iter_mut().enumerate() {
            // Histórico de rastro da bola
            ball.trail.push((ball.x, ball.y));
            if ball.trail.len() > 10 {
                ball.trail.remove(0);
            }

            ball.x += ball.vx;
            ball.y += ball.vy;

            // Parede Esquerda
            if ball.x <= 46.0 {
                ball.x = 46.0;
                ball.vx = ball.vx.abs();
                events.push(GameEvent::WallSparks { x: ball.x, y: ball.y, color: ICE_BLUE });
            }
            // Parede Direita
            else if ball.x >= 754.0 {
                ball.x = 754.0;
                ball.vx = -ball.vx.abs();
                events.push(GameEvent::WallSparks { x: ball.x, y: ball.y, color: ICE_BLUE });
            }

            // Teto
            if ball.y <= 32.0 {
                ball.y = 32.0;
                ball.vy = ball.vy.abs();
                events.push(GameEvent::WallSparks { x: ball.x, y: ball.y, color: ICE_BLUE });
            }

            // Escudo de Fundo (Safety Net)
            if safetynet && ball.y >= 570.0 && ball.vy > 0.0 {
                ball.vy = -ball.vy.abs();
                events.push(GameEvent::ScreenShake(3.0));
                events.push(GameEvent::WallSparks { x: ball.x, y: 570.0, color: EMERALD });
                events.push(GameEvent::FloatText {
                    x: ball.x,
                    y: 560.0,
                    text: "REBOTE DO ESCUDO!".to_string(),
                    color: EMERALD,
                });
            }

            // Colisão com a Raquete (Física angular baseada na zona de impacto)
            if ball.y >= py - 8.0 && ball.y <= py + ph &&
               ball.x >= px - 6.0 && ball.x <= px + pw + 6.0 &&
               ball.vy > 0.0
            {
                ball.vy = -ball.vy.abs();
                let offset = (ball.x - (px + pw * 0.5)) / (pw * 0.5);
                let max_h_speed = 8.5;
                ball.vx = (offset * max_h_speed).clamp(-max_h_speed, max_h_speed);

                // Garante velocidade vertical mínima
                if ball.vy.abs() < 3.5 {
                    ball.vy = -3.5;
                }

                events.push(GameEvent::PaddleHit { x: ball.x, y: py });
            }

            // =================================================================
            // COLISÃO PRECISA CIRCULO-AABB COM OS TIJOLOS
            // =================================================================
            let mut hit_idx: Option<usize> = None;
            let mut hit_normal = (0.0f32, 0.0f32);

            for (i, brick) in self.bricks.iter().enumerate() {
                if !brick.alive {
                    continue;
                }

                // Ponto mais próximo no retângulo
                let closest_x = ball.x.clamp(brick.x, brick.x + brick.w);
                let closest_y = ball.y.clamp(brick.y, brick.y + brick.h);

                let dx = ball.x - closest_x;
                let dy = ball.y - closest_y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq < (ball.radius * ball.radius) {
                    hit_idx = Some(i);

                    // Determina a face atingida comparando a penetração
                    let overlap_x = (ball.radius - dx.abs()).max(0.0);
                    let overlap_y = (ball.radius - dy.abs()).max(0.0);

                    if overlap_x < overlap_y {
                        hit_normal = (if dx > 0.0 { 1.0 } else { -1.0 }, 0.0);
                    } else {
                        hit_normal = (0.0, if dy > 0.0 { 1.0 } else { -1.0 });
                    }
                    break;
                }
            }

            if let Some(idx) = hit_idx {
                let brick = &mut self.bricks[idx];

                if brick.kind == BrickKind::Indestructible {
                    // Ricochete imediato com faíscas metálicas
                    if hit_normal.0 != 0.0 {
                        ball.vx = hit_normal.0 * ball.vx.abs();
                    }
                    if hit_normal.1 != 0.0 {
                        ball.vy = hit_normal.1 * ball.vy.abs();
                    }
                    events.push(GameEvent::WallSparks { x: ball.x, y: ball.y, color: Color::new(0.6, 0.7, 0.85, 1.0) });
                    events.push(GameEvent::ScreenShake(2.0));
                } else if brick.kind == BrickKind::Explosive {
                    brick.alive = false;
                    events.push(GameEvent::TriggerBomb(idx));
                } else {
                    // Tijolo destrutível
                    if !is_fireball {
                        if hit_normal.0 != 0.0 {
                            ball.vx = hit_normal.0 * ball.vx.abs();
                        }
                        if hit_normal.1 != 0.0 {
                            ball.vy = hit_normal.1 * ball.vy.abs();
                        }
                    }

                    brick.hp -= 1;
                    if brick.hp <= 0 {
                        brick.alive = false;
                        let bx = brick.x + brick.w * 0.5;
                        let by = brick.y + brick.h * 0.5;
                        let col = brick.color;
                        let kind = brick.kind;
                        let pts = brick.points;

                        events.push(GameEvent::AddScore(pts, bx, by));
                        events.push(GameEvent::BrickParticles { x: bx, y: by, color: col });

                        // Chance de soltar Power-Up
                        if kind == BrickKind::PowerUpSpawner || rand::gen_range(0, 100) < 16 {
                            events.push(GameEvent::SpawnPowerUp { x: bx, y: by });
                        }
                    } else {
                        // Apenas rachou
                        let bx = brick.x + brick.w * 0.5;
                        let by = brick.y + brick.h * 0.5;
                        events.push(GameEvent::FloatText {
                            x: bx,
                            y: by,
                            text: "RACHOU!".to_string(),
                            color: ICE_BLUE,
                        });
                        events.push(GameEvent::WallSparks { x: ball.x, y: ball.y, color: brick.color });
                    }
                }
            }

            // Queda da bola para fora da arena
            if ball.y > 615.0 {
                dropped_balls.push(ball_idx);
            }
        }

        // Processa todos os eventos coletados
        for ev in events {
            match ev {
                GameEvent::WallSparks { x, y, color } => {
                    self.spawn_wall_sparks(x, y, color);
                }
                GameEvent::PaddleHit { x, y } => {
                    self.combo = 0;
                    for _ in 0..12 {
                        self.particles.push(BreakoutParticle {
                            x,
                            y,
                            vx: rand::gen_range(-3.0, 3.0),
                            vy: rand::gen_range(-4.0, -1.0),
                            color: ICE_BLUE,
                            life: 0.5,
                            max_life: 0.5,
                            size: 3.5,
                        });
                    }
                }
                GameEvent::BrickParticles { x, y, color } => {
                    for _ in 0..18 {
                        self.particles.push(BreakoutParticle {
                            x,
                            y,
                            vx: rand::gen_range(-4.5, 4.5),
                            vy: rand::gen_range(-4.5, 4.5),
                            color,
                            life: 0.65,
                            max_life: 0.65,
                            size: rand::gen_range(2.5, 5.0),
                        });
                    }
                }
                GameEvent::FloatText { x, y, text, color } => {
                    self.float_texts.push(FloatText {
                        x,
                        y,
                        text,
                        color,
                        alpha: 1.0,
                    });
                }
                GameEvent::TriggerBomb(idx) => {
                    self.trigger_bomb_explosion(idx);
                }
                GameEvent::SpawnPowerUp { x, y } => {
                    self.spawn_powerup_capsule(x, y, None);
                }
                GameEvent::ScreenShake(amt) => {
                    self.screen_shake = self.screen_shake.max(amt);
                }
                GameEvent::AddScore(base_pts, bx, by) => {
                    self.combo += 1;
                    let combo_mult = 1.0 + (self.combo.saturating_sub(1) as f32) * 0.25;
                    let earned_pts = (base_pts as f32 * self.difficulty.score_multiplier() * combo_mult) as u32;
                    self.score += earned_pts;

                    let ft_text = if self.combo > 2 {
                        format!("+{} (x{})", earned_pts, self.combo)
                    } else {
                        format!("+{}", earned_pts)
                    };
                    self.float_texts.push(FloatText {
                        x: bx,
                        y: by,
                        text: ft_text,
                        color: if self.combo > 2 { YELLOW_ACCENT } else { TEXT_PRIMARY },
                        alpha: 1.0,
                    });
                }
            }
        }

        // Remove bolas que caíram
        for idx in dropped_balls.into_iter().rev() {
            self.balls.remove(idx);
        }

        // Se todas as bolas foram perdidas
        if self.balls.is_empty() {
            self.lives -= 1;
            self.screen_shake = 6.0;
            self.combo = 0;

            if self.lives <= 0 {
                self.game_over = true;
            } else {
                self.attach_ball_to_paddle();
            }
        }

        // Checagem de conclusão da fase: todos os tijolos quebráveis foram destruídos?
        let remaining_destructible = self.bricks.iter().any(|b| b.alive && b.kind != BrickKind::Indestructible);
        if !remaining_destructible {
            let bonus = (self.lives as u32) * 400 + self.current_level * 500;
            self.score += (bonus as f32 * self.difficulty.score_multiplier()) as u32;
            self.state = PlayState::LevelClear { bonus, timer: 0.0 };
        }
    }

    // =========================================================================
    // 7. RENDERIZAÇÃO GRÁFICA NEON & EFEITOS
    // =========================================================================

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let shake_x = if self.screen_shake > 0.0 { rand::gen_range(-self.screen_shake, self.screen_shake) } else { 0.0 };
        let shake_y = if self.screen_shake > 0.0 { rand::gen_range(-self.screen_shake, self.screen_shake) } else { 0.0 };

        let ox = (screen_w - 800.0) * 0.5 + shake_x;
        let oy = (screen_h - 600.0) * 0.5 + 24.0 + shake_y;

        // Fundo da arena arcade
        draw_rectangle(ox, oy, 800.0, 600.0, BG_COLOR);
        draw_rectangle_lines(ox, oy, 800.0, 600.0, 1.5, BORDER_COLOR);

        // Grade de coordenadas de fundo sutil
        for i in 0..11 {
            let gx = ox + 40.0 + (i as f32) * 72.0;
            draw_line(gx, oy + 25.0, gx, oy + 580.0, 1.0, Color::new(1.0, 1.0, 1.0, 0.015));
        }

        // Escudo de Fundo (Safety Net) Ativo
        if self.safetynet_active {
            let net_alpha = 0.55 + 0.35 * (self.time * 7.0).sin();
            draw_line(ox + 42.0, oy + 570.0, ox + 758.0, oy + 570.0, 3.0, Color::new(0.06, 0.8, 0.55, net_alpha));
            // Nós do laser
            for i in 0..25 {
                let nx = ox + 46.0 + (i as f32) * 29.5;
                draw_circle(nx, oy + 570.0, 2.5, EMERALD);
            }
        }

        // Partículas
        for p in &self.particles {
            let a = (p.life / p.max_life).clamp(0.0, 1.0);
            let mut c = p.color;
            c.a = a;
            draw_circle(ox + p.x, oy + p.y, p.size * a, c);
        }

        // Projéteis Laser
        for l in &self.lasers {
            if l.alive {
                let lx = ox + l.x;
                let ly = oy + l.y;
                draw_rectangle(lx - 2.5, ly - 10.0, 5.0, 20.0, CORAL);
                draw_rectangle(lx - 1.0, ly - 8.0, 2.0, 16.0, TEXT_PRIMARY);
            }
        }

        // Tijolos
        for b in &self.bricks {
            if !b.alive {
                continue;
            }
            let bx = ox + b.x;
            let by = oy + b.y;

            match b.kind {
                BrickKind::Indestructible => {
                    // Titânio Metálico
                    draw_rectangle(bx + 2.0, by + 3.0, b.w, b.h, Color::new(0.0, 0.0, 0.0, 0.45));
                    draw_rectangle(bx, by, b.w, b.h, Color::new(0.24, 0.30, 0.40, 1.0));
                    draw_rectangle_lines(bx, by, b.w, b.h, 1.5, Color::new(0.55, 0.65, 0.80, 1.0));
                    // Parafusos nas quinas
                    draw_circle(bx + 4.0, by + 4.0, 1.5, Color::new(0.8, 0.85, 0.95, 0.8));
                    draw_circle(bx + b.w - 4.0, by + 4.0, 1.5, Color::new(0.8, 0.85, 0.95, 0.8));
                    draw_circle(bx + 4.0, by + b.h - 4.0, 1.5, Color::new(0.8, 0.85, 0.95, 0.8));
                    draw_circle(bx + b.w - 4.0, by + b.h - 4.0, 1.5, Color::new(0.8, 0.85, 0.95, 0.8));
                }
                BrickKind::Explosive => {
                    // Bomba Neon Pulsante
                    let pulse = 0.5 + 0.5 * (self.time * 8.0).sin();
                    let b_col = Color::new(0.95, 0.20 + pulse * 0.25, 0.15, 1.0);
                    draw_rectangle(bx + 2.0, by + 3.0, b.w, b.h, Color::new(0.0, 0.0, 0.0, 0.45));
                    draw_rectangle(bx, by, b.w, b.h, b_col);
                    draw_rectangle_lines(bx, by, b.w, b.h, 1.5, YELLOW_ACCENT);
                    // Ícone de perigo/losango no centro
                    draw_circle(bx + b.w * 0.5, by + b.h * 0.5, 4.0, YELLOW_ACCENT);
                }
                BrickKind::PowerUpSpawner => {
                    // Cristal Mágico Prismático
                    let glow = 0.6 + 0.4 * (self.time * 6.0).sin();
                    let p_col = Color::new(0.85, 0.40, 0.98, glow);
                    draw_rectangle(bx + 2.0, by + 3.0, b.w, b.h, Color::new(0.0, 0.0, 0.0, 0.45));
                    draw_rectangle(bx, by, b.w, b.h, p_col);
                    draw_rectangle_lines(bx, by, b.w, b.h, 1.5, TEXT_PRIMARY);
                    // Losango de cristal
                    let cx = bx + b.w * 0.5;
                    let cy = by + b.h * 0.5;
                    draw_triangle(
                        Vec2::new(cx, cy - 5.0),
                        Vec2::new(cx + 6.0, cy),
                        Vec2::new(cx - 6.0, cy),
                        TEXT_PRIMARY,
                    );
                    draw_triangle(
                        Vec2::new(cx, cy + 5.0),
                        Vec2::new(cx + 6.0, cy),
                        Vec2::new(cx - 6.0, cy),
                        TEXT_PRIMARY,
                    );
                }
                BrickKind::Reinforced => {
                    // Tijolo com Armadura
                    draw_rectangle(bx + 2.0, by + 3.0, b.w, b.h, Color::new(0.0, 0.0, 0.0, 0.4));
                    draw_rectangle(bx, by, b.w, b.h, b.color);
                    draw_rectangle(bx, by, b.w, 4.0, Color::new(1.0, 1.0, 1.0, 0.35));
                    draw_rectangle_lines(bx, by, b.w, b.h, 1.5, Color::new(1.0, 1.0, 1.0, 0.6));

                    // Indicadores de HP restantes
                    for h_idx in 0..b.hp {
                        let dot_x = bx + 8.0 + (h_idx as f32) * 7.0;
                        draw_circle(dot_x, by + b.h - 5.0, 2.0, TEXT_PRIMARY);
                    }

                    // Se tomou dano, desenha rachaduras
                    if b.hp < b.max_hp {
                        draw_line(bx + 12.0, by + 4.0, bx + 28.0, by + 14.0, 1.5, TEXT_PRIMARY);
                        draw_line(bx + 28.0, by + 14.0, bx + 45.0, by + 9.0, 1.5, TEXT_PRIMARY);
                    }
                }
                BrickKind::Standard => {
                    // Tijolo Padrão 3D Neon
                    draw_rectangle(bx + 2.0, by + 3.0, b.w, b.h, Color::new(0.0, 0.0, 0.0, 0.4));
                    draw_rectangle(bx, by, b.w, b.h, b.color);
                    draw_rectangle(bx, by, b.w, 4.0, Color::new(1.0, 1.0, 1.0, 0.45));
                    draw_rectangle_lines(bx, by, b.w, b.h, 1.0, Color::new(1.0, 1.0, 1.0, 0.25));
                }
            }
        }

        // Cápsulas de Power-up caindo
        for pup in &self.powerups {
            let px = ox + pup.x;
            let py = oy + pup.y;
            let col = pup.kind.color();

            // Pílula / cápsula arredondada
            draw_rectangle(px - 14.0, py - 7.0, 28.0, 14.0, SURFACE_COLOR);
            draw_rectangle_lines(px - 14.0, py - 7.0, 28.0, 14.0, 1.5, col);
            draw_circle(px - 14.0, py, 7.0, SURFACE_COLOR);
            draw_circle_lines(px - 14.0, py, 7.0, 1.5, col);
            draw_circle(px + 14.0, py, 7.0, SURFACE_COLOR);
            draw_circle_lines(px + 14.0, py, 7.0, 1.5, col);

            // Letra do power-up
            draw_text(pup.kind.icon_letter(), px - 4.5, py + 4.5, 14.0, col);
        }

        // Bolas e Rastros
        for ball in &self.balls {
            let is_fireball = self.fireball_timer > 0.0;

            // Rastro cometa da bola
            for (i, &(tx, ty)) in ball.trail.iter().enumerate() {
                let alpha = (i as f32 / ball.trail.len() as f32) * 0.45;
                let r = 2.5 + (i as f32 * 0.4);
                let trail_col = if is_fireball {
                    Color::new(1.0, 0.5, 0.1, alpha)
                } else {
                    Color::new(0.38, 0.65, 0.98, alpha)
                };
                draw_circle(ox + tx, oy + ty, r, trail_col);
            }

            // Núcleo brilhante da bola
            let bx = ox + ball.x;
            let by = oy + ball.y;

            if is_fireball {
                draw_circle(bx, by, 10.0, Color::new(1.0, 0.4, 0.1, 0.35)); // Halo de fogo
                draw_circle(bx, by, 7.0, Color::new(1.0, 0.7, 0.2, 1.0));
                draw_circle(bx - 1.5, by - 1.5, 3.0, TEXT_PRIMARY);
            } else {
                draw_circle(bx, by, 9.0, Color::new(0.38, 0.65, 0.98, 0.25)); // Halo neon
                draw_circle(bx, by, 6.5, TEXT_PRIMARY);
                draw_circle(bx - 1.5, by - 1.5, 2.5, ICE_BLUE);
            }
        }

        // Linha Guia de Mira no estado ReadyToServe
        if self.state == PlayState::ReadyToServe && !self.balls.is_empty() {
            let bx = ox + self.balls[0].x;
            let by = oy + self.balls[0].y;
            let angle_offset = (self.time * 2.8).sin() * 0.40;
            let dir_x = angle_offset.sin();
            let dir_y = -angle_offset.cos();

            for step in 1..8 {
                let d = (step as f32) * 16.0;
                let px = bx + dir_x * d;
                let py = by + dir_y * d;
                let alpha = 0.8 - (step as f32) * 0.1;
                draw_circle(px, py, 2.0, Color::new(0.6, 0.85, 1.0, alpha));
            }
        }

        // Raquete do Jogador
        let px = ox + self.paddle_x;
        let py = oy + 538.0;
        let pw = self.paddle_w;
        let ph = self.paddle_h;

        // Base metálica
        draw_rectangle(px, py, pw, ph, COBALT_ACCENT);
        // Friso neon superior
        draw_rectangle(px + 4.0, py, pw - 8.0, 3.0, ICE_BLUE);
        // Indicador central esmeralda
        draw_circle(px + pw * 0.5, py + ph * 0.5, 3.5, EMERALD);
        draw_rectangle_lines(px, py, pw, ph, 1.0, Color::new(0.5, 0.7, 1.0, 0.6));

        // Acoplamento dos canhões laser se ativo
        if self.laser_timer > 0.0 {
            // Canhão Esquerdo
            draw_rectangle(px + 4.0, py - 6.0, 6.0, 8.0, CORAL);
            draw_circle(px + 7.0, py - 6.0, 3.0, YELLOW_ACCENT);
            // Canhão Direito
            draw_rectangle(px + pw - 10.0, py - 6.0, 6.0, 8.0, CORAL);
            draw_circle(px + pw - 7.0, py - 6.0, 3.0, YELLOW_ACCENT);
        }

        // Textos flutuantes (+50, COMBO x3, etc.)
        for ft in &self.float_texts {
            let mut col = ft.color;
            col.a = ft.alpha;
            draw_text(&ft.text, ox + ft.x - 16.0, oy + ft.y, 17.0, col);
        }

        // =====================================================================
        // HUD INTERNO DA ARENA (Fase, Vidas, Multiplicador, Power-ups)
        // =====================================================================
        // Canto Superior Esquerdo: Fase Atual e Dificuldade
        let stage_str = format!("FASE {}/5", self.current_level);
        draw_text(&stage_str, ox + 45.0, oy + 22.0, 16.0, YELLOW_ACCENT);

        let diff_str = format!("[{}] x{:.1}", self.difficulty.name(), self.difficulty.score_multiplier());
        draw_text(&diff_str, ox + 140.0, oy + 22.0, 14.0, TEXT_MUTED);

        // Combo ativo no HUD
        if self.combo > 1 {
            let combo_str = format!("COMBO x{}!", self.combo);
            draw_text(&combo_str, ox + 320.0, oy + 22.0, 16.0, CORAL);
        }

        // Badges de Power-ups ativos no topo da arena
        let mut badge_x = ox + 440.0;
        if self.wide_timer > 0.0 {
            let t_str = format!("W {:.0}s", self.wide_timer);
            draw_card_rect(badge_x, oy + 8.0, 52.0, 18.0, SURFACE_COLOR, ICE_BLUE);
            draw_text(&t_str, badge_x + 6.0, oy + 21.0, 11.0, ICE_BLUE);
            badge_x += 58.0;
        }
        if self.laser_timer > 0.0 {
            let t_str = format!("L {:.0}s", self.laser_timer);
            draw_card_rect(badge_x, oy + 8.0, 50.0, 18.0, SURFACE_COLOR, CORAL);
            draw_text(&t_str, badge_x + 6.0, oy + 21.0, 11.0, CORAL);
            badge_x += 56.0;
        }
        if self.fireball_timer > 0.0 {
            let t_str = format!("F {:.0}s", self.fireball_timer);
            draw_card_rect(badge_x, oy + 8.0, 50.0, 18.0, SURFACE_COLOR, Color::new(1.0, 0.45, 0.1, 1.0));
            draw_text(&t_str, badge_x + 6.0, oy + 21.0, 11.0, Color::new(1.0, 0.45, 0.1, 1.0));
            badge_x += 56.0;
        }
        if self.safetynet_active {
            let t_str = format!("S {:.0}s", self.safetynet_timer);
            draw_card_rect(badge_x, oy + 8.0, 50.0, 18.0, SURFACE_COLOR, EMERALD);
            draw_text(&t_str, badge_x + 6.0, oy + 21.0, 11.0, EMERALD);
        }

        // Vidas restantes estilizadas no Canto Superior Direito
        let max_display_lives = self.difficulty.lives();
        for i in 0..max_display_lives {
            let col = if i < self.lives { CORAL } else { Color::new(0.2, 0.25, 0.35, 0.5) };
            draw_circle(ox + 750.0 - (i as f32) * 16.0, oy + 16.0, 5.0, col);
        }

        // Instruções no estado ReadyToServe
        if self.state == PlayState::ReadyToServe {
            let serve_msg = "Pressione [ESPAÇO] ou [CLIQUE] para Lançar a Bola";
            let dim = measure_text(serve_msg, None, 18, 1.0);
            draw_text(serve_msg, ox + (800.0 - dim.width) * 0.5, oy + 490.0, 18.0, TEXT_PRIMARY);

            if self.current_level == 1 && self.score == 0 {
                let diff_hint = "Alterar Dificuldade: [1] Fácil  [2] Normal  [3] Difícil";
                let h_dim = measure_text(diff_hint, None, 14, 1.0);
                draw_text(diff_hint, ox + (800.0 - h_dim.width) * 0.5, oy + 514.0, 14.0, TEXT_MUTED);
            }
        }

        // =====================================================================
        // TELA DE TRANSIÇÃO DE FASE (LEVEL CLEAR)
        // =====================================================================
        if let PlayState::LevelClear { bonus, timer: _ } = &self.state {
            draw_rectangle(ox, oy, 800.0, 600.0, Color::new(0.03, 0.05, 0.09, 0.85));

            let banner_w = 520.0;
            let banner_h = 200.0;
            let bx = ox + (800.0 - banner_w) * 0.5;
            let by = oy + (600.0 - banner_h) * 0.5;
            draw_card_rect(bx, by, banner_w, banner_h, SURFACE_COLOR, YELLOW_ACCENT);

            let msg = format!("FASE {} CONCLUÍDA!", self.current_level);
            let m_dim = measure_text(&msg, None, 28, 1.0);
            draw_text(&msg, bx + (banner_w - m_dim.width) * 0.5, by + 50.0, 28.0, YELLOW_ACCENT);

            let b_msg = format!("Bônus de Fase: +{} PONTOS", bonus);
            let b_dim = measure_text(&b_msg, None, 18, 1.0);
            draw_text(&b_msg, bx + (banner_w - b_dim.width) * 0.5, by + 90.0, 18.0, EMERALD);

            let s_msg = format!("Pontuação Acumulada: {}", self.score);
            let s_dim = measure_text(&s_msg, None, 16, 1.0);
            draw_text(&s_msg, bx + (banner_w - s_dim.width) * 0.5, by + 122.0, 16.0, TEXT_PRIMARY);

            let prompt = "Pressione [ESPAÇO] ou [CLIQUE] para a Próxima Fase";
            let p_dim = measure_text(prompt, None, 15, 1.0);
            draw_text(prompt, bx + (banner_w - p_dim.width) * 0.5, by + 165.0, 15.0, ICE_BLUE);
        }

        // =====================================================================
        // TELA DE VITÓRIA CÓSMICA (CAMPANHA ZERADA)
        // =====================================================================
        if let PlayState::Victory { timer: _ } = &self.state {
            draw_rectangle(ox, oy, 800.0, 600.0, Color::new(0.03, 0.05, 0.09, 0.92));

            let banner_w = 600.0;
            let banner_h = 240.0;
            let bx = ox + (800.0 - banner_w) * 0.5;
            let by = oy + (600.0 - banner_h) * 0.5;
            draw_card_rect(bx, by, banner_w, banner_h, SURFACE_COLOR, EMERALD);

            let v_title = "VITÓRIA CÓSMICA!";
            let v_dim = measure_text(v_title, None, 34, 1.0);
            draw_text(v_title, bx + (banner_w - v_dim.width) * 0.5, by + 55.0, 34.0, EMERALD);

            let sub = "PARABÉNS! VOCÊ CONQUISTOU TODOS OS 5 NÍVEIS DO PRISMA!";
            let s_dim = measure_text(sub, None, 15, 1.0);
            draw_text(sub, bx + (banner_w - s_dim.width) * 0.5, by + 95.0, 15.0, TEXT_PRIMARY);

            let score_final = format!("PONTUAÇÃO SUPREMA: {} PONTOS", self.score);
            let sf_dim = measure_text(&score_final, None, 20, 1.0);
            draw_text(&score_final, bx + (banner_w - sf_dim.width) * 0.5, by + 138.0, 20.0, YELLOW_ACCENT);

            let diff_note = format!("Dificuldade Conquistada: {} (Multiplicador x{:.1})", self.difficulty.name(), self.difficulty.score_multiplier());
            let d_dim = measure_text(&diff_note, None, 14, 1.0);
            draw_text(&diff_note, bx + (banner_w - d_dim.width) * 0.5, by + 168.0, 14.0, ICE_BLUE);

            let opt = "Pressione [R] para Jogar Novamente ou [ESC] para Voltar à Galeria";
            let o_dim = measure_text(opt, None, 14, 1.0);
            draw_text(opt, bx + (banner_w - o_dim.width) * 0.5, by + 208.0, 14.0, TEXT_MUTED);
        }
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
        "Breakout Prism"
    }
}
