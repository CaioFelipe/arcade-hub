use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

// =========================================================================
// 1. ENUMS, CONFIGURAÇÕES E BIOMAS
// =========================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Difficulty {
    Cadet,  // Fácil
    Pilot,  // Normal
    Ace,    // Difícil
}

impl Difficulty {
    pub fn name(&self) -> &'static str {
        match self {
            Difficulty::Cadet => "CADETE (FÁCIL)",
            Difficulty::Pilot => "PILOTO (NORMAL)",
            Difficulty::Ace => "ÁS DO ESPAÇO (DIFÍCIL)",
        }
    }

    pub fn scroll_speed(&self) -> f32 {
        match self {
            Difficulty::Cadet => 3.2,
            Difficulty::Pilot => 3.8,
            Difficulty::Ace => 4.6,
        }
    }

    pub fn gap_size(&self) -> f32 {
        match self {
            Difficulty::Cadet => 165.0,
            Difficulty::Pilot => 140.0,
            Difficulty::Ace => 120.0,
        }
    }

    pub fn gravity(&self) -> f32 {
        match self {
            Difficulty::Cadet => 0.33,
            Difficulty::Pilot => 0.39,
            Difficulty::Ace => 0.45,
        }
    }

    pub fn flap_force(&self) -> f32 {
        match self {
            Difficulty::Cadet => -7.2,
            Difficulty::Pilot => -7.8,
            Difficulty::Ace => -8.4,
        }
    }

    pub fn score_multiplier(&self) -> f32 {
        match self {
            Difficulty::Cadet => 1.0,
            Difficulty::Pilot => 1.5,
            Difficulty::Ace => 2.5,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Biome {
    CyberCity,       // 0 a 14 canos
    PlasmaForge,     // 15 a 34 canos
    OrbitalStation,  // 35 a 59 canos
    QuantumHyperspace,// 60+ canos
}

impl Biome {
    pub fn name(&self) -> &'static str {
        match self {
            Biome::CyberCity => "CIDADE CYBERPUNK",
            Biome::PlasmaForge => "FORJA DE PLASMA",
            Biome::OrbitalStation => "ESTAÇÃO ORBITAL",
            Biome::QuantumHyperspace => "HIPERESPAÇO QUÂNTICO",
        }
    }

    pub fn bg_color(&self) -> Color {
        match self {
            Biome::CyberCity => Color::new(0.035, 0.047, 0.078, 1.0),
            Biome::PlasmaForge => Color::new(0.078, 0.035, 0.042, 1.0),
            Biome::OrbitalStation => Color::new(0.025, 0.035, 0.065, 1.0),
            Biome::QuantumHyperspace => Color::new(0.045, 0.025, 0.075, 1.0),
        }
    }

    pub fn pipe_accent(&self) -> Color {
        match self {
            Biome::CyberCity => EMERALD,
            Biome::PlasmaForge => CORAL,
            Biome::OrbitalStation => ICE_BLUE,
            Biome::QuantumHyperspace => Color::new(0.85, 0.40, 0.98, 1.0),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PowerUpType {
    Shield,   // Escudo de energia (aguenta 1 colisão fatal)
    Magnet,   // Ímã quântico de moedas e cristais
    Turbo,    // Hiperpropulsor invencível temporário
    SlowMo,   // Câmera lenta gravitacional
    Shrink,   // Mini-jetpack para frestas apertadas
}

impl PowerUpType {
    pub fn label(&self) -> &'static str {
        match self {
            PowerUpType::Shield => "ESCUDO DE FORÇA",
            PowerUpType::Magnet => "ÍMÃ QUÂNTICO",
            PowerUpType::Turbo => "TURBO PROPULSOR",
            PowerUpType::SlowMo => "CÂMERA LENTA",
            PowerUpType::Shrink => "MINI-JETPACK",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            PowerUpType::Shield => ICE_BLUE,
            PowerUpType::Magnet => YELLOW_ACCENT,
            PowerUpType::Turbo => CORAL,
            PowerUpType::SlowMo => Color::new(0.70, 0.40, 0.95, 1.0),
            PowerUpType::Shrink => EMERALD,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            PowerUpType::Shield => "S",
            PowerUpType::Magnet => "M",
            PowerUpType::Turbo => "T",
            PowerUpType::SlowMo => "C",
            PowerUpType::Shrink => "N",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CollectibleKind {
    Coin,                    // Moeda de ouro
    Crystal,                 // Cristal prismático de alto valor
    PowerUp(PowerUpType),    // Cápsula de poder
}

// =========================================================================
// 2. ESTRUTURAS DE ELEMENTOS DO MUNDO
// =========================================================================

#[derive(Clone)]
struct PipeObstacle {
    x: f32,
    gap_y: f32,
    gap_size: f32,
    scored: bool,
    is_moving: bool,
    move_dir: f32,
    move_speed: f32,
    is_laser_barrier: bool,
    laser_active: bool,
}

#[derive(Clone)]
struct Collectible {
    x: f32,
    y: f32,
    kind: CollectibleKind,
    alive: bool,
    float_phase: f32,
}

struct JetpackParticle {
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

struct Star {
    x: f32,
    y: f32,
    spd: f32,
    size: f32,
    brightness: f32,
}

struct Building {
    x: f32,
    w: f32,
    h: f32,
    color: Color,
    lights: Vec<(f32, f32)>,
}

#[derive(PartialEq, Debug)]
enum GameState {
    Ready,
    Playing,
    GameOver { timer: f32 },
}

// =========================================================================
// 3. ESTRUTURA PRINCIPAL DO FLAPPY JETPACK
// =========================================================================

pub struct FlappyGame {
    state: GameState,
    difficulty: Difficulty,
    biome: Biome,

    // Piloto
    bird_y: f32,
    velocity: f32,
    rotation: f32,
    shield_hp: i32,
    scale: f32,

    // Coleções
    pipes: Vec<PipeObstacle>,
    collectibles: Vec<Collectible>,
    particles: Vec<JetpackParticle>,
    float_texts: Vec<FloatText>,

    // Timers de power-ups ativos
    magnet_timer: f32,
    turbo_timer: f32,
    slowmo_timer: f32,
    shrink_timer: f32,

    // Cenário e Parallax
    stars: Vec<Star>,
    buildings: Vec<Building>,

    // Métricas e pontuações
    score: u32,
    pipes_passed: u32,
    coins_collected: u32,
    time: f32,
    spawn_timer: f32,
    screen_shake: f32,
    combo: u32,
    game_over: bool,
}

impl FlappyGame {
    pub fn new() -> Self {
        let diff = Difficulty::Pilot;

        // Gera estrelas de fundo
        let mut stars = Vec::with_capacity(75);
        for _ in 0..75 {
            stars.push(Star {
                x: rand::gen_range(0.0, 800.0),
                y: rand::gen_range(20.0, 580.0),
                spd: rand::gen_range(0.3, 1.4),
                size: rand::gen_range(1.0, 2.5),
                brightness: rand::gen_range(0.4, 1.0),
            });
        }

        // Gera silhueta de edifícios futuristas para o parallax urbano
        let mut buildings = Vec::new();
        let mut bx = 0.0;
        while bx < 900.0 {
            let bw = rand::gen_range(45.0, 90.0);
            let bh = rand::gen_range(80.0, 220.0);
            let mut lights = Vec::new();
            for lx in (10..bw as i32 - 10).step_by(16) {
                for ly in (16..bh as i32 - 10).step_by(22) {
                    if rand::gen_range(0, 3) > 0 {
                        lights.push((lx as f32, ly as f32));
                    }
                }
            }
            buildings.push(Building {
                x: bx,
                w: bw,
                h: bh,
                color: Color::new(0.06, 0.08, 0.13, 1.0),
                lights,
            });
            bx += bw + rand::gen_range(6.0, 18.0);
        }

        let mut game = Self {
            state: GameState::Ready,
            difficulty: diff,
            biome: Biome::CyberCity,

            bird_y: 290.0,
            velocity: 0.0,
            rotation: 0.0,
            shield_hp: if diff == Difficulty::Cadet { 1 } else { 0 },
            scale: 1.0,

            pipes: Vec::new(),
            collectibles: Vec::new(),
            particles: Vec::new(),
            float_texts: Vec::new(),

            magnet_timer: 0.0,
            turbo_timer: 0.0,
            slowmo_timer: 0.0,
            shrink_timer: 0.0,

            stars,
            buildings,

            score: 0,
            pipes_passed: 0,
            coins_collected: 0,
            time: 0.0,
            spawn_timer: 0.0,
            screen_shake: 0.0,
            combo: 0,
            game_over: false,
        };

        game.spawn_obstacle_pair();
        game
    }

    fn set_difficulty(&mut self, diff: Difficulty) {
        self.difficulty = diff;
        self.shield_hp = if diff == Difficulty::Cadet { 1 } else { 0 };
        self.float_texts.push(FloatText {
            x: 400.0,
            y: 460.0,
            text: format!("DIFICULDADE: {}", diff.name()),
            color: YELLOW_ACCENT,
            alpha: 1.0,
        });
    }

    fn update_biome(&mut self) {
        let old_biome = self.biome;
        self.biome = if self.pipes_passed < 15 {
            Biome::CyberCity
        } else if self.pipes_passed < 35 {
            Biome::PlasmaForge
        } else if self.pipes_passed < 60 {
            Biome::OrbitalStation
        } else {
            Biome::QuantumHyperspace
        };

        if self.biome != old_biome {
            self.screen_shake = 4.0;
            self.float_texts.push(FloatText {
                x: 400.0,
                y: 120.0,
                text: format!("ENTRANDO: {}!", self.biome.name()),
                color: self.biome.pipe_accent(),
                alpha: 1.2,
            });
            // Chuva de partículas cósmicas de transição
            for _ in 0..30 {
                self.particles.push(JetpackParticle {
                    x: rand::gen_range(100.0, 700.0),
                    y: rand::gen_range(50.0, 550.0),
                    vx: rand::gen_range(-4.0, 4.0),
                    vy: rand::gen_range(-4.0, 4.0),
                    color: self.biome.pipe_accent(),
                    life: 0.7,
                    max_life: 0.7,
                    size: rand::gen_range(3.0, 6.0),
                });
            }
        }
    }

    fn flap(&mut self) {
        self.velocity = self.difficulty.flap_force();

        // Efeito de propulsão neon dupla
        let bx = 220.0;
        let by = self.bird_y;

        for _ in 0..10 {
            self.particles.push(JetpackParticle {
                x: bx - 16.0,
                y: by + rand::gen_range(-3.0, 5.0),
                vx: rand::gen_range(-5.5, -2.5),
                vy: rand::gen_range(-1.8, 2.5),
                color: if rand::gen_range(0, 2) == 0 { YELLOW_ACCENT } else { CORAL },
                life: 0.40,
                max_life: 0.40,
                size: rand::gen_range(2.5, 4.5),
            });
        }
    }

    fn spawn_obstacle_pair(&mut self) {
        let gap_sz = self.difficulty.gap_size();
        let gap_y = rand::gen_range(150.0, 430.0);

        // Canos móveis a partir de certos marcos
        let is_moving = self.pipes_passed >= 6 && rand::gen_range(0, 100) < 40;
        let is_laser = self.pipes_passed >= 18 && rand::gen_range(0, 100) < 28;

        let spawn_x = 860.0;

        self.pipes.push(PipeObstacle {
            x: spawn_x,
            gap_y,
            gap_size: gap_sz,
            scored: false,
            is_moving,
            move_dir: if rand::gen_range(0, 2) == 0 { 1.0 } else { -1.0 },
            move_speed: rand::gen_range(0.8, 1.8),
            is_laser_barrier: is_laser,
            laser_active: true,
        });

        // Gera Moeda ou Cristal no vão
        let roll_collectible = rand::gen_range(0, 100);
        if roll_collectible < 40 {
            // Moeda clássica
            self.collectibles.push(Collectible {
                x: spawn_x + 30.0,
                y: gap_y,
                kind: CollectibleKind::Coin,
                alive: true,
                float_phase: rand::gen_range(0.0, 6.28),
            });
        } else if roll_collectible < 62 {
            // Cristal energético prismático
            self.collectibles.push(Collectible {
                x: spawn_x + 30.0,
                y: gap_y,
                kind: CollectibleKind::Crystal,
                alive: true,
                float_phase: rand::gen_range(0.0, 6.28),
            });
        } else if roll_collectible < 78 {
            // Cápsula de Power-up
            let pup_type = match rand::gen_range(0, 5) {
                0 => PowerUpType::Shield,
                1 => PowerUpType::Magnet,
                2 => PowerUpType::Turbo,
                3 => PowerUpType::SlowMo,
                _ => PowerUpType::Shrink,
            };
            self.collectibles.push(Collectible {
                x: spawn_x + 30.0,
                y: gap_y,
                kind: CollectibleKind::PowerUp(pup_type),
                alive: true,
                float_phase: rand::gen_range(0.0, 6.28),
            });
        }
    }

    fn apply_powerup(&mut self, kind: PowerUpType) {
        match kind {
            PowerUpType::Shield => {
                self.shield_hp = (self.shield_hp + 1).min(2);
                self.float_texts.push(FloatText {
                    x: 220.0,
                    y: self.bird_y - 30.0,
                    text: "+ESCUDO DE FORÇA!".to_string(),
                    color: ICE_BLUE,
                    alpha: 1.0,
                });
            }
            PowerUpType::Magnet => {
                self.magnet_timer = 11.0;
                self.float_texts.push(FloatText {
                    x: 220.0,
                    y: self.bird_y - 30.0,
                    text: "+ÍMÃ QUÂNTICO!".to_string(),
                    color: YELLOW_ACCENT,
                    alpha: 1.0,
                });
            }
            PowerUpType::Turbo => {
                self.turbo_timer = 4.5;
                self.screen_shake = 5.0;
                self.float_texts.push(FloatText {
                    x: 220.0,
                    y: self.bird_y - 30.0,
                    text: "+TURBO BOOST INVENCÍVEL!".to_string(),
                    color: CORAL,
                    alpha: 1.0,
                });
            }
            PowerUpType::SlowMo => {
                self.slowmo_timer = 8.0;
                self.float_texts.push(FloatText {
                    x: 220.0,
                    y: self.bird_y - 30.0,
                    text: "+CÂMERA LENTA!".to_string(),
                    color: Color::new(0.70, 0.40, 0.95, 1.0),
                    alpha: 1.0,
                });
            }
            PowerUpType::Shrink => {
                self.shrink_timer = 9.0;
                self.scale = 0.60;
                self.float_texts.push(FloatText {
                    x: 220.0,
                    y: self.bird_y - 30.0,
                    text: "+MINI-JETPACK!".to_string(),
                    color: EMERALD,
                    alpha: 1.0,
                });
            }
        }
    }

    fn trigger_crash(&mut self) {
        self.screen_shake = 9.0;
        self.state = GameState::GameOver { timer: 0.0 };
        self.game_over = true;

        let bx = 220.0;
        let by = self.bird_y;

        // Explosão de estilhaços e fumaça
        for _ in 0..35 {
            let spd = rand::gen_range(2.0, 7.5);
            let ang: f32 = rand::gen_range(0.0, 6.28);
            self.particles.push(JetpackParticle {
                x: bx,
                y: by,
                vx: ang.cos() * spd,
                vy: ang.sin() * spd,
                color: if rand::gen_range(0, 2) == 0 { CORAL } else { YELLOW_ACCENT },
                life: 0.85,
                max_life: 0.85,
                size: rand::gen_range(3.0, 6.0),
            });
        }
    }
}

// =========================================================================
// 4. IMPLEMENTAÇÃO DA TRAIT ARCADETRAIT
// =========================================================================

impl ArcadeGame for FlappyGame {
    fn update(&mut self) {
        let dt = get_frame_time();
        self.time += dt;

        // Amortecimento do tremor de tela
        if self.screen_shake > 0.0 {
            self.screen_shake = (self.screen_shake - dt * 16.0).max(0.0);
        }

        // Fator de lentidão gravitacional
        let time_scale = if self.slowmo_timer > 0.0 { 0.55 } else { 1.0 };
        let world_speed = if self.turbo_timer > 0.0 {
            self.difficulty.scroll_speed() * 1.8
        } else {
            self.difficulty.scroll_speed() * time_scale
        };

        // Atualização dos timers de power-ups
        if self.magnet_timer > 0.0 {
            self.magnet_timer -= dt;
        }
        if self.turbo_timer > 0.0 {
            self.turbo_timer -= dt;
        }
        if self.slowmo_timer > 0.0 {
            self.slowmo_timer -= dt;
        }
        if self.shrink_timer > 0.0 {
            self.shrink_timer -= dt;
            if self.shrink_timer <= 0.0 {
                self.scale = 1.0;
            }
        }

        // Atualização de partículas
        self.particles.retain_mut(|p| {
            p.x += p.vx;
            p.y += p.vy;
            p.life -= dt;
            p.life > 0.0
        });

        // Atualização de textos flutuantes
        self.float_texts.retain_mut(|ft| {
            ft.y -= 0.8;
            ft.alpha -= dt * 1.2;
            ft.alpha > 0.0
        });

        // Rolagem das estrelas de fundo (Parallax distante)
        for s in &mut self.stars {
            s.x -= s.spd * world_speed * 0.25;
            if s.x < 0.0 {
                s.x = 800.0;
                s.y = rand::gen_range(20.0, 580.0);
            }
        }

        // Rolagem dos edifícios (Parallax médio)
        for b in &mut self.buildings {
            b.x -= world_speed * 0.45;
        }
        if let Some(first) = self.buildings.first() {
            if first.x + first.w < 0.0 {
                let last_x = self.buildings.last().map(|b| b.x + b.w).unwrap_or(800.0);
                let mut b = self.buildings.remove(0);
                b.x = last_x + rand::gen_range(8.0, 20.0);
                self.buildings.push(b);
            }
        }

        // =====================================================================
        // TRATAMENTO DOS ESTADOS (READY, PLAYING, GAME OVER)
        // =====================================================================
        match &mut self.state {
            GameState::Ready => {
                // Flutuação suave em senoide no menu
                self.bird_y = 290.0 + (self.time * 4.0).sin() * 8.0;
                self.rotation = 0.0;

                // Seleção de dificuldade
                if is_key_pressed(KeyCode::Key1) {
                    self.set_difficulty(Difficulty::Cadet);
                } else if is_key_pressed(KeyCode::Key2) {
                    self.set_difficulty(Difficulty::Pilot);
                } else if is_key_pressed(KeyCode::Key3) {
                    self.set_difficulty(Difficulty::Ace);
                }

                // Decolagem
                if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up) || is_mouse_button_pressed(MouseButton::Left) {
                    self.state = GameState::Playing;
                    self.flap();
                }
                return;
            }
            GameState::GameOver { timer } => {
                *timer += dt;
                if is_key_pressed(KeyCode::R) {
                    *self = Self::new();
                }
                return;
            }
            GameState::Playing => {}
        }

        if self.game_over {
            return;
        }

        // =====================================================================
        // FÍSICA DO PILOTO
        // =====================================================================
        if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Up) || is_mouse_button_pressed(MouseButton::Left) {
            self.flap();
        }

        // Gravidade e velocidade vertical
        let grav = self.difficulty.gravity() * time_scale;
        self.velocity += grav;
        self.bird_y += self.velocity * time_scale;

        // Rotação aerodinâmica suave
        self.rotation = (self.velocity * 0.06).clamp(-0.45, 0.75);

        // Teto e Chão
        if self.bird_y <= 24.0 {
            self.bird_y = 24.0;
            self.velocity = 0.0;
        }
        if self.bird_y >= 565.0 {
            self.trigger_crash();
            return;
        }

        // =====================================================================
        // SPAWN E MOVIMENTO DOS CANOS E OBSTÁCULOS
        // =====================================================================
        let spawn_interval = (95.0 / (world_speed / 3.8)).max(65.0);
        self.spawn_timer += 1.0;
        if self.spawn_timer >= spawn_interval {
            self.spawn_timer = 0.0;
            self.spawn_obstacle_pair();
        }

        let bird_x = 220.0;
        let bird_hit_radius = 14.0 * self.scale;

        let mut passed_pipe = false;
        for pipe in &mut self.pipes {
            pipe.x -= world_speed;

            // Oscilação vertical dos canos móveis
            if pipe.is_moving {
                pipe.gap_y += pipe.move_dir * pipe.move_speed;
                if pipe.gap_y <= 130.0 {
                    pipe.gap_y = 130.0;
                    pipe.move_dir = 1.0;
                } else if pipe.gap_y >= 450.0 {
                    pipe.gap_y = 450.0;
                    pipe.move_dir = -1.0;
                }
            }

            let top_h = pipe.gap_y - pipe.gap_size * 0.5;
            let bot_y = pipe.gap_y + pipe.gap_size * 0.5;

            // Verificação de Colisão com o Cano Superior e Inferior
            let in_pipe_x = bird_x + bird_hit_radius >= pipe.x && bird_x - bird_hit_radius <= pipe.x + 58.0;
            let hits_top = self.bird_y - bird_hit_radius <= top_h;
            let hits_bottom = self.bird_y + bird_hit_radius >= bot_y;

            if in_pipe_x && (hits_top || hits_bottom) {
                if self.turbo_timer > 0.0 {
                    // Turbo ignora e destrói o obstáculo!
                    pipe.x = -200.0;
                    self.screen_shake = 4.0;
                    self.score += 50;
                } else if self.shield_hp > 0 {
                    // Escudo de energia absorve a colisão
                    self.shield_hp -= 1;
                    self.screen_shake = 5.0;
                    pipe.x = -200.0; // Desintegra o cano
                    self.float_texts.push(FloatText {
                        x: bird_x,
                        y: self.bird_y - 25.0,
                        text: "ESCUDO ABSORVEU!".to_string(),
                        color: ICE_BLUE,
                        alpha: 1.0,
                    });
                } else {
                    self.trigger_crash();
                    return;
                }
            }

            // Pontuação por ultrapassar o cano
            if !pipe.scored && pipe.x < bird_x {
                pipe.scored = true;
                self.pipes_passed += 1;
                self.combo += 1;
                passed_pipe = true;

                let combo_mult = 1.0 + (self.combo.saturating_sub(1) as f32) * 0.15;
                let earned = (10.0 * self.difficulty.score_multiplier() * combo_mult) as u32;
                self.score += earned;

                self.float_texts.push(FloatText {
                    x: bird_x + 10.0,
                    y: self.bird_y - 15.0,
                    text: format!("+{}", earned),
                    color: YELLOW_ACCENT,
                    alpha: 0.9,
                });
            }
        }

        if passed_pipe {
            self.update_biome();
        }

        // =====================================================================
        // COLETA DE MOEDAS, CRISTAIS E POWER-UPS
        // =====================================================================
        let magnet_active = self.magnet_timer > 0.0;
        let mut collected_powerups = Vec::new();

        for item in &mut self.collectibles {
            if !item.alive {
                continue;
            }
            item.x -= world_speed;
            item.float_phase += dt * 3.0;
            let visual_y = item.y + item.float_phase.sin() * 5.0;

            // Atração magnética quântica
            if magnet_active {
                let dx = bird_x - item.x;
                let dy = self.bird_y - visual_y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < 240.0 && dist > 1.0 {
                    item.x += (dx / dist) * 7.5;
                    item.y += (dy / dist) * 7.5;
                }
            }

            // Colisão com o piloto
            let dx = bird_x - item.x;
            let dy = self.bird_y - visual_y;
            let dist_sq = dx * dx + dy * dy;

            if dist_sq < (26.0 * 26.0) {
                item.alive = false;

                match item.kind {
                    CollectibleKind::Coin => {
                        self.coins_collected += 1;
                        let pts = (25.0 * self.difficulty.score_multiplier()) as u32;
                        self.score += pts;
                        self.float_texts.push(FloatText {
                            x: item.x,
                            y: visual_y,
                            text: format!("+{}", pts),
                            color: YELLOW_ACCENT,
                            alpha: 1.0,
                        });
                        for _ in 0..8 {
                            self.particles.push(JetpackParticle {
                                x: item.x,
                                y: visual_y,
                                vx: rand::gen_range(-2.5, 2.5),
                                vy: rand::gen_range(-2.5, 2.5),
                                color: YELLOW_ACCENT,
                                life: 0.4,
                                max_life: 0.4,
                                size: 3.0,
                            });
                        }
                    }
                    CollectibleKind::Crystal => {
                        self.coins_collected += 2;
                        let pts = (75.0 * self.difficulty.score_multiplier()) as u32;
                        self.score += pts;
                        self.float_texts.push(FloatText {
                            x: item.x,
                            y: visual_y,
                            text: format!("+{} CRISTAL!", pts),
                            color: EMERALD,
                            alpha: 1.0,
                        });
                        for _ in 0..12 {
                            self.particles.push(JetpackParticle {
                                x: item.x,
                                y: visual_y,
                                vx: rand::gen_range(-3.5, 3.5),
                                vy: rand::gen_range(-3.5, 3.5),
                                color: EMERALD,
                                life: 0.5,
                                max_life: 0.5,
                                size: 3.5,
                            });
                        }
                    }
                    CollectibleKind::PowerUp(kind) => {
                        collected_powerups.push(kind);
                    }
                }
            }
        }

        for p_kind in collected_powerups {
            self.apply_powerup(p_kind);
        }

        // Limpeza de obstáculos e itens fora da tela
        self.pipes.retain(|p| p.x > -80.0);
        self.collectibles.retain(|c| c.alive && c.x > -60.0);
    }

    // =========================================================================
    // 5. RENDERIZAÇÃO GRÁFICA & AMBIENTAÇÃO NEON
    // =========================================================================

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let shake_x = if self.screen_shake > 0.0 { rand::gen_range(-self.screen_shake, self.screen_shake) } else { 0.0 };
        let shake_y = if self.screen_shake > 0.0 { rand::gen_range(-self.screen_shake, self.screen_shake) } else { 0.0 };

        let ox = (screen_w - 800.0) * 0.5 + shake_x;
        let oy = (screen_h - 600.0) * 0.5 + 24.0 + shake_y;

        // Fundo dinâmico baseado no Bioma atual
        draw_rectangle(ox, oy, 800.0, 600.0, self.biome.bg_color());
        draw_rectangle_lines(ox, oy, 800.0, 600.0, 1.5, BORDER_COLOR);

        // Estrelas / Poeira Estelar do Parallax Distante
        for s in &self.stars {
            let col = Color::new(1.0, 1.0, 1.0, s.brightness * 0.7);
            draw_circle(ox + s.x, oy + s.y, s.size, col);
        }

        // Silhueta dos edifícios do Parallax Médio
        for b in &self.buildings {
            let by = oy + 600.0 - b.h;
            draw_rectangle(ox + b.x, by, b.w, b.h, b.color);
            draw_rectangle_lines(ox + b.x, by, b.w, b.h, 1.0, Color::new(0.12, 0.16, 0.24, 0.6));
            // Janelas holográficas acesas
            for &(lx, ly) in &b.lights {
                draw_circle(ox + b.x + lx, by + ly, 1.5, Color::new(0.38, 0.65, 0.98, 0.25));
            }
        }

        // Grade de coordenadas no chão/teto
        draw_line(ox, oy + 24.0, ox + 800.0, oy + 24.0, 1.5, BORDER_COLOR);
        draw_line(ox, oy + 576.0, ox + 800.0, oy + 576.0, 2.0, self.biome.pipe_accent());

        // Partículas
        for p in &self.particles {
            let a = (p.life / p.max_life).clamp(0.0, 1.0);
            let mut c = p.color;
            c.a = a;
            draw_circle(ox + p.x, oy + p.y, p.size * a, c);
        }

        // =====================================================================
        // CANOS E OBSTÁCULOS FUTURISTAS
        // =====================================================================
        for pipe in &self.pipes {
            let px = ox + pipe.x;
            let top_h = pipe.gap_y - pipe.gap_size * 0.5;
            let bot_y = pipe.gap_y + pipe.gap_size * 0.5;
            let pipe_w = 58.0;
            let accent_col = self.biome.pipe_accent();

            // Cano Superior (Corpo metálico escuro + Borda Neon)
            draw_rectangle(px, oy + 24.0, pipe_w, top_h - 24.0, SURFACE_COLOR);
            draw_rectangle(px + 4.0, oy + 24.0, 4.0, top_h - 24.0, Color::new(1.0, 1.0, 1.0, 0.12));
            draw_rectangle_lines(px, oy + 24.0, pipe_w, top_h - 24.0, 1.5, accent_col);

            // Bocal do Cano Superior
            let cap_h = 20.0;
            draw_rectangle(px - 5.0, oy + top_h - cap_h, pipe_w + 10.0, cap_h, SURFACE_HOVER);
            draw_rectangle_lines(px - 5.0, oy + top_h - cap_h, pipe_w + 10.0, cap_h, 1.5, accent_col);
            draw_circle(px + pipe_w * 0.5, oy + top_h - cap_h * 0.5, 3.5, accent_col);

            // Cano Inferior (Corpo)
            let bot_h = 576.0 - bot_y;
            draw_rectangle(px, oy + bot_y, pipe_w, bot_h, SURFACE_COLOR);
            draw_rectangle(px + 4.0, oy + bot_y, 4.0, bot_h, Color::new(1.0, 1.0, 1.0, 0.12));
            draw_rectangle_lines(px, oy + bot_y, pipe_w, bot_h, 1.5, accent_col);

            // Bocal do Cano Inferior
            draw_rectangle(px - 5.0, oy + bot_y, pipe_w + 10.0, cap_h, SURFACE_HOVER);
            draw_rectangle_lines(px - 5.0, oy + bot_y, pipe_w + 10.0, cap_h, 1.5, accent_col);
            draw_circle(px + pipe_w * 0.5, oy + bot_y + cap_h * 0.5, 3.5, accent_col);

            // Se for barreira laser pulsante entre bocais
            if pipe.is_laser_barrier {
                let laser_alpha = 0.5 + 0.5 * (self.time * 8.0).sin();
                draw_line(
                    px + pipe_w * 0.5,
                    oy + top_h,
                    px + pipe_w * 0.5,
                    oy + bot_y,
                    2.5,
                    Color::new(0.95, 0.25, 0.25, laser_alpha),
                );
            }
        }

        // =====================================================================
        // MOEDAS, CRISTAIS E POWER-UPS FLUTUANTES
        // =====================================================================
        for item in &self.collectibles {
            if !item.alive {
                continue;
            }
            let ix = ox + item.x;
            let iy = oy + item.y + item.float_phase.sin() * 5.0;

            match item.kind {
                CollectibleKind::Coin => {
                    // Moeda de ouro com anel brilhante
                    draw_circle(ix, iy, 11.0, YELLOW_ACCENT);
                    draw_circle(ix, iy, 7.5, Color::new(0.85, 0.65, 0.05, 1.0));
                    draw_circle(ix - 2.0, iy - 2.0, 3.0, TEXT_PRIMARY);
                }
                CollectibleKind::Crystal => {
                    // Cristal esmeralda/safira em losango
                    draw_triangle(
                        Vec2::new(ix, iy - 12.0),
                        Vec2::new(ix + 9.0, iy),
                        Vec2::new(ix - 9.0, iy),
                        EMERALD,
                    );
                    draw_triangle(
                        Vec2::new(ix, iy + 12.0),
                        Vec2::new(ix + 9.0, iy),
                        Vec2::new(ix - 9.0, iy),
                        Color::new(0.04, 0.55, 0.38, 1.0),
                    );
                    draw_circle(ix, iy, 2.5, TEXT_PRIMARY);
                }
                CollectibleKind::PowerUp(kind) => {
                    // Cápsula holográfica de poder
                    let col = kind.color();
                    draw_circle(ix, iy, 13.0, SURFACE_COLOR);
                    draw_circle_lines(ix, iy, 13.0, 2.0, col);
                    draw_text(kind.icon(), ix - 5.0, iy + 5.0, 15.0, col);
                }
            }
        }

        // =====================================================================
        // PILOTO JETPACK ESTILIZADO
        // =====================================================================
        let bx = ox + 220.0;
        let by = oy + self.bird_y;
        let s = self.scale;

        // Efeito de Turbo Boost Incandescente
        if self.turbo_timer > 0.0 {
            draw_circle(bx, by, 26.0 * s, Color::new(1.0, 0.45, 0.1, 0.35));
        }

        // Escudo de Força Protetor
        if self.shield_hp > 0 {
            let shield_pulse = 0.65 + 0.35 * (self.time * 6.0).sin();
            draw_circle_lines(bx, by, 24.0 * s, 2.0, Color::new(0.38, 0.65, 0.98, shield_pulse));
            draw_circle(bx, by, 22.0 * s, Color::new(0.38, 0.65, 0.98, 0.15));
        }

        // Fogo da Propulsão
        if self.velocity < 1.0 {
            let flame_len = rand::gen_range(16.0, 26.0) * s;
            draw_triangle(
                Vec2::new(bx - 14.0 * s, by - 5.0 * s),
                Vec2::new(bx - 14.0 * s, by + 5.0 * s),
                Vec2::new(bx - (14.0 * s + flame_len), by),
                YELLOW_ACCENT,
            );
            draw_triangle(
                Vec2::new(bx - 14.0 * s, by - 2.5 * s),
                Vec2::new(bx - 14.0 * s, by + 2.5 * s),
                Vec2::new(bx - (14.0 * s + flame_len * 0.6), by),
                CORAL,
            );
        }

        // Mochila a jato (Cilindro Metálico com friso neon)
        draw_rectangle(bx - 15.0 * s, by - 8.0 * s, 8.0 * s, 16.0 * s, Color::new(0.25, 0.32, 0.44, 1.0));
        draw_rectangle(bx - 13.0 * s, by - 4.0 * s, 4.0 * s, 8.0 * s, ICE_BLUE);

        // Corpo do Piloto / Traje Cibernético
        draw_circle(bx, by, 14.0 * s, COBALT_ACCENT);
        draw_circle_lines(bx, by, 14.0 * s, 1.5, ICE_BLUE);

        // Capacete e Visor Reflexivo
        draw_circle(bx + 4.0 * s, by - 2.0 * s, 7.0 * s, SURFACE_COLOR);
        draw_circle(bx + 5.0 * s, by - 2.0 * s, 5.0 * s, YELLOW_ACCENT);
        draw_circle(bx + 6.0 * s, by - 3.0 * s, 2.0 * s, TEXT_PRIMARY);

        // Textos flutuantes (+25, Bioma, etc.)
        for ft in &self.float_texts {
            let mut col = ft.color;
            col.a = ft.alpha;
            draw_text(&ft.text, ox + ft.x - 16.0, oy + ft.y, 16.0, col);
        }

        // =====================================================================
        // HUD INTERNO DA ARENA
        // =====================================================================
        // Bioma e Dificuldade no Canto Superior Esquerdo
        let b_str = format!("BIOMA: {}", self.biome.name());
        draw_text(&b_str, ox + 35.0, oy + 18.0, 14.0, self.biome.pipe_accent());

        let d_str = format!("[{}] x{:.1}", self.difficulty.name(), self.difficulty.score_multiplier());
        draw_text(&d_str, ox + 220.0, oy + 18.0, 13.0, TEXT_MUTED);

        // Moedas Coletadas
        let coin_str = format!("MOEDAS: {}", self.coins_collected);
        draw_text(&coin_str, ox + 430.0, oy + 18.0, 14.0, YELLOW_ACCENT);

        // Canos Ultrapassados
        let pass_str = format!("CANOS: {}", self.pipes_passed);
        draw_text(&pass_str, ox + 560.0, oy + 18.0, 14.0, TEXT_PRIMARY);

        // Escudo indicador
        if self.shield_hp > 0 {
            draw_card_rect(ox + 670.0, oy + 6.0, 95.0, 18.0, SURFACE_COLOR, ICE_BLUE);
            draw_text("ESCUDO ATIVO", ox + 678.0, oy + 19.0, 11.0, ICE_BLUE);
        }

        // Badges de Power-ups ativos no rodapé da tela
        let mut p_x = ox + 40.0;
        let p_y = oy + 540.0;
        if self.turbo_timer > 0.0 {
            let t_str = format!("TURBO {:.1}s", self.turbo_timer);
            draw_card_rect(p_x, p_y, 85.0, 22.0, SURFACE_COLOR, CORAL);
            draw_text(&t_str, p_x + 8.0, p_y + 16.0, 12.0, CORAL);
            p_x += 95.0;
        }
        if self.magnet_timer > 0.0 {
            let m_str = format!("ÍMÃ {:.1}s", self.magnet_timer);
            draw_card_rect(p_x, p_y, 75.0, 22.0, SURFACE_COLOR, YELLOW_ACCENT);
            draw_text(&m_str, p_x + 8.0, p_y + 16.0, 12.0, YELLOW_ACCENT);
            p_x += 85.0;
        }
        if self.slowmo_timer > 0.0 {
            let s_str = format!("SLOW-MO {:.1}s", self.slowmo_timer);
            draw_card_rect(p_x, p_y, 95.0, 22.0, SURFACE_COLOR, Color::new(0.70, 0.40, 0.95, 1.0));
            draw_text(&s_str, p_x + 8.0, p_y + 16.0, 12.0, Color::new(0.70, 0.40, 0.95, 1.0));
            p_x += 105.0;
        }
        if self.shrink_timer > 0.0 {
            let n_str = format!("MINI {:.1}s", self.shrink_timer);
            draw_card_rect(p_x, p_y, 75.0, 22.0, SURFACE_COLOR, EMERALD);
            draw_text(&n_str, p_x + 8.0, p_y + 16.0, 12.0, EMERALD);
        }

        // =====================================================================
        // TELA DE PREPARAÇÃO (READY)
        // =====================================================================
        if self.state == GameState::Ready {
            let title = "FLAPPY JETPACK ULTRA";
            let t_dim = measure_text(title, None, 28, 1.0);
            draw_text(title, ox + (800.0 - t_dim.width) * 0.5, oy + 210.0, 28.0, YELLOW_ACCENT);

            let prompt = "Pressione [ESPAÇO] ou [CLIQUE] para Acionar os Propulsores";
            let p_dim = measure_text(prompt, None, 16, 1.0);
            draw_text(prompt, ox + (800.0 - p_dim.width) * 0.5, oy + 250.0, 16.0, TEXT_PRIMARY);

            let diff_prompt = "Dificuldade: [1] Cadete  [2] Piloto  [3] Ás do Espaço";
            let d_dim = measure_text(diff_prompt, None, 14, 1.0);
            draw_text(diff_prompt, ox + (800.0 - d_dim.width) * 0.5, oy + 380.0, 14.0, ICE_BLUE);
        }

        // =====================================================================
        // TELA DE GAME OVER (CRASH)
        // =====================================================================
        if let GameState::GameOver { timer: _ } = &self.state {
            draw_rectangle(ox, oy, 800.0, 600.0, Color::new(0.04, 0.05, 0.09, 0.88));

            let box_w = 480.0;
            let box_h = 220.0;
            let bx = ox + (800.0 - box_w) * 0.5;
            let by = oy + (600.0 - box_h) * 0.5;
            draw_card_rect(bx, by, box_w, box_h, SURFACE_COLOR, CORAL);

            let go_title = "PROPULSORES INTERROMPIDOS!";
            let go_dim = measure_text(go_title, None, 24, 1.0);
            draw_text(go_title, bx + (box_w - go_dim.width) * 0.5, by + 45.0, 24.0, CORAL);

            let s_msg = format!("Pontuação Total: {} PONTOS", self.score);
            let s_dim = measure_text(&s_msg, None, 18, 1.0);
            draw_text(&s_msg, bx + (box_w - s_dim.width) * 0.5, by + 85.0, 18.0, YELLOW_ACCENT);

            let stats = format!("Canos Ultrapassados: {}  |  Moedas Coletadas: {}", self.pipes_passed, self.coins_collected);
            let st_dim = measure_text(&stats, None, 13, 1.0);
            draw_text(&stats, bx + (box_w - st_dim.width) * 0.5, by + 120.0, 13.0, TEXT_PRIMARY);

            let b_msg = format!("Último Bioma Alcançado: {}", self.biome.name());
            let b_dim = measure_text(&b_msg, None, 13, 1.0);
            draw_text(&b_msg, bx + (box_w - b_dim.width) * 0.5, by + 145.0, 13.0, self.biome.pipe_accent());

            let r_prompt = "Pressione [R] para Voar Novamente ou [ESC] para Voltar";
            let r_dim = measure_text(r_prompt, None, 13, 1.0);
            draw_text(r_prompt, bx + (box_w - r_dim.width) * 0.5, by + 185.0, 13.0, TEXT_MUTED);
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
        "Flappy Jetpack Ultra"
    }
}
