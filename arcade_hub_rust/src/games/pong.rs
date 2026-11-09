use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

// =========================================================================
// 1. ENUMS E CONFIGURAÇÕES
// =========================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Difficulty {
    Casual,      // IA suave, velocidade moderada
    Competitive, // IA rápida que tenta smashes
    CyberMaster, // IA de alta velocidade com leitura de efeito
}

impl Difficulty {
    pub fn name(&self) -> &'static str {
        match self {
            Difficulty::Casual => "CASUAL",
            Difficulty::Competitive => "COMPETITIVO",
            Difficulty::CyberMaster => "CYBER MASTER",
        }
    }

    pub fn ai_speed(&self) -> f32 {
        match self {
            Difficulty::Casual => 5.8,
            Difficulty::Competitive => 7.6,
            Difficulty::CyberMaster => 9.4,
        }
    }

    pub fn score_multiplier(&self) -> f32 {
        match self {
            Difficulty::Casual => 1.0,
            Difficulty::Competitive => 1.5,
            Difficulty::CyberMaster => 2.5,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameMode {
    SinglePlayer,
    TwoPlayersLocal,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PowerUpType {
    Fireball,   // Bola de fogo ultraveloz (+40% velocidade)
    WidePaddle, // Aumenta a raquete em +35px por 12s
    GoalShield, // Barreira de energia no fundo da quadra que salva 1 bola
    MultiBall,  // Divide a bola em duas bolas ativas no campo
    SlowMo,     // Câmera lenta temporária
}

impl PowerUpType {
    pub fn label(&self) -> &'static str {
        match self {
            PowerUpType::Fireball => "BOLA DE FOGO!",
            PowerUpType::WidePaddle => "RAQUETE EXPANDIDA!",
            PowerUpType::GoalShield => "ESCUDO DE DEFESA!",
            PowerUpType::MultiBall => "MULTI-BOLA!",
            PowerUpType::SlowMo => "CÂMERA LENTA!",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            PowerUpType::Fireball => Color::new(1.0, 0.45, 0.1, 1.0),
            PowerUpType::WidePaddle => ICE_BLUE,
            PowerUpType::GoalShield => EMERALD,
            PowerUpType::MultiBall => YELLOW_ACCENT,
            PowerUpType::SlowMo => Color::new(0.70, 0.40, 0.95, 1.0),
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            PowerUpType::Fireball => "F",
            PowerUpType::WidePaddle => "W",
            PowerUpType::GoalShield => "S",
            PowerUpType::MultiBall => "M",
            PowerUpType::SlowMo => "C",
        }
    }
}

#[derive(Clone)]
struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    spin: f32, // Efeito de curva vertical
    is_smash: bool,
    is_fireball: bool,
    last_hitter: u32, // 1 para P1, 2 para P2/IA
    trail: Vec<(f32, f32)>,
}

#[derive(Clone)]
struct PowerOrb {
    x: f32,
    y: f32,
    kind: PowerUpType,
    alive: bool,
    float_phase: f32,
}

struct PongParticle {
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

#[derive(PartialEq, Debug)]
enum MatchState {
    ReadyToServe,
    Playing,
    GoalScored { scorer: u32, timer: f32 },
    MatchOver { champion: u32, timer: f32 },
}

// =========================================================================
// 2. ESTRUTURA PRINCIPAL DO JOGO
// =========================================================================

pub struct PongGame {
    state: MatchState,
    difficulty: Difficulty,
    mode: GameMode,

    // Raquetes
    p1_y: f32,
    p1_vy: f32,
    p1_h: f32,
    p1_smash_cooldown: f32,
    p1_shield: bool,
    p1_wide_timer: f32,

    p2_y: f32,
    p2_vy: f32,
    p2_h: f32,
    p2_smash_cooldown: f32,
    p2_shield: bool,
    p2_wide_timer: f32,

    // Bolas e orbes
    balls: Vec<Ball>,
    orbs: Vec<PowerOrb>,
    orb_spawn_timer: f32,

    // Placar e estatísticas
    p1_score: u32,
    p2_score: u32,
    rally_count: u32,
    longest_rally: u32,
    smashes_p1: u32,
    smashes_p2: u32,
    total_score: u32,

    time: f32,
    screen_shake: f32,
    court_flash_color: Option<(Color, f32)>,
    game_over: bool,

    particles: Vec<PongParticle>,
    float_texts: Vec<FloatText>,
}

impl PongGame {
    pub fn new() -> Self {
        let diff = Difficulty::Competitive;
        let mut game = Self {
            state: MatchState::ReadyToServe,
            difficulty: diff,
            mode: GameMode::SinglePlayer,

            p1_y: 250.0,
            p1_vy: 0.0,
            p1_h: 100.0,
            p1_smash_cooldown: 0.0,
            p1_shield: false,
            p1_wide_timer: 0.0,

            p2_y: 250.0,
            p2_vy: 0.0,
            p2_h: 100.0,
            p2_smash_cooldown: 0.0,
            p2_shield: false,
            p2_wide_timer: 0.0,

            balls: Vec::new(),
            orbs: Vec::new(),
            orb_spawn_timer: 0.0,

            p1_score: 0,
            p2_score: 0,
            rally_count: 0,
            longest_rally: 0,
            smashes_p1: 0,
            smashes_p2: 0,
            total_score: 0,

            time: 0.0,
            screen_shake: 0.0,
            court_flash_color: None,
            game_over: false,

            particles: Vec::new(),
            float_texts: Vec::new(),
        };

        game.spawn_initial_ball(1);
        game.spawn_power_orb();
        game
    }

    fn spawn_initial_ball(&mut self, serve_towards: u32) {
        self.balls.clear();
        let vx = if serve_towards == 1 { -6.5 } else { 6.5 };
        let vy = rand::gen_range(-2.8, 2.8);
        self.balls.push(Ball {
            x: 400.0,
            y: 300.0,
            vx,
            vy,
            spin: 0.0,
            is_smash: false,
            is_fireball: false,
            last_hitter: if serve_towards == 1 { 2 } else { 1 },
            trail: Vec::new(),
        });
        self.rally_count = 0;
    }

    fn spawn_power_orb(&mut self) {
        for _ in 0..50 {
            let x = rand::gen_range(280.0, 520.0);
            let y = rand::gen_range(90.0, 510.0);
            let kind = match rand::gen_range(0, 5) {
                0 => PowerUpType::Fireball,
                1 => PowerUpType::WidePaddle,
                2 => PowerUpType::GoalShield,
                3 => PowerUpType::MultiBall,
                _ => PowerUpType::SlowMo,
            };
            self.orbs.push(PowerOrb {
                x,
                y,
                kind,
                alive: true,
                float_phase: rand::gen_range(0.0, 6.28),
            });
            break;
        }
    }

    fn set_difficulty(&mut self, diff: Difficulty) {
        self.difficulty = diff;
        self.float_texts.push(FloatText {
            x: 400.0,
            y: 460.0,
            text: format!("DIFICULDADE: {}", diff.name()),
            color: YELLOW_ACCENT,
            alpha: 1.0,
        });
    }

    fn apply_powerup(&mut self, kind: PowerUpType, player_id: u32) {
        let (name, col) = match player_id {
            1 => ("P1", ICE_BLUE),
            _ => ("P2", CORAL),
        };

        match kind {
            PowerUpType::Fireball => {
                for b in &mut self.balls {
                    if b.last_hitter == player_id {
                        b.is_fireball = true;
                        b.vx *= 1.35;
                    }
                }
                self.screen_shake = 4.0;
                self.float_texts.push(FloatText {
                    x: 400.0,
                    y: 200.0,
                    text: format!("{}: BOLA DE FOGO!", name),
                    color: Color::new(1.0, 0.45, 0.1, 1.0),
                    alpha: 1.2,
                });
            }
            PowerUpType::WidePaddle => {
                if player_id == 1 {
                    self.p1_h = 135.0;
                    self.p1_wide_timer = 12.0;
                } else {
                    self.p2_h = 135.0;
                    self.p2_wide_timer = 12.0;
                }
                self.float_texts.push(FloatText {
                    x: 400.0,
                    y: 200.0,
                    text: format!("{}: RAQUETE EXPANDIDA!", name),
                    color: col,
                    alpha: 1.2,
                });
            }
            PowerUpType::GoalShield => {
                if player_id == 1 {
                    self.p1_shield = true;
                } else {
                    self.p2_shield = true;
                }
                self.float_texts.push(FloatText {
                    x: 400.0,
                    y: 200.0,
                    text: format!("{}: ESCUDO ATIVO!", name),
                    color: EMERALD,
                    alpha: 1.2,
                });
            }
            PowerUpType::MultiBall => {
                let mut new_balls = Vec::new();
                for b in &self.balls {
                    new_balls.push(Ball {
                        x: b.x,
                        y: b.y,
                        vx: b.vx * 0.9,
                        vy: -b.vy - 2.0,
                        spin: 0.0,
                        is_smash: false,
                        is_fireball: false,
                        last_hitter: player_id,
                        trail: Vec::new(),
                    });
                }
                self.balls.extend(new_balls);
                self.float_texts.push(FloatText {
                    x: 400.0,
                    y: 200.0,
                    text: "MULTI-BOLA ATIVADA!".to_string(),
                    color: YELLOW_ACCENT,
                    alpha: 1.2,
                });
            }
            PowerUpType::SlowMo => {
                for b in &mut self.balls {
                    b.vx *= 0.65;
                    b.vy *= 0.65;
                }
                self.float_texts.push(FloatText {
                    x: 400.0,
                    y: 200.0,
                    text: "CÂMERA LENTA!".to_string(),
                    color: Color::new(0.70, 0.40, 0.95, 1.0),
                    alpha: 1.2,
                });
            }
        }
    }

    fn spawn_impact_particles(&mut self, x: f32, y: f32, color: Color, count: usize) {
        for _ in 0..count {
            self.particles.push(PongParticle {
                x,
                y,
                vx: rand::gen_range(-4.0, 4.0),
                vy: rand::gen_range(-4.0, 4.0),
                color,
                life: 0.45,
                max_life: 0.45,
                size: rand::gen_range(2.5, 4.5),
            });
        }
    }
}

// =========================================================================
// 3. IMPLEMENTAÇÃO DO TRAIT ARCADETRAIT
// =========================================================================

impl ArcadeGame for PongGame {
    fn update(&mut self) {
        let dt = get_frame_time();
        self.time += dt;

        // Amortecimento do tremor de tela
        if self.screen_shake > 0.0 {
            self.screen_shake = (self.screen_shake - dt * 16.0).max(0.0);
        }

        // Amortecimento do flash da quadra
        if let Some((_, timer)) = &mut self.court_flash_color {
            *timer -= dt;
            if *timer <= 0.0 {
                self.court_flash_color = None;
            }
        }

        // Atualização dos temporizadores de raquete e poderes
        if self.p1_smash_cooldown > 0.0 {
            self.p1_smash_cooldown -= dt;
        }
        if self.p2_smash_cooldown > 0.0 {
            self.p2_smash_cooldown -= dt;
        }

        if self.p1_wide_timer > 0.0 {
            self.p1_wide_timer -= dt;
            if self.p1_wide_timer <= 0.0 {
                self.p1_h = 100.0;
            }
        }
        if self.p2_wide_timer > 0.0 {
            self.p2_wide_timer -= dt;
            if self.p2_wide_timer <= 0.0 {
                self.p2_h = 100.0;
            }
        }

        // Spawn periódico de orbes
        self.orb_spawn_timer += dt;
        if self.orb_spawn_timer >= 10.0 {
            self.orb_spawn_timer = 0.0;
            if self.orbs.len() < 2 {
                self.spawn_power_orb();
            }
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

        // =====================================================================
        // TRATAMENTO DE ESTADOS DA PARTIDA
        // =====================================================================
        match &mut self.state {
            MatchState::ReadyToServe => {
                // Seleção de Dificuldade
                if is_key_pressed(KeyCode::Key1) {
                    self.set_difficulty(Difficulty::Casual);
                } else if is_key_pressed(KeyCode::Key2) {
                    self.set_difficulty(Difficulty::Competitive);
                } else if is_key_pressed(KeyCode::Key3) {
                    self.set_difficulty(Difficulty::CyberMaster);
                }

                // Alternar Modo 1P / 2P
                if is_key_pressed(KeyCode::Tab) || is_key_pressed(KeyCode::M) {
                    self.mode = match self.mode {
                        GameMode::SinglePlayer => GameMode::TwoPlayersLocal,
                        GameMode::TwoPlayersLocal => GameMode::SinglePlayer,
                    };
                    self.float_texts.push(FloatText {
                        x: 400.0,
                        y: 430.0,
                        text: if self.mode == GameMode::SinglePlayer { "MODO: 1 JOGADOR (VS IA)".into() } else { "MODO: 2 JOGADORES LOCAL".into() },
                        color: YELLOW_ACCENT,
                        alpha: 1.0,
                    });
                }

                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    self.state = MatchState::Playing;
                }
                return;
            }
            MatchState::GoalScored { scorer, timer } => {
                *timer += dt;
                if *timer >= 1.6 || is_key_pressed(KeyCode::Space) {
                    if self.p1_score >= 5 || self.p2_score >= 5 {
                        let champ = if self.p1_score >= 5 { 1 } else { 2 };
                        self.state = MatchState::MatchOver { champion: champ, timer: 0.0 };
                    } else {
                        let next_serve = if *scorer == 1 { 2 } else { 1 };
                        self.spawn_initial_ball(next_serve);
                        self.state = MatchState::Playing;
                    }
                }
                return;
            }
            MatchState::MatchOver { champion: _, timer } => {
                *timer += dt;
                if is_key_pressed(KeyCode::R) {
                    *self = Self::new();
                }
                return;
            }
            MatchState::Playing => {}
        }

        if self.game_over {
            return;
        }

        // =====================================================================
        // CONTROLES JOGADOR 1 (ESQUERDA)
        // =====================================================================
        let speed = 9.2;
        let prev_p1_y = self.p1_y;

        if is_key_down(KeyCode::W) || (self.mode == GameMode::SinglePlayer && is_key_down(KeyCode::Up)) {
            self.p1_y = (self.p1_y - speed).max(28.0);
        }
        if is_key_down(KeyCode::S) || (self.mode == GameMode::SinglePlayer && is_key_down(KeyCode::Down)) {
            self.p1_y = (self.p1_y + speed).min(572.0 - self.p1_h);
        }
        self.p1_vy = self.p1_y - prev_p1_y;

        // Smash Jogador 1 (Espaço)
        let mut p1_wants_smash = false;
        if is_key_pressed(KeyCode::Space) && self.p1_smash_cooldown <= 0.0 {
            p1_wants_smash = true;
            self.p1_smash_cooldown = 1.8;
        }

        // =====================================================================
        // CONTROLES JOGADOR 2 / INTELIGÊNCIA ARTIFICIAL (DIREITA)
        // =====================================================================
        let prev_p2_y = self.p2_y;
        let mut p2_wants_smash = false;

        if self.mode == GameMode::TwoPlayersLocal {
            // Controles manuais Jogador 2
            if is_key_down(KeyCode::Up) {
                self.p2_y = (self.p2_y - speed).max(28.0);
            }
            if is_key_down(KeyCode::Down) {
                self.p2_y = (self.p2_y + speed).min(572.0 - self.p2_h);
            }
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::RightShift) {
                if self.p2_smash_cooldown <= 0.0 {
                    p2_wants_smash = true;
                    self.p2_smash_cooldown = 1.8;
                }
            }
        } else {
            // IA com cálculo de trajetória, curva e reflexo de Smash
            let target_ball = self.balls.iter().min_by(|a, b| (740.0 - a.x).partial_cmp(&(740.0 - b.x)).unwrap());
            if let Some(target) = target_ball {
                let ai_target = target.y - self.p2_h * 0.5;
                let ai_speed = (self.difficulty.ai_speed() + (self.rally_count as f32 * 0.12)).min(11.0);

                if (self.p2_y - ai_target).abs() > 6.0 {
                    if self.p2_y < ai_target {
                        self.p2_y = (self.p2_y + ai_speed).min(572.0 - self.p2_h);
                    } else {
                        self.p2_y = (self.p2_y - ai_speed).max(28.0);
                    }
                }

                // IA decide se executa Smash quando a bola estiver próxima
                if target.x >= 690.0 && target.vx > 0.0 && self.p2_smash_cooldown <= 0.0 {
                    let should_smash = match self.difficulty {
                        Difficulty::Casual => false,
                        Difficulty::Competitive => rand::gen_range(0, 10) < 4,
                        Difficulty::CyberMaster => rand::gen_range(0, 10) < 8,
                    };
                    if should_smash {
                        p2_wants_smash = true;
                        self.p2_smash_cooldown = 2.0;
                    }
                }
            }
        }
        self.p2_vy = self.p2_y - prev_p2_y;

        // =====================================================================
        // FÍSICA DAS BOLAS, SPIN E COLISÕES
        // =====================================================================
        let p1_x = 42.0;
        let p1_w = 14.0;
        let p2_x = 744.0;
        let p2_w = 14.0;

        enum PongEvent {
            ImpactParticles { x: f32, y: f32, color: Color, count: usize },
            ApplyPowerUp { kind: PowerUpType, player_id: u32 },
        }

        let mut events = Vec::new();
        let mut scored_event = None;

        for ball in &mut self.balls {
            // Rastro cometa da bola
            ball.trail.push((ball.x, ball.y));
            if ball.trail.len() > 10 {
                ball.trail.remove(0);
            }

            // Aplica efeito de curva vertical dinâmico (spin)
            ball.vy += ball.spin * dt * 40.0;
            ball.spin *= 0.985; // Amortece o spin gradativamente

            ball.x += ball.vx;
            ball.y += ball.vy;

            // Paredes Superior e Inferior
            if ball.y <= 28.0 {
                ball.y = 28.0;
                ball.vy = ball.vy.abs();
                events.push(PongEvent::ImpactParticles { x: ball.x, y: ball.y, color: ICE_BLUE, count: 6 });
            } else if ball.y >= 572.0 {
                ball.y = 572.0;
                ball.vy = -ball.vy.abs();
                events.push(PongEvent::ImpactParticles { x: ball.x, y: ball.y, color: ICE_BLUE, count: 6 });
            }

            // Escudo de Defesa P1
            if self.p1_shield && ball.x <= 20.0 && ball.vx < 0.0 {
                self.p1_shield = false;
                ball.vx = ball.vx.abs();
                self.screen_shake = 5.0;
                events.push(PongEvent::ImpactParticles { x: 20.0, y: ball.y, color: EMERALD, count: 14 });
                self.float_texts.push(FloatText {
                    x: 60.0,
                    y: ball.y,
                    text: "ESCUDO DE DEFESA SALVOU!".into(),
                    color: EMERALD,
                    alpha: 1.0,
                });
            }

            // Escudo de Defesa P2
            if self.p2_shield && ball.x >= 780.0 && ball.vx > 0.0 {
                self.p2_shield = false;
                ball.vx = -ball.vx.abs();
                self.screen_shake = 5.0;
                events.push(PongEvent::ImpactParticles { x: 780.0, y: ball.y, color: EMERALD, count: 14 });
                self.float_texts.push(FloatText {
                    x: 720.0,
                    y: ball.y,
                    text: "ESCUDO DE DEFESA SALVOU!".into(),
                    color: EMERALD,
                    alpha: 1.0,
                });
            }

            // =================================================================
            // COLISÃO COM RAQUETE P1 (ESQUERDA)
            // =================================================================
            if ball.x >= p1_x - 4.0 && ball.x <= p1_x + p1_w + 6.0 &&
               ball.y >= self.p1_y - 6.0 && ball.y <= self.p1_y + self.p1_h + 6.0 &&
               ball.vx < 0.0
            {
                self.rally_count += 1;
                self.longest_rally = self.longest_rally.max(self.rally_count);
                ball.last_hitter = 1;

                let offset = (ball.y - (self.p1_y + self.p1_h * 0.5)) / (self.p1_h * 0.5);

                // Executa SMASH se o jogador acionou o timing
                let is_smash = p1_wants_smash || (self.p1_vy.abs() > 7.0 && rand::gen_range(0, 10) < 3);
                let base_spd = if is_smash {
                    self.smashes_p1 += 1;
                    ball.is_smash = true;
                    self.screen_shake = 6.0;
                    self.float_texts.push(FloatText {
                        x: p1_x + 50.0,
                        y: ball.y,
                        text: "SMASH POTENTE!".into(),
                        color: YELLOW_ACCENT,
                        alpha: 1.2,
                    });
                    15.0 + (self.rally_count as f32 * 0.25).min(5.0)
                } else {
                    ball.is_smash = false;
                    (7.0 + (self.rally_count as f32 * 0.22)).min(14.0)
                };

                ball.vx = base_spd;
                ball.vy = offset * (base_spd * 0.75);

                // Transfere velocidade da raquete para o Spin (efeito de curva)
                ball.spin = (self.p1_vy * 0.22).clamp(-1.2, 1.2);

                self.court_flash_color = Some((ICE_BLUE, 0.12));
                events.push(PongEvent::ImpactParticles { x: p1_x + p1_w, y: ball.y, color: ICE_BLUE, count: if is_smash { 18 } else { 10 } });
                self.total_score += (10.0 * self.difficulty.score_multiplier()) as u32;
            }

            // =================================================================
            // COLISÃO COM RAQUETE P2 / IA (DIREITA)
            // =================================================================
            if ball.x >= p2_x - 6.0 && ball.x <= p2_x + p2_w + 4.0 &&
               ball.y >= self.p2_y - 6.0 && ball.y <= self.p2_y + self.p2_h + 6.0 &&
               ball.vx > 0.0
            {
                self.rally_count += 1;
                self.longest_rally = self.longest_rally.max(self.rally_count);
                ball.last_hitter = 2;

                let offset = (ball.y - (self.p2_y + self.p2_h * 0.5)) / (self.p2_h * 0.5);

                let is_smash = p2_wants_smash;
                let base_spd = if is_smash {
                    self.smashes_p2 += 1;
                    ball.is_smash = true;
                    self.screen_shake = 6.0;
                    self.float_texts.push(FloatText {
                        x: p2_x - 60.0,
                        y: ball.y,
                        text: "SMASH ADVERSÁRIO!".into(),
                        color: CORAL,
                        alpha: 1.2,
                    });
                    15.0 + (self.rally_count as f32 * 0.25).min(5.0)
                } else {
                    ball.is_smash = false;
                    (7.0 + (self.rally_count as f32 * 0.22)).min(14.0)
                };

                ball.vx = -base_spd;
                ball.vy = offset * (base_spd * 0.75);
                ball.spin = (self.p2_vy * 0.22).clamp(-1.2, 1.2);

                self.court_flash_color = Some((CORAL, 0.12));
                events.push(PongEvent::ImpactParticles { x: p2_x, y: ball.y, color: CORAL, count: if is_smash { 18 } else { 10 } });
            }

            // =================================================================
            // COLISÃO COM ORBES DE PODER NO CENTRO DA QUADRA
            // =================================================================
            for orb in &mut self.orbs {
                if !orb.alive {
                    continue;
                }
                let dx = ball.x - orb.x;
                let dy = ball.y - orb.y;
                if dx * dx + dy * dy < (18.0 * 18.0) {
                    orb.alive = false;
                    let last_hit = ball.last_hitter;
                    events.push(PongEvent::ApplyPowerUp { kind: orb.kind, player_id: last_hit });
                    events.push(PongEvent::ImpactParticles { x: orb.x, y: orb.y, color: orb.kind.color(), count: 16 });
                }
            }

            // Verificação de Ponto
            if ball.x < -10.0 {
                scored_event = Some(2);
            } else if ball.x > 810.0 {
                scored_event = Some(1);
            }
        }

        // Processa eventos coletados
        for ev in events {
            match ev {
                PongEvent::ImpactParticles { x, y, color, count } => {
                    self.spawn_impact_particles(x, y, color, count);
                }
                PongEvent::ApplyPowerUp { kind, player_id } => {
                    self.apply_powerup(kind, player_id);
                }
            }
        }

        self.orbs.retain(|o| o.alive);

        // Tratamento de Gol Marcado
        if let Some(scorer) = scored_event {
            self.screen_shake = 8.0;
            if scorer == 1 {
                self.p1_score += 1;
                let pts = (150.0 * self.difficulty.score_multiplier()) as u32;
                self.total_score += pts;
                self.float_texts.push(FloatText {
                    x: 400.0,
                    y: 280.0,
                    text: format!("PONTO DO JOGADOR 1 (+{})!", pts),
                    color: ICE_BLUE,
                    alpha: 1.5,
                });
            } else {
                self.p2_score += 1;
                self.float_texts.push(FloatText {
                    x: 400.0,
                    y: 280.0,
                    text: if self.mode == GameMode::SinglePlayer { "PONTO DA IA!".into() } else { "PONTO DO JOGADOR 2!".into() },
                    color: CORAL,
                    alpha: 1.5,
                });
            }
            self.state = MatchState::GoalScored { scorer, timer: 0.0 };
        }
    }

    // =========================================================================
    // 4. RENDERIZAÇÃO GRÁFICA & ESTÉTICA SMASH NEON
    // =========================================================================

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let shake_x = if self.screen_shake > 0.0 { rand::gen_range(-self.screen_shake, self.screen_shake) } else { 0.0 };
        let shake_y = if self.screen_shake > 0.0 { rand::gen_range(-self.screen_shake, self.screen_shake) } else { 0.0 };

        let ox = (screen_w - 800.0) * 0.5 + shake_x;
        let oy = (screen_h - 600.0) * 0.5 + 24.0 + shake_y;

        // Fundo com flash reativo
        let base_bg = if let Some((col, _timer)) = self.court_flash_color {
            Color::new(col.r * 0.25, col.g * 0.25, col.b * 0.25, 1.0)
        } else {
            BG_COLOR
        };
        draw_rectangle(ox, oy, 800.0, 600.0, base_bg);
        draw_rectangle_lines(ox, oy, 800.0, 600.0, 2.0, BORDER_ACTIVE);

        // Linha divisória central neon
        for i in 0..16 {
            let ly = oy + 28.0 + (i as f32) * 36.0;
            draw_rectangle(ox + 398.0, ly, 4.0, 18.0, Color::new(0.38, 0.65, 0.98, 0.35));
        }

        // Círculo central da quadra com iluminação
        draw_circle_lines(ox + 400.0, oy + 300.0, 65.0, 1.5, Color::new(0.38, 0.65, 0.98, 0.25));
        draw_circle(ox + 400.0, oy + 300.0, 4.0, ICE_BLUE);

        // Placar translúcido gigante no chão da quadra
        let p1_str = format!("{}", self.p1_score);
        let p2_str = format!("{}", self.p2_score);
        draw_text(&p1_str, ox + 310.0, oy + 120.0, 85.0, Color::new(0.38, 0.65, 0.98, 0.18));
        draw_text(&p2_str, ox + 440.0, oy + 120.0, 85.0, Color::new(0.96, 0.44, 0.44, 0.18));

        // Escudo de Defesa P1
        if self.p1_shield {
            draw_line(ox + 16.0, oy + 28.0, ox + 16.0, oy + 572.0, 3.0, EMERALD);
        }
        // Escudo de Defesa P2
        if self.p2_shield {
            draw_line(ox + 784.0, oy + 28.0, ox + 784.0, oy + 572.0, 3.0, EMERALD);
        }

        // Orbes de Poder flutuantes
        for orb in &self.orbs {
            if !orb.alive {
                continue;
            }
            let pulse = (self.time * 6.0 + orb.float_phase).sin() * 2.5;
            let col = orb.kind.color();
            draw_circle(ox + orb.x, oy + orb.y, 11.0 + pulse * 0.5, SURFACE_COLOR);
            draw_circle_lines(ox + orb.x, oy + orb.y, 11.0 + pulse * 0.5, 2.0, col);
            draw_text(orb.kind.icon(), ox + orb.x - 4.5, oy + orb.y + 4.5, 13.0, col);
        }

        // Partículas
        for p in &self.particles {
            let a = (p.life / p.max_life).clamp(0.0, 1.0);
            let mut c = p.color;
            c.a = a;
            draw_circle(ox + p.x, oy + p.y, p.size * a, c);
        }

        // =====================================================================
        // BOLAS, RASTROS E EFEITOS DE CURVA
        // =====================================================================
        for ball in &self.balls {
            let trail_col = if ball.is_fireball {
                Color::new(1.0, 0.45, 0.1, 0.45)
            } else if ball.is_smash {
                YELLOW_ACCENT
            } else if ball.last_hitter == 1 {
                Color::new(0.38, 0.65, 0.98, 0.35)
            } else {
                Color::new(0.96, 0.44, 0.44, 0.35)
            };

            for (i, &(tx, ty)) in ball.trail.iter().enumerate() {
                let alpha = (i as f32 / ball.trail.len() as f32) * 0.45;
                draw_circle(ox + tx, oy + ty, 3.0 + (i as f32 * 0.4), Color::new(trail_col.r, trail_col.g, trail_col.b, alpha));
            }

            let bx = ox + ball.x;
            let by = oy + ball.y;

            if ball.is_fireball {
                draw_circle(bx, by, 11.0, Color::new(1.0, 0.45, 0.1, 0.35));
                draw_circle(bx, by, 7.5, Color::new(1.0, 0.7, 0.2, 1.0));
                draw_circle(bx - 1.5, by - 1.5, 2.5, TEXT_PRIMARY);
            } else if ball.is_smash {
                draw_circle(bx, by, 10.0, Color::new(1.0, 0.85, 0.2, 0.45));
                draw_circle(bx, by, 7.0, TEXT_PRIMARY);
                draw_circle(bx - 1.5, by - 1.5, 2.5, YELLOW_ACCENT);
            } else {
                draw_circle(bx, by, 8.5, Color::new(0.38, 0.65, 0.98, 0.25));
                draw_circle(bx, by, 6.0, TEXT_PRIMARY);
                draw_circle(bx - 1.5, by - 1.5, 2.0, ICE_BLUE);
            }
        }

        // =====================================================================
        // RAQUETES P1 E P2 / IA
        // =====================================================================
        // Raquete Jogador 1 (Azul Cobalto / Safira)
        let px = ox + 42.0;
        let py = oy + self.p1_y;
        let pw = 14.0;
        draw_rectangle(px, py, pw, self.p1_h, COBALT_ACCENT);
        draw_rectangle(px + pw - 3.0, py, 3.0, self.p1_h, ICE_BLUE);
        draw_rectangle_lines(px, py, pw, self.p1_h, 1.5, Color::new(0.5, 0.8, 1.0, 0.6));

        // Raquete Jogador 2 / IA (Coral / Ouro)
        let p2x = ox + 744.0;
        let p2y = oy + self.p2_y;
        draw_rectangle(p2x, p2y, pw, self.p2_h, Color::new(0.75, 0.2, 0.25, 1.0));
        draw_rectangle(p2x, p2y, 3.0, self.p2_h, CORAL);
        draw_rectangle_lines(p2x, p2y, pw, self.p2_h, 1.5, Color::new(1.0, 0.5, 0.5, 0.6));

        // Textos flutuantes
        for ft in &self.float_texts {
            let mut col = ft.color;
            col.a = ft.alpha;
            draw_text(&ft.text, ox + ft.x - 16.0, oy + ft.y, 16.0, col);
        }

        // =====================================================================
        // HUD SUPERIOR DA ARENA
        // =====================================================================
        let r_str = format!("RALI ATUAL: {}  (RECORDE: {})", self.rally_count, self.longest_rally);
        draw_text(&r_str, ox + 35.0, oy + 20.0, 13.0, YELLOW_ACCENT);

        let m_str = if self.mode == GameMode::SinglePlayer {
            format!("1P vs IA [{}]", self.difficulty.name())
        } else {
            "2 JOGADORES LOCAL".to_string()
        };
        draw_text(&m_str, ox + 580.0, oy + 20.0, 13.0, ICE_BLUE);

        // Indicador de Cooldown Smash P1
        if self.p1_smash_cooldown <= 0.0 {
            draw_text("SMASH [ESPAÇO] PRONTO", ox + 42.0, oy + 586.0, 11.0, YELLOW_ACCENT);
        }

        // Indicador de Cooldown Smash P2
        if self.mode == GameMode::TwoPlayersLocal && self.p2_smash_cooldown <= 0.0 {
            draw_text("SMASH [ENTER] PRONTO", ox + 650.0, oy + 586.0, 11.0, CORAL);
        }

        // =====================================================================
        // TELAS DE ESTADO
        // =====================================================================
        if self.state == MatchState::ReadyToServe {
            let title = "PONG SMASH NEON";
            let t_dim = measure_text(title, None, 28, 1.0);
            draw_text(title, ox + (800.0 - t_dim.width) * 0.5, oy + 200.0, 28.0, YELLOW_ACCENT);

            let prompt = "Pressione [ESPAÇO] para Sacar a Bola";
            let p_dim = measure_text(prompt, None, 16, 1.0);
            draw_text(prompt, ox + (800.0 - p_dim.width) * 0.5, oy + 240.0, 16.0, TEXT_PRIMARY);

            let mode_hint = "Modo de Jogo: [TAB / M] 1P vs IA ou 2P Local";
            let m_dim = measure_text(mode_hint, None, 14, 1.0);
            draw_text(mode_hint, ox + (800.0 - m_dim.width) * 0.5, oy + 370.0, 14.0, ICE_BLUE);

            let diff_hint = "Dificuldade IA: [1] Casual  [2] Competitivo  [3] Cyber Master";
            let d_dim = measure_text(diff_hint, None, 13, 1.0);
            draw_text(diff_hint, ox + (800.0 - d_dim.width) * 0.5, oy + 400.0, 13.0, TEXT_MUTED);
        }

        if let MatchState::MatchOver { champion, timer: _ } = &self.state {
            draw_rectangle(ox, oy, 800.0, 600.0, Color::new(0.04, 0.05, 0.09, 0.88));

            let box_w = 520.0;
            let box_h = 240.0;
            let bx = ox + (800.0 - box_w) * 0.5;
            let by = oy + (600.0 - box_h) * 0.5;
            let champ_col = if *champion == 1 { ICE_BLUE } else { CORAL };
            draw_card_rect(bx, by, box_w, box_h, SURFACE_COLOR, champ_col);

            let title = if *champion == 1 { "JOGADOR 1 É O CAMPEÃO!" } else if self.mode == GameMode::SinglePlayer { "A IA VENCEU A PARTIDA!" } else { "JOGADOR 2 É O CAMPEÃO!" };
            let t_dim = measure_text(title, None, 28, 1.0);
            draw_text(title, bx + (box_w - t_dim.width) * 0.5, by + 50.0, 28.0, champ_col);

            let score_final = format!("Placar Final: {} x {}", self.p1_score, self.p2_score);
            let sf_dim = measure_text(&score_final, None, 20, 1.0);
            draw_text(&score_final, bx + (box_w - sf_dim.width) * 0.5, by + 95.0, 20.0, YELLOW_ACCENT);

            let rally_stat = format!("Maior Rali da Partida: {} rebatidas  |  Smashes P1: {}", self.longest_rally, self.smashes_p1);
            let rs_dim = measure_text(&rally_stat, None, 13, 1.0);
            draw_text(&rally_stat, bx + (box_w - rs_dim.width) * 0.5, by + 135.0, 13.0, TEXT_PRIMARY);

            let r_prompt = "Pressione [R] para Nova Partida ou [ESC] para Voltar";
            let r_dim = measure_text(r_prompt, None, 13, 1.0);
            draw_text(r_prompt, bx + (box_w - r_dim.width) * 0.5, by + 190.0, 13.0, TEXT_MUTED);
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }

    fn get_score(&self) -> u32 {
        self.total_score
    }

    fn is_game_over(&self) -> bool {
        self.game_over
    }

    fn get_title(&self) -> &str {
        "Pong Smash Neon"
    }
}
