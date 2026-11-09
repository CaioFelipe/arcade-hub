use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

// =========================================================================
// 1. ENUMS, CONFIGURAÇÕES E ARENAS
// =========================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Difficulty {
    Novice,   // IA básica, desvia de colisões óbvias
    Tactical, // IA intermediária com flood-fill e busca de espaço
    Master,   // IA agressiva que tenta cercar o jogador e usa turbo
}

impl Difficulty {
    pub fn name(&self) -> &'static str {
        match self {
            Difficulty::Novice => "NOVATO",
            Difficulty::Tactical => "TÁTICO",
            Difficulty::Master => "MESTRE DA GRADE",
        }
    }

    pub fn base_speed(&self) -> u32 {
        match self {
            Difficulty::Novice => 4,
            Difficulty::Tactical => 3,
            Difficulty::Master => 2,
        }
    }

    pub fn score_multiplier(&self) -> f32 {
        match self {
            Difficulty::Novice => 1.0,
            Difficulty::Tactical => 1.5,
            Difficulty::Master => 2.5,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ArenaType {
    ClassicGrid, // Arena aberta
    Portals,     // Portais de teletransporte quântico
    CentralCore, // Pilares de energia centrais
    Chambers,    // Corredores táticos e câmaras
}

impl ArenaType {
    pub fn name(&self) -> &'static str {
        match self {
            ArenaType::ClassicGrid => "GRADE CLÁSSICA",
            ArenaType::Portals => "PORTAIS QUÂNTICOS",
            ArenaType::CentralCore => "NÚCLEO DE ENERGIA",
            ArenaType::Chambers => "CÂMARAS DE CHOQUE",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ItemKind {
    TurboRecharge, // Recarrega 50% da barra de turbo
    EmpBlast,      // Pulso EMP que desintegra 16 blocos do rastro adversário
    PhaseShield,   // Escudo que absorve 1 colisão de rastro
}

impl ItemKind {
    pub fn color(&self) -> Color {
        match self {
            ItemKind::TurboRecharge => YELLOW_ACCENT,
            ItemKind::EmpBlast => Color::new(0.95, 0.45, 0.15, 1.0),
            ItemKind::PhaseShield => ICE_BLUE,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ItemKind::TurboRecharge => "+TURBO!",
            ItemKind::EmpBlast => "+PULSO EMP!",
            ItemKind::PhaseShield => "+ESCUDO DE FASE!",
        }
    }
}

#[derive(Clone)]
struct ArenaItem {
    pos: (i32, i32),
    kind: ItemKind,
    alive: bool,
    float_timer: f32,
}

#[derive(Clone)]
struct Portal {
    entry: (i32, i32),
    exit: (i32, i32),
    color: Color,
}

struct TronParticle {
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
    ReadyToStart,
    RoundCountdown { timer: f32 },
    Playing,
    RoundOver { winner: u32, timer: f32 },
    MatchOver { champion: u32, timer: f32 },
}

// =========================================================================
// 2. ESTRUTURA DO CORREDOR (CYCLE RACER)
// =========================================================================

#[derive(Clone)]
struct CycleRacer {
    pos: (i32, i32),
    dir: (i32, i32),
    next_dir: (i32, i32),
    trail: Vec<(i32, i32)>,
    turbo_fuel: f32,
    is_boosting: bool,
    shield_charges: i32,
    emp_charges: i32,
    jump_cooldown: f32,
    is_jumping: bool,
    color: Color,
    trail_color: Color,
}

impl CycleRacer {
    fn new(start_x: i32, start_y: i32, dir_x: i32, dir_y: i32, color: Color, trail_col: Color) -> Self {
        Self {
            pos: (start_x, start_y),
            dir: (dir_x, dir_y),
            next_dir: (dir_x, dir_y),
            trail: vec![(start_x, start_y)],
            turbo_fuel: 100.0,
            is_boosting: false,
            shield_charges: 0,
            emp_charges: 0,
            jump_cooldown: 0.0,
            is_jumping: false,
            color,
            trail_color: trail_col,
        }
    }

    fn reset(&mut self, start_x: i32, start_y: i32, dir_x: i32, dir_y: i32) {
        self.pos = (start_x, start_y);
        self.dir = (dir_x, dir_y);
        self.next_dir = (dir_x, dir_y);
        self.trail.clear();
        self.trail.push((start_x, start_y));
        self.turbo_fuel = 100.0;
        self.is_boosting = false;
        self.shield_charges = 0;
        self.emp_charges = 0;
        self.jump_cooldown = 0.0;
        self.is_jumping = false;
    }
}

// =========================================================================
// 3. ESTRUTURA PRINCIPAL DO JOGO
// =========================================================================

pub struct TronGame {
    state: MatchState,
    difficulty: Difficulty,
    arena_type: ArenaType,

    // Corredores
    p1: CycleRacer,
    ai: CycleRacer,

    // Placar do Torneio (Melhor de 5 rounds)
    p1_rounds: u32,
    ai_rounds: u32,
    current_round: u32,

    // Elementos da Arena
    walls: Vec<(i32, i32)>,
    portals: Vec<Portal>,
    items: Vec<ArenaItem>,

    // Sistema e Física
    tick_timer_p1: u32,
    tick_timer_ai: u32,
    score: u32,
    game_over: bool,
    time: f32,
    screen_shake: f32,

    // Efeitos visuais
    particles: Vec<TronParticle>,
    float_texts: Vec<FloatText>,
}

impl TronGame {
    pub fn new() -> Self {
        let diff = Difficulty::Tactical;
        let mut game = Self {
            state: MatchState::ReadyToStart,
            difficulty: diff,
            arena_type: ArenaType::ClassicGrid,

            p1: CycleRacer::new(15, 30, 1, 0, TEXT_PRIMARY, ICE_BLUE),
            ai: CycleRacer::new(65, 30, -1, 0, YELLOW_ACCENT, CORAL),

            p1_rounds: 0,
            ai_rounds: 0,
            current_round: 1,

            walls: Vec::new(),
            portals: Vec::new(),
            items: Vec::new(),

            tick_timer_p1: 0,
            tick_timer_ai: 0,
            score: 0,
            game_over: false,
            time: 0.0,
            screen_shake: 0.0,

            particles: Vec::new(),
            float_texts: Vec::new(),
        };

        game.load_arena(ArenaType::ClassicGrid);
        game.spawn_arena_item();
        game
    }

    fn set_difficulty(&mut self, diff: Difficulty) {
        self.difficulty = diff;
        self.float_texts.push(FloatText {
            x: 400.0,
            y: 470.0,
            text: format!("DIFICULDADE: {}", diff.name()),
            color: YELLOW_ACCENT,
            alpha: 1.0,
        });
    }

    fn load_arena(&mut self, arena: ArenaType) {
        self.arena_type = arena;
        self.walls.clear();
        self.portals.clear();
        self.items.clear();

        match arena {
            ArenaType::ClassicGrid => {
                // Arena aberta clássica com alta fluidez
            }
            ArenaType::Portals => {
                // Par de portais quânticos norte-sul e leste-oeste
                self.portals.push(Portal {
                    entry: (40, 6),
                    exit: (40, 53),
                    color: ICE_BLUE,
                });
                self.portals.push(Portal {
                    entry: (8, 30),
                    exit: (71, 30),
                    color: Color::new(0.95, 0.45, 0.15, 1.0),
                });
            }
            ArenaType::CentralCore => {
                // Pilares quádruplos de contenção no centro
                for (px, py) in [(28, 20), (51, 20), (28, 39), (51, 39)] {
                    for dx in 0..3 {
                        for dy in 0..3 {
                            self.walls.push((px + dx, py + dy));
                        }
                    }
                }
            }
            ArenaType::Chambers => {
                // Corredores táticos e blocos de contenção
                for x in 22..35 {
                    self.walls.push((x, 15));
                    self.walls.push((x, 44));
                }
                for x in 45..58 {
                    self.walls.push((x, 15));
                    self.walls.push((x, 44));
                }
                for y in 22..38 {
                    self.walls.push((40, y));
                }
                self.portals.push(Portal {
                    entry: (5, 5),
                    exit: (74, 54),
                    color: EMERALD,
                });
            }
        }
    }

    fn spawn_arena_item(&mut self) {
        for _ in 0..100 {
            let x = rand::gen_range(5, 75);
            let y = rand::gen_range(5, 55);
            if !self.walls.contains(&(x, y)) && !self.p1.trail.contains(&(x, y)) && !self.ai.trail.contains(&(x, y)) {
                let kind = match rand::gen_range(0, 3) {
                    0 => ItemKind::TurboRecharge,
                    1 => ItemKind::EmpBlast,
                    _ => ItemKind::PhaseShield,
                };
                self.items.push(ArenaItem {
                    pos: (x, y),
                    kind,
                    alive: true,
                    float_timer: rand::gen_range(0.0, 6.28),
                });
                break;
            }
        }
    }

    fn start_new_round(&mut self) {
        self.p1.reset(15, 30, 1, 0);
        self.ai.reset(65, 30, -1, 0);
        self.state = MatchState::RoundCountdown { timer: 2.0 };

        // Altera a arena em cada rodada para manter a variedade tática
        let next_arena = match self.current_round % 4 {
            1 => ArenaType::ClassicGrid,
            2 => ArenaType::CentralCore,
            3 => ArenaType::Portals,
            _ => ArenaType::Chambers,
        };
        self.load_arena(next_arena);
        self.spawn_arena_item();
    }

    fn trigger_emp_p1(&mut self) {
        if self.p1.emp_charges > 0 {
            self.p1.emp_charges -= 1;
            self.screen_shake = 5.0;

            // Desintegra 18 blocos do rastro da IA
            let remove_count = 18.min(self.ai.trail.len().saturating_sub(2));
            for _ in 0..remove_count {
                if let Some(pos) = self.ai.trail.pop() {
                    let px = (pos.0 as f32) * 10.0 + 5.0;
                    let py = (pos.1 as f32) * 10.0 + 5.0;
                    self.particles.push(TronParticle {
                        x: px,
                        y: py,
                        vx: rand::gen_range(-2.0, 2.0),
                        vy: rand::gen_range(-2.0, 2.0),
                        color: CORAL,
                        life: 0.45,
                        max_life: 0.45,
                        size: 3.0,
                    });
                }
            }

            self.float_texts.push(FloatText {
                x: 400.0,
                y: 260.0,
                text: "PULSO EMP DISPARADO!".to_string(),
                color: Color::new(0.95, 0.45, 0.15, 1.0),
                alpha: 1.2,
            });
        }
    }

    fn trigger_jump_p1(&mut self) {
        if self.p1.jump_cooldown <= 0.0 {
            self.p1.jump_cooldown = 6.0;
            self.p1.is_jumping = true;
            self.float_texts.push(FloatText {
                x: (self.p1.pos.0 as f32) * 10.0,
                y: (self.p1.pos.1 as f32) * 10.0 - 15.0,
                text: "SALTO QUÂNTICO!".to_string(),
                color: ICE_BLUE,
                alpha: 1.0,
            });
        }
    }

    // =========================================================================
    // 4. INTELIGÊNCIA ARTIFICIAL TÁTICA DA MOTO ADVERSÁRIA
    // =========================================================================

    fn compute_ai_direction(&mut self) {
        let (ax, ay) = self.ai.pos;
        let mut dirs = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

        // Não pode inverter 180 graus na mesma linha
        dirs.retain(|d| *d != (-self.ai.dir.0, -self.ai.dir.1));

        let current_dir = self.ai.dir;

        // Função de avaliação de segurança para um passo
        let is_safe = |x: i32, y: i32, walls: &[(i32, i32)], p1_tr: &[(i32, i32)], ai_tr: &[(i32, i32)]| -> bool {
            if x <= 0 || x >= 79 || y <= 0 || y >= 59 {
                return false;
            }
            if walls.contains(&(x, y)) || p1_tr.contains(&(x, y)) || ai_tr.contains(&(x, y)) {
                return false;
            }
            true
        };

        match self.difficulty {
            Difficulty::Novice => {
                // IA básica: mantém rumo até perigo iminente
                let next_cur = (ax + current_dir.0, ay + current_dir.1);
                if !is_safe(next_cur.0, next_cur.1, &self.walls, &self.p1.trail, &self.ai.trail) {
                    for d in dirs {
                        let nx = ax + d.0;
                        let ny = ay + d.1;
                        if is_safe(nx, ny, &self.walls, &self.p1.trail, &self.ai.trail) {
                            self.ai.next_dir = d;
                            break;
                        }
                    }
                }
            }
            Difficulty::Tactical | Difficulty::Master => {
                // IA tática: avalia o espaço livre (flood-fill simplificado em profundidade de 6 passos)
                let mut best_score = -9999;
                let mut best_dir = current_dir;

                for d in dirs {
                    let nx = ax + d.0;
                    let ny = ay + d.1;
                    if !is_safe(nx, ny, &self.walls, &self.p1.trail, &self.ai.trail) {
                        continue;
                    }

                    // Calcula profundidade de passos livres à frente
                    let mut score = 0;
                    for step in 1..=8 {
                        let sx = ax + d.0 * step;
                        let sy = ay + d.1 * step;
                        if is_safe(sx, sy, &self.walls, &self.p1.trail, &self.ai.trail) {
                            score += 10;
                        } else {
                            break;
                        }
                    }

                    // Se for na mesma direção atual, adiciona preferência de inércia para evitar zigue-zague
                    if d == current_dir {
                        score += 5;
                    }

                    // No modo Master, avalia distância do jogador para tentar cercá-lo
                    if self.difficulty == Difficulty::Master {
                        let dist_to_p1 = ((nx - self.p1.pos.0).abs() + (ny - self.p1.pos.1).abs()) as i32;
                        if dist_to_p1 < 18 {
                            score += 8;
                        }
                    }

                    if score > best_score {
                        best_score = score;
                        best_dir = d;
                    }
                }

                self.ai.next_dir = best_dir;

                // Uso de Turbo pela IA em retas longas
                if best_score >= 60 && self.ai.turbo_fuel > 40.0 && rand::gen_range(0, 10) == 0 {
                    self.ai.is_boosting = true;
                } else {
                    self.ai.is_boosting = false;
                }
            }
        }
    }
}

// =========================================================================
// 5. IMPLEMENTAÇÃO DO TRAIT ARCADETRAIT
// =========================================================================

impl ArcadeGame for TronGame {
    fn update(&mut self) {
        let dt = get_frame_time();
        self.time += dt;

        // Amortecimento do tremor de tela
        if self.screen_shake > 0.0 {
            self.screen_shake = (self.screen_shake - dt * 16.0).max(0.0);
        }

        // Atualiza cooldown de salto e recarga passiva de turbo
        if self.p1.jump_cooldown > 0.0 {
            self.p1.jump_cooldown -= dt;
        }
        if !self.p1.is_boosting && self.p1.turbo_fuel < 100.0 {
            self.p1.turbo_fuel = (self.p1.turbo_fuel + dt * 12.0).min(100.0);
        }
        if !self.ai.is_boosting && self.ai.turbo_fuel < 100.0 {
            self.ai.turbo_fuel = (self.ai.turbo_fuel + dt * 12.0).min(100.0);
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
        // TRATAMENTO DOS ESTADOS DO TORNEIO
        // =====================================================================
        match &mut self.state {
            MatchState::ReadyToStart => {
                // Seletor de Dificuldade
                if is_key_pressed(KeyCode::Key1) {
                    self.set_difficulty(Difficulty::Novice);
                } else if is_key_pressed(KeyCode::Key2) {
                    self.set_difficulty(Difficulty::Tactical);
                } else if is_key_pressed(KeyCode::Key3) {
                    self.set_difficulty(Difficulty::Master);
                }

                if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
                    self.start_new_round();
                }
                return;
            }
            MatchState::RoundCountdown { timer } => {
                *timer -= dt;
                if *timer <= 0.0 {
                    self.state = MatchState::Playing;
                }
                return;
            }
            MatchState::RoundOver { winner: _, timer } => {
                *timer += dt;
                if *timer >= 2.5 || is_key_pressed(KeyCode::Space) {
                    if self.p1_rounds >= 3 || self.ai_rounds >= 3 {
                        let champ = if self.p1_rounds >= 3 { 1 } else { 2 };
                        self.state = MatchState::MatchOver { champion: champ, timer: 0.0 };
                        if champ == 1 {
                            self.score += 500;
                        }
                    } else {
                        self.current_round += 1;
                        self.start_new_round();
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
        // CONTROLES DO JOGADOR 1
        // =====================================================================
        if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W)) && self.p1.dir != (0, 1) {
            self.p1.next_dir = (0, -1);
        } else if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S)) && self.p1.dir != (0, -1) {
            self.p1.next_dir = (0, 1);
        } else if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)) && self.p1.dir != (1, 0) {
            self.p1.next_dir = (-1, 0);
        } else if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D)) && self.p1.dir != (-1, 0) {
            self.p1.next_dir = (1, 0);
        }

        // Ativação do Turbo Boost do Jogador 1 (Espaço ou Shift)
        self.p1.is_boosting = (is_key_down(KeyCode::Space) || is_key_down(KeyCode::LeftShift)) && self.p1.turbo_fuel > 5.0;
        if self.p1.is_boosting {
            self.p1.turbo_fuel = (self.p1.turbo_fuel - dt * 45.0).max(0.0);
        }

        // Disparo de EMP do Jogador 1 (Tecla E)
        if is_key_pressed(KeyCode::E) {
            self.trigger_emp_p1();
        }

        // Salto Quântico do Jogador 1 (Tecla J)
        if is_key_pressed(KeyCode::J) {
            self.trigger_jump_p1();
        }

        // =====================================================================
        // ATUALIZAÇÃO DA IA
        // =====================================================================
        self.compute_ai_direction();

        // =====================================================================
        // TICK DE FÍSICA E MOVIMENTO (Com suporte a Turbo Boost diferenciado)
        // =====================================================================
        let base_interval = self.difficulty.base_speed();
        let p1_interval = if self.p1.is_boosting { 1.max(base_interval / 2) } else { base_interval };
        let ai_interval = if self.ai.is_boosting { 1.max(base_interval / 2) } else { base_interval };

        self.tick_timer_p1 += 1;
        self.tick_timer_ai += 1;

        let mut p1_moved = false;
        let mut ai_moved = false;

        let mut next_p1 = self.p1.pos;
        let mut next_ai = self.ai.pos;

        if self.tick_timer_p1 >= p1_interval {
            self.tick_timer_p1 = 0;
            self.p1.dir = self.p1.next_dir;

            let step = if self.p1.is_jumping {
                self.p1.is_jumping = false;
                3
            } else {
                1
            };

            next_p1 = (self.p1.pos.0 + self.p1.dir.0 * step, self.p1.pos.1 + self.p1.dir.1 * step);

            // Verificação de Portais para o P1
            for portal in &self.portals {
                if next_p1 == portal.entry {
                    next_p1 = (portal.exit.0 + self.p1.dir.0, portal.exit.1 + self.p1.dir.1);
                    self.screen_shake = 3.5;
                    break;
                } else if next_p1 == portal.exit {
                    next_p1 = (portal.entry.0 + self.p1.dir.0, portal.entry.1 + self.p1.dir.1);
                    self.screen_shake = 3.5;
                    break;
                }
            }
            p1_moved = true;
        }

        if self.tick_timer_ai >= ai_interval {
            self.tick_timer_ai = 0;
            self.ai.dir = self.ai.next_dir;
            next_ai = (self.ai.pos.0 + self.ai.dir.0, self.ai.pos.1 + self.ai.dir.1);

            // Verificação de Portais para a IA
            for portal in &self.portals {
                if next_ai == portal.entry {
                    next_ai = (portal.exit.0 + self.ai.dir.0, portal.exit.1 + self.ai.dir.1);
                    break;
                } else if next_ai == portal.exit {
                    next_ai = (portal.entry.0 + self.ai.dir.0, portal.entry.1 + self.ai.dir.1);
                    break;
                }
            }
            ai_moved = true;
        }

        // =====================================================================
        // RESOLUÇÃO DE COLISÕES
        // =====================================================================
        let mut p1_crashed = false;
        let mut ai_crashed = false;

        if p1_moved {
            let hit_wall = next_p1.0 <= 0 || next_p1.0 >= 80 || next_p1.1 <= 0 || next_p1.1 >= 60 || self.walls.contains(&next_p1);
            let hit_trail = self.p1.trail.contains(&next_p1) || self.ai.trail.contains(&next_p1);

            if hit_wall || hit_trail {
                if self.p1.shield_charges > 0 {
                    self.p1.shield_charges -= 1;
                    self.screen_shake = 4.0;
                    self.float_texts.push(FloatText {
                        x: (next_p1.0 as f32) * 10.0,
                        y: (next_p1.1 as f32) * 10.0 - 15.0,
                        text: "ESCUDO ABSORVEU!".to_string(),
                        color: ICE_BLUE,
                        alpha: 1.0,
                    });
                } else {
                    p1_crashed = true;
                }
            } else {
                self.p1.pos = next_p1;
                self.p1.trail.push(next_p1);
                self.score += (1.0 * self.difficulty.score_multiplier()) as u32;

                // Emite partículas de rastro brilhante
                if self.p1.is_boosting {
                    let px = (next_p1.0 as f32) * 10.0 + 5.0;
                    let py = (next_p1.1 as f32) * 10.0 + 5.0;
                    self.particles.push(TronParticle {
                        x: px,
                        y: py,
                        vx: rand::gen_range(-1.5, 1.5),
                        vy: rand::gen_range(-1.5, 1.5),
                        color: TEXT_PRIMARY,
                        life: 0.35,
                        max_life: 0.35,
                        size: 3.0,
                    });
                }
            }
        }

        if ai_moved {
            let hit_wall = next_ai.0 <= 0 || next_ai.0 >= 80 || next_ai.1 <= 0 || next_ai.1 >= 60 || self.walls.contains(&next_ai);
            let hit_trail = self.p1.trail.contains(&next_ai) || self.ai.trail.contains(&next_ai);

            if hit_wall || hit_trail {
                ai_crashed = true;
            } else {
                self.ai.pos = next_ai;
                self.ai.trail.push(next_ai);
            }
        }

        // Colisão direta frontal mútua
        if p1_moved && ai_moved && next_p1 == next_ai {
            p1_crashed = true;
            ai_crashed = true;
        }

        // =====================================================================
        // COLETA DE ITENS DE ARENA
        // =====================================================================
        for item in &mut self.items {
            if !item.alive {
                continue;
            }
            if self.p1.pos == item.pos {
                item.alive = false;
                match item.kind {
                    ItemKind::TurboRecharge => {
                        self.p1.turbo_fuel = (self.p1.turbo_fuel + 50.0).min(100.0);
                    }
                    ItemKind::EmpBlast => {
                        self.p1.emp_charges += 1;
                    }
                    ItemKind::PhaseShield => {
                        self.p1.shield_charges += 1;
                    }
                }
                self.float_texts.push(FloatText {
                    x: (item.pos.0 as f32) * 10.0,
                    y: (item.pos.1 as f32) * 10.0 - 15.0,
                    text: item.kind.label().to_string(),
                    color: item.kind.color(),
                    alpha: 1.0,
                });
            } else if self.ai.pos == item.pos {
                item.alive = false;
                if item.kind == ItemKind::TurboRecharge {
                    self.ai.turbo_fuel = 100.0;
                }
            }
        }

        self.items.retain(|i| i.alive);

        // =====================================================================
        // TRATAMENTO DO FINAL DA RODADA (EXPLOSÃO E PONTUAÇÃO)
        // =====================================================================
        if p1_crashed || ai_crashed {
            self.screen_shake = 8.0;

            if p1_crashed && ai_crashed {
                // Empate na rodada
                self.state = MatchState::RoundOver { winner: 0, timer: 0.0 };
            } else if p1_crashed {
                self.ai_rounds += 1;
                self.state = MatchState::RoundOver { winner: 2, timer: 0.0 };
            } else {
                self.p1_rounds += 1;
                let bonus = (150.0 * self.difficulty.score_multiplier()) as u32;
                self.score += bonus;
                self.state = MatchState::RoundOver { winner: 1, timer: 0.0 };
            }

            // Desintegração em partículas neon
            let crash_pos = if p1_crashed { self.p1.pos } else { self.ai.pos };
            let crash_color = if p1_crashed { ICE_BLUE } else { CORAL };

            for _ in 0..40 {
                let spd = rand::gen_range(2.0, 7.5);
                let ang: f32 = rand::gen_range(0.0, 6.28);
                self.particles.push(TronParticle {
                    x: (crash_pos.0 as f32) * 10.0 + 5.0,
                    y: (crash_pos.1 as f32) * 10.0 + 5.0,
                    vx: ang.cos() * spd,
                    vy: ang.sin() * spd,
                    color: crash_color,
                    life: 0.75,
                    max_life: 0.75,
                    size: rand::gen_range(3.0, 5.5),
                });
            }
        }
    }

    // =========================================================================
    // 6. RENDERIZAÇÃO GRÁFICA NEON & EFEITOS
    // =========================================================================

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let shake_x = if self.screen_shake > 0.0 { rand::gen_range(-self.screen_shake, self.screen_shake) } else { 0.0 };
        let shake_y = if self.screen_shake > 0.0 { rand::gen_range(-self.screen_shake, self.screen_shake) } else { 0.0 };

        let ox = (screen_w - 800.0) * 0.5 + shake_x;
        let oy = (screen_h - 600.0) * 0.5 + 24.0 + shake_y;
        let cell = 10.0;

        // Fundo com bordas de contenção eletromagnética
        draw_rectangle(ox, oy, 800.0, 600.0, BG_COLOR);
        draw_rectangle_lines(ox, oy, 800.0, 600.0, 2.0, BORDER_ACTIVE);

        // Grade Neon Retro com nós de energia luminosa
        for i in (0..80).step_by(4) {
            for j in (0..60).step_by(4) {
                draw_circle(ox + (i as f32) * cell, oy + (j as f32) * cell, 1.2, Color::new(0.3, 0.5, 0.8, 0.12));
            }
        }

        // =====================================================================
        // PAREDES E OBSTÁCULOS DA ARENA
        // =====================================================================
        for &(wx, wy) in &self.walls {
            let cx = ox + (wx as f32) * cell;
            let cy = oy + (wy as f32) * cell;
            draw_rectangle(cx + 1.0, cy + 1.0, cell - 2.0, cell - 2.0, SURFACE_COLOR);
            draw_rectangle_lines(cx + 1.0, cy + 1.0, cell - 2.0, cell - 2.0, 1.5, CORAL);
            draw_circle(cx + cell * 0.5, cy + cell * 0.5, 1.8, CORAL);
        }

        // =====================================================================
        // PORTAIS QUÂNTICOS DE TELETRANSPORTE
        // =====================================================================
        for portal in &self.portals {
            let p_pulse = (self.time * 6.0).sin() * 2.0;
            for &(px, py) in &[portal.entry, portal.exit] {
                let cx = ox + (px as f32) * cell + cell * 0.5;
                let cy = oy + (py as f32) * cell + cell * 0.5;
                draw_circle_lines(cx, cy, 8.0 + p_pulse, 1.5, portal.color);
                draw_circle(cx, cy, 5.0, portal.color);
                draw_circle(cx, cy, 2.0, TEXT_PRIMARY);
            }
        }

        // =====================================================================
        // ITENS DE ARENA (ORBES DE DADOS)
        // =====================================================================
        for item in &self.items {
            if !item.alive {
                continue;
            }
            let ix = ox + (item.pos.0 as f32) * cell + cell * 0.5;
            let iy = oy + (item.pos.1 as f32) * cell + cell * 0.5;
            let col = item.kind.color();
            let pulse = (self.time * 7.0 + item.float_timer).sin() * 2.5;

            draw_circle(ix, iy, 7.0 + pulse * 0.5, col);
            draw_circle_lines(ix, iy, 9.0 + pulse, 1.5, TEXT_PRIMARY);
            draw_circle(ix, iy, 2.5, TEXT_PRIMARY);
        }

        // Partículas
        for p in &self.particles {
            let a = (p.life / p.max_life).clamp(0.0, 1.0);
            let mut c = p.color;
            c.a = a;
            draw_circle(ox + p.x, oy + p.y, p.size * a, c);
        }

        // =====================================================================
        // RASTRO E MOTO DO JOGADOR 1 (AZUL SAFIRA / COBALTO)
        // =====================================================================
        for &(tx, ty) in &self.p1.trail {
            let rx = ox + (tx as f32) * cell;
            let ry = oy + (ty as f32) * cell;
            draw_rectangle(rx + 0.5, ry + 0.5, cell - 1.0, cell - 1.0, self.p1.trail_color);
            draw_rectangle(rx + 2.0, ry + 2.0, cell - 4.0, cell - 4.0, Color::new(1.0, 1.0, 1.0, 0.35));
        }

        // Moto P1
        let p1x = ox + (self.p1.pos.0 as f32) * cell;
        let p1y = oy + (self.p1.pos.1 as f32) * cell;
        draw_rectangle(p1x, p1y, cell, cell, TEXT_PRIMARY);
        draw_rectangle_lines(p1x, p1y, cell, cell, 1.5, ICE_BLUE);

        // Indicador de Escudo P1
        if self.p1.shield_charges > 0 {
            draw_circle_lines(p1x + cell * 0.5, p1y + cell * 0.5, 12.0, 1.5, ICE_BLUE);
        }

        // =====================================================================
        // RASTRO E MOTO DA IA (CORAL / OURO)
        // =====================================================================
        for &(tx, ty) in &self.ai.trail {
            let rx = ox + (tx as f32) * cell;
            let ry = oy + (ty as f32) * cell;
            draw_rectangle(rx + 0.5, ry + 0.5, cell - 1.0, cell - 1.0, self.ai.trail_color);
            draw_rectangle(rx + 2.0, ry + 2.0, cell - 4.0, cell - 4.0, Color::new(1.0, 1.0, 1.0, 0.35));
        }

        // Moto IA
        let aix = ox + (self.ai.pos.0 as f32) * cell;
        let aiy = oy + (self.ai.pos.1 as f32) * cell;
        draw_rectangle(aix, aiy, cell, cell, YELLOW_ACCENT);
        draw_rectangle_lines(aix, aiy, cell, cell, 1.5, CORAL);

        // Textos flutuantes (+TURBO, EMP, etc.)
        for ft in &self.float_texts {
            let mut col = ft.color;
            col.a = ft.alpha;
            draw_text(&ft.text, ox + ft.x - 16.0, oy + ft.y, 16.0, col);
        }

        // =====================================================================
        // HUD SUPERIOR DO TORNEIO (PLACAR MELHOR DE 5)
        // =====================================================================
        // Placar P1 vs IA
        let score_txt = format!("P1 [ {} ] - [ {} ] CPU", self.p1_rounds, self.ai_rounds);
        let s_dim = measure_text(&score_txt, None, 20, 1.0);
        draw_text(&score_txt, ox + (800.0 - s_dim.width) * 0.5, oy + 22.0, 20.0, YELLOW_ACCENT);

        // Arena e Dificuldade
        let a_str = format!("ARENA: {}", self.arena_type.name());
        draw_text(&a_str, ox + 35.0, oy + 22.0, 13.0, ICE_BLUE);

        let d_str = format!("DIFICULDADE: {}", self.difficulty.name());
        draw_text(&d_str, ox + 600.0, oy + 22.0, 13.0, TEXT_MUTED);

        // Barra de Turbo do Jogador 1 no canto inferior esquerdo
        let t_bar_w = 140.0;
        let t_bar_h = 14.0;
        let tx = ox + 40.0;
        let ty = oy + 560.0;
        draw_card_rect(tx, ty, t_bar_w, t_bar_h, SURFACE_COLOR, BORDER_ACTIVE);
        let fill_w = (self.p1.turbo_fuel / 100.0) * (t_bar_w - 2.0);
        draw_rectangle(tx + 1.0, ty + 1.0, fill_w, t_bar_h - 2.0, YELLOW_ACCENT);
        draw_text("TURBO (ESPAÇO)", tx, ty - 5.0, 12.0, YELLOW_ACCENT);

        // Cargas de EMP e Salto
        if self.p1.emp_charges > 0 {
            draw_card_rect(tx + 160.0, ty - 6.0, 95.0, 22.0, SURFACE_COLOR, Color::new(0.95, 0.45, 0.15, 1.0));
            draw_text("EMP [E] ATIVO", tx + 168.0, ty + 10.0, 11.0, Color::new(0.95, 0.45, 0.15, 1.0));
        }

        if self.p1.jump_cooldown <= 0.0 {
            draw_card_rect(tx + 265.0, ty - 6.0, 95.0, 22.0, SURFACE_COLOR, ICE_BLUE);
            draw_text("SALTO [J] PRONTO", tx + 271.0, ty + 10.0, 11.0, ICE_BLUE);
        }

        // =====================================================================
        // OVERLAYS DE ESTADOS
        // =====================================================================
        if self.state == MatchState::ReadyToStart {
            let title = "TRON CYBER CYCLES NEO";
            let t_dim = measure_text(title, None, 28, 1.0);
            draw_text(title, ox + (800.0 - t_dim.width) * 0.5, oy + 210.0, 28.0, YELLOW_ACCENT);

            let prompt = "Pressione [ESPAÇO] para Iniciar o Torneio";
            let p_dim = measure_text(prompt, None, 16, 1.0);
            draw_text(prompt, ox + (800.0 - p_dim.width) * 0.5, oy + 250.0, 16.0, TEXT_PRIMARY);

            let diff_prompt = "Dificuldade: [1] Novato  [2] Tático  [3] Mestre da Grade";
            let d_dim = measure_text(diff_prompt, None, 14, 1.0);
            draw_text(diff_prompt, ox + (800.0 - d_dim.width) * 0.5, oy + 380.0, 14.0, ICE_BLUE);
        }

        if let MatchState::RoundCountdown { timer } = &self.state {
            let count_txt = format!("ROUND {} EM {:.1}...", self.current_round, timer);
            let c_dim = measure_text(&count_txt, None, 30, 1.0);
            draw_text(&count_txt, ox + (800.0 - c_dim.width) * 0.5, oy + 300.0, 30.0, YELLOW_ACCENT);
        }

        if let MatchState::RoundOver { winner, timer: _ } = &self.state {
            let msg = match winner {
                1 => "RODADA VENCIDA PELO JOGADOR AZUL!",
                2 => "RODADA VENCIDA PELA IA!",
                _ => "EMPATE NA RODADA!",
            };
            let col = match winner {
                1 => EMERALD,
                2 => CORAL,
                _ => YELLOW_ACCENT,
            };
            let dim = measure_text(msg, None, 26, 1.0);
            draw_text(msg, ox + (800.0 - dim.width) * 0.5, oy + 290.0, 26.0, col);

            let prompt = "Pressione [ESPAÇO] para Próxima Rodada";
            let pr_dim = measure_text(prompt, None, 14, 1.0);
            draw_text(prompt, ox + (800.0 - pr_dim.width) * 0.5, oy + 330.0, 14.0, TEXT_PRIMARY);
        }

        if let MatchState::MatchOver { champion, timer: _ } = &self.state {
            draw_rectangle(ox, oy, 800.0, 600.0, Color::new(0.04, 0.05, 0.09, 0.88));

            let box_w = 520.0;
            let box_h = 230.0;
            let bx = ox + (800.0 - box_w) * 0.5;
            let by = oy + (600.0 - box_h) * 0.5;
            let champ_col = if *champion == 1 { EMERALD } else { CORAL };
            draw_card_rect(bx, by, box_w, box_h, SURFACE_COLOR, champ_col);

            let title = if *champion == 1 { "CAMPEÃO DA GRADE CYBER!" } else { "A IA CONQUISTOU A GRADE!" };
            let t_dim = measure_text(title, None, 28, 1.0);
            draw_text(title, bx + (box_w - t_dim.width) * 0.5, by + 50.0, 28.0, champ_col);

            let score_final = format!("Placar Final: P1 {} x {} CPU", self.p1_rounds, self.ai_rounds);
            let sf_dim = measure_text(&score_final, None, 18, 1.0);
            draw_text(&score_final, bx + (box_w - sf_dim.width) * 0.5, by + 95.0, 18.0, YELLOW_ACCENT);

            let pts_txt = format!("Pontuação Total: {} PONTOS", self.score);
            let pt_dim = measure_text(&pts_txt, None, 15, 1.0);
            draw_text(&pts_txt, bx + (box_w - pt_dim.width) * 0.5, by + 130.0, 15.0, TEXT_PRIMARY);

            let r_prompt = "Pressione [R] para Novo Torneio ou [ESC] para Voltar";
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
        "Tron Cyber Cycles Neo"
    }
}
