use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

// =========================================================================
// 1. ENUMS, ESTRUTURAS E CONFIGURAÇÕES
// =========================================================================

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Difficulty {
    Casual,     // Bordas infinitas (wrap-around), velocidade moderada
    Normal,     // Paredes sólidas nas bordas, velocidade média
    Cyberpunk,  // Paredes sólidas, velocidade alta, pontuação multiplicada
}

impl Difficulty {
    pub fn name(&self) -> &'static str {
        match self {
            Difficulty::Casual => "CASUAL (SEM PAREDES)",
            Difficulty::Normal => "NORMAL (CLÁSSICO)",
            Difficulty::Cyberpunk => "CYBERPUNK (HARDCORE)",
        }
    }

    pub fn base_interval(&self) -> u32 {
        match self {
            Difficulty::Casual => 7,
            Difficulty::Normal => 5,
            Difficulty::Cyberpunk => 3,
        }
    }

    pub fn wrap_borders(&self) -> bool {
        match self {
            Difficulty::Casual => true,
            Difficulty::Normal => false,
            Difficulty::Cyberpunk => false,
        }
    }

    pub fn score_multiplier(&self) -> f32 {
        match self {
            Difficulty::Casual => 1.0,
            Difficulty::Normal => 1.5,
            Difficulty::Cyberpunk => 2.5,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum FoodKind {
    StandardApple,   // Maçã vermelha padrão (+10)
    GoldenApple,     // Maçã dourada bônus com temporizador (+50, combo duplo)
    GhostBerry,      // Fruta fantasma: atravessa o próprio corpo por 8s
    FreezeBerry,     // Fruta congelante: câmera lenta por 8s
    MagnetBerry,     // Fruta ímã: atrai itens próximos por 8s
    NanitePill,      // Pílula nanotecnológica: reduz o tamanho da cauda em 3
}

impl FoodKind {
    pub fn color(&self) -> Color {
        match self {
            FoodKind::StandardApple => CORAL,
            FoodKind::GoldenApple => YELLOW_ACCENT,
            FoodKind::GhostBerry => ICE_BLUE,
            FoodKind::FreezeBerry => Color::new(0.65, 0.40, 0.95, 1.0),
            FoodKind::MagnetBerry => Color::new(0.95, 0.50, 0.15, 1.0),
            FoodKind::NanitePill => EMERALD,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            FoodKind::StandardApple => "+10",
            FoodKind::GoldenApple => "+50 OURO!",
            FoodKind::GhostBerry => "FANTASMA QUÂNTICO!",
            FoodKind::FreezeBerry => "CÂMERA LENTA!",
            FoodKind::MagnetBerry => "ÍMÃ ATIVADO!",
            FoodKind::NanitePill => "REDUÇÃO NANITE!",
        }
    }
}

#[derive(Clone)]
struct FoodItem {
    pos: (i32, i32),
    kind: FoodKind,
    life_timer: f32, // Para itens temporários
    max_life: f32,
}

#[derive(Clone)]
struct PortalPair {
    entry: (i32, i32),
    exit: (i32, i32),
    color: Color,
}

struct SnakeParticle {
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
enum PlayState {
    Ready,
    Playing,
    GameOver { timer: f32 },
}

// =========================================================================
// 2. ESTRUTURA PRINCIPAL DO SNAKE CYBER NEON
// =========================================================================

pub struct SnakeGame {
    state: PlayState,
    difficulty: Difficulty,
    level: u32,

    // Corpo e física da cobra
    body: Vec<(i32, i32)>,
    dir: (i32, i32),
    next_dir: (i32, i32),
    move_timer: u32,
    apples_eaten: u32,

    // Comidas e colecionáveis
    foods: Vec<FoodItem>,
    golden_spawn_timer: f32,

    // Labirinto, paredes e portais
    walls: Vec<(i32, i32)>,
    portals: Vec<PortalPair>,

    // Timers de power-ups
    ghost_timer: f32,
    slowmo_timer: f32,
    magnet_timer: f32,

    // Combos e pontuação
    score: u32,
    combo: u32,
    combo_timer: f32,
    game_over: bool,
    time: f32,
    screen_shake: f32,

    // Partículas e textos flutuantes
    particles: Vec<SnakeParticle>,
    float_texts: Vec<FloatText>,
}

impl SnakeGame {
    pub fn new() -> Self {
        let diff = Difficulty::Normal;
        let mut game = Self {
            state: PlayState::Ready,
            difficulty: diff,
            level: 1,

            body: vec![(20, 15), (19, 15), (18, 15)],
            dir: (1, 0),
            next_dir: (1, 0),
            move_timer: 0,
            apples_eaten: 0,

            foods: Vec::new(),
            golden_spawn_timer: 0.0,

            walls: Vec::new(),
            portals: Vec::new(),

            ghost_timer: 0.0,
            slowmo_timer: 0.0,
            magnet_timer: 0.0,

            score: 0,
            combo: 0,
            combo_timer: 0.0,
            game_over: false,
            time: 0.0,
            screen_shake: 0.0,

            particles: Vec::new(),
            float_texts: Vec::new(),
        };

        game.load_level(1);
        game.spawn_standard_food();
        game
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

    // =========================================================================
    // 3. MAPAS E LABIRINTOS POR FASE
    // =========================================================================

    fn load_level(&mut self, lvl: u32) {
        self.level = lvl;
        self.walls.clear();
        self.portals.clear();

        match lvl {
            1 => {
                // Nível 1: Arena Aberta - ideal para aquecimento e velocidade
            }
            2 => {
                // Nível 2: 4 Pilares de Alta Densidade
                let pillars = [(10, 8), (29, 8), (10, 21), (29, 21)];
                for (px, py) in pillars {
                    for dx in 0..2 {
                        for dy in 0..2 {
                            self.walls.push((px + dx, py + dy));
                        }
                    }
                }
            }
            3 => {
                // Nível 3: Portais de Teletransporte Quânticos (Wormholes)
                self.portals.push(PortalPair {
                    entry: (5, 5),
                    exit: (34, 24),
                    color: ICE_BLUE,
                });
                self.portals.push(PortalPair {
                    entry: (5, 24),
                    exit: (34, 5),
                    color: Color::new(0.95, 0.50, 0.15, 1.0),
                });
                // Barreiras centrais
                for y in 11..19 {
                    self.walls.push((20, y));
                }
            }
            4 => {
                // Nível 4: Circuito Integrado com Corredores
                for x in 8..16 {
                    self.walls.push((x, 7));
                    self.walls.push((39 - x, 7));
                    self.walls.push((x, 22));
                    self.walls.push((39 - x, 22));
                }
                for y in 10..20 {
                    self.walls.push((12, y));
                    self.walls.push((27, y));
                }
                self.portals.push(PortalPair {
                    entry: (3, 15),
                    exit: (36, 15),
                    color: EMERALD,
                });
            }
            _ => {
                // Nível 5: O Núcleo Cibernético Supremo
                // Pilares nas quinas
                for (px, py) in [(6, 5), (33, 5), (6, 24), (33, 24)] {
                    for dx in 0..2 {
                        for dy in 0..2 {
                            self.walls.push((px + dx, py + dy));
                        }
                    }
                }
                // Cruz no centro com abertura
                for x in 16..24 {
                    if x != 19 && x != 20 {
                        self.walls.push((x, 15));
                    }
                }
                for y in 11..19 {
                    if y != 14 && y != 15 {
                        self.walls.push((20, y));
                    }
                }
                // Par de portais diagonais
                self.portals.push(PortalPair {
                    entry: (2, 2),
                    exit: (37, 27),
                    color: ICE_BLUE,
                });
                self.portals.push(PortalPair {
                    entry: (2, 27),
                    exit: (37, 2),
                    color: Color::new(0.85, 0.40, 0.98, 1.0),
                });
            }
        }
    }

    fn is_occupied(&self, pos: (i32, i32)) -> bool {
        if self.body.contains(&pos) || self.walls.contains(&pos) {
            return true;
        }
        for f in &self.foods {
            if f.pos == pos {
                return true;
            }
        }
        for p in &self.portals {
            if p.entry == pos || p.exit == pos {
                return true;
            }
        }
        false
    }

    fn spawn_standard_food(&mut self) {
        for _ in 0..200 {
            let x = rand::gen_range(1, 39);
            let y = rand::gen_range(1, 29);
            if !self.is_occupied((x, y)) {
                self.foods.push(FoodItem {
                    pos: (x, y),
                    kind: FoodKind::StandardApple,
                    life_timer: 9999.0,
                    max_life: 9999.0,
                });
                break;
            }
        }
    }

    fn spawn_special_food(&mut self, kind: FoodKind) {
        for _ in 0..200 {
            let x = rand::gen_range(2, 38);
            let y = rand::gen_range(2, 28);
            if !self.is_occupied((x, y)) {
                self.foods.push(FoodItem {
                    pos: (x, y),
                    kind,
                    life_timer: 12.0,
                    max_life: 12.0,
                });
                break;
            }
        }
    }

    fn apply_food_effect(&mut self, kind: FoodKind, pos: (i32, i32)) {
        let fpx = (pos.0 as f32) * 20.0 + 10.0;
        let fpy = (pos.1 as f32) * 20.0 + 10.0;
        let mult = self.difficulty.score_multiplier();

        match kind {
            FoodKind::StandardApple => {
                self.combo += 1;
                self.combo_timer = 4.5;
                let c_mult = 1.0 + (self.combo.saturating_sub(1) as f32) * 0.20;
                let pts = (10.0 * mult * c_mult) as u32;
                self.score += pts;

                let txt = if self.combo > 2 {
                    format!("+{} (x{})", pts, self.combo)
                } else {
                    format!("+{}", pts)
                };
                self.float_texts.push(FloatText {
                    x: fpx,
                    y: fpy - 10.0,
                    text: txt,
                    color: if self.combo > 2 { YELLOW_ACCENT } else { TEXT_PRIMARY },
                    alpha: 1.0,
                });

                self.apples_eaten += 1;
                if self.apples_eaten % 12 == 0 {
                    let next_lvl = (self.level % 5) + 1;
                    self.load_level(next_lvl);
                    self.float_texts.push(FloatText {
                        x: 400.0,
                        y: 180.0,
                        text: format!("FASE {} DESBLOQUEADA!", next_lvl),
                        color: YELLOW_ACCENT,
                        alpha: 1.5,
                    });
                }
            }
            FoodKind::GoldenApple => {
                self.combo += 2;
                self.combo_timer = 5.0;
                let pts = (50.0 * mult * 2.0) as u32;
                self.score += pts;
                self.screen_shake = 3.5;
                self.float_texts.push(FloatText {
                    x: fpx,
                    y: fpy - 10.0,
                    text: format!("+{} OURO SUPREMO!", pts),
                    color: YELLOW_ACCENT,
                    alpha: 1.2,
                });
            }
            FoodKind::GhostBerry => {
                self.ghost_timer = 8.5;
                self.float_texts.push(FloatText {
                    x: fpx,
                    y: fpy - 10.0,
                    text: "FANTASMA QUÂNTICO (Atravessa Corpo)!".to_string(),
                    color: ICE_BLUE,
                    alpha: 1.2,
                });
            }
            FoodKind::FreezeBerry => {
                self.slowmo_timer = 8.0;
                self.float_texts.push(FloatText {
                    x: fpx,
                    y: fpy - 10.0,
                    text: "CÂMERA LENTA!".to_string(),
                    color: Color::new(0.65, 0.40, 0.95, 1.0),
                    alpha: 1.2,
                });
            }
            FoodKind::MagnetBerry => {
                self.magnet_timer = 8.5;
                self.float_texts.push(FloatText {
                    x: fpx,
                    y: fpy - 10.0,
                    text: "ÍMÃ QUÂNTICO DE ALIMENTOS!".to_string(),
                    color: Color::new(0.95, 0.50, 0.15, 1.0),
                    alpha: 1.2,
                });
            }
            FoodKind::NanitePill => {
                let to_remove = 3.min(self.body.len().saturating_sub(3));
                for _ in 0..to_remove {
                    self.body.pop();
                }
                self.float_texts.push(FloatText {
                    x: fpx,
                    y: fpy - 10.0,
                    text: "NANITES: CORPO ENCURTADO!".to_string(),
                    color: EMERALD,
                    alpha: 1.2,
                });
            }
        }

        // Explosão de partículas vibrantes
        for _ in 0..18 {
            let spd = rand::gen_range(2.0, 5.0);
            let ang: f32 = rand::gen_range(0.0, 6.28);
            self.particles.push(SnakeParticle {
                x: fpx,
                y: fpy,
                vx: ang.cos() * spd,
                vy: ang.sin() * spd,
                color: kind.color(),
                life: 0.65,
                max_life: 0.65,
                size: rand::gen_range(2.5, 4.5),
            });
        }
    }
}

// =========================================================================
// 4. IMPLEMENTAÇÃO DA TRAIT ARCADETRAIT
// =========================================================================

impl ArcadeGame for SnakeGame {
    fn update(&mut self) {
        let dt = get_frame_time();
        self.time += dt;

        // Amortecimento do tremor de tela
        if self.screen_shake > 0.0 {
            self.screen_shake = (self.screen_shake - dt * 16.0).max(0.0);
        }

        // Atualiza combo timer
        if self.combo_timer > 0.0 {
            self.combo_timer -= dt;
            if self.combo_timer <= 0.0 {
                self.combo = 0;
            }
        }

        // Timers de power-ups
        if self.ghost_timer > 0.0 {
            self.ghost_timer -= dt;
        }
        if self.slowmo_timer > 0.0 {
            self.slowmo_timer -= dt;
        }
        if self.magnet_timer > 0.0 {
            self.magnet_timer -= dt;
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

        // Atualiza tempo de vida de comidas especiais
        self.foods.retain_mut(|f| {
            f.life_timer -= dt;
            f.life_timer > 0.0
        });

        // Spawn periódico de comidas especiais e maçãs douradas
        self.golden_spawn_timer += dt;
        if self.golden_spawn_timer >= 12.0 {
            self.golden_spawn_timer = 0.0;
            let roll = rand::gen_range(0, 5);
            let special_kind = match roll {
                0 => FoodKind::GoldenApple,
                1 => FoodKind::GhostBerry,
                2 => FoodKind::FreezeBerry,
                3 => FoodKind::MagnetBerry,
                _ => FoodKind::NanitePill,
            };
            self.spawn_special_food(special_kind);
        }

        // Garante que sempre existe pelo menos uma maçã padrão
        if !self.foods.iter().any(|f| f.kind == FoodKind::StandardApple) {
            self.spawn_standard_food();
        }

        // Estado inicial de preparação
        match &mut self.state {
            PlayState::Ready => {
                if is_key_pressed(KeyCode::Key1) {
                    self.set_difficulty(Difficulty::Casual);
                } else if is_key_pressed(KeyCode::Key2) {
                    self.set_difficulty(Difficulty::Normal);
                } else if is_key_pressed(KeyCode::Key3) {
                    self.set_difficulty(Difficulty::Cyberpunk);
                }

                if is_key_pressed(KeyCode::Space)
                    || is_key_pressed(KeyCode::Up)
                    || is_key_pressed(KeyCode::W)
                    || is_key_pressed(KeyCode::Down)
                    || is_key_pressed(KeyCode::S)
                    || is_key_pressed(KeyCode::Left)
                    || is_key_pressed(KeyCode::A)
                    || is_key_pressed(KeyCode::Right)
                    || is_key_pressed(KeyCode::D)
                    || is_mouse_button_pressed(MouseButton::Left)
                {
                    self.state = PlayState::Playing;
                }
                return;
            }
            PlayState::GameOver { timer } => {
                *timer += dt;
                if is_key_pressed(KeyCode::R) {
                    *self = Self::new();
                }
                return;
            }
            PlayState::Playing => {}
        }

        if self.game_over {
            return;
        }

        // =====================================================================
        // CONTROLES DE DIREÇÃO COM BUFFER
        // =====================================================================
        if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W)) && self.dir != (0, 1) {
            self.next_dir = (0, -1);
        } else if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S)) && self.dir != (0, -1) {
            self.next_dir = (0, 1);
        } else if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)) && self.dir != (1, 0) {
            self.next_dir = (-1, 0);
        } else if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D)) && self.dir != (-1, 0) {
            self.next_dir = (1, 0);
        }

        // =====================================================================
        // TICK DE MOVIMENTO DA COBRA
        // =====================================================================
        self.move_timer += 1;
        let mut base_spd = self.difficulty.base_interval();
        if self.slowmo_timer > 0.0 {
            base_spd = (base_spd * 2).max(6);
        }

        if self.move_timer >= base_spd {
            self.move_timer = 0;
            self.dir = self.next_dir;
            let (hx, hy) = self.body[0];
            let mut nx = hx + self.dir.0;
            let mut ny = hy + self.dir.1;

            // Tratamento de Paredes Exteriores vs Wrap-around
            if self.difficulty.wrap_borders() {
                nx = (nx + 40) % 40;
                ny = (ny + 30) % 30;
            } else {
                if nx < 0 || nx >= 40 || ny < 0 || ny >= 30 {
                    self.game_over = true;
                    self.screen_shake = 7.0;
                    self.state = PlayState::GameOver { timer: 0.0 };
                    return;
                }
            }

            // Colisão com Paredes Internas do Labirinto
            if self.walls.contains(&(nx, ny)) {
                self.game_over = true;
                self.screen_shake = 7.0;
                self.state = PlayState::GameOver { timer: 0.0 };
                return;
            }

            // Verificação de Portais de Teletransporte
            for portal in &self.portals {
                if (nx, ny) == portal.entry {
                    nx = portal.exit.0 + self.dir.0;
                    ny = portal.exit.1 + self.dir.1;
                    self.screen_shake = 4.0;
                    // Partículas de teletransporte
                    for _ in 0..15 {
                        self.particles.push(SnakeParticle {
                            x: (portal.exit.0 as f32) * 20.0 + 10.0,
                            y: (portal.exit.1 as f32) * 20.0 + 10.0,
                            vx: rand::gen_range(-3.5, 3.5),
                            vy: rand::gen_range(-3.5, 3.5),
                            color: portal.color,
                            life: 0.5,
                            max_life: 0.5,
                            size: 3.5,
                        });
                    }
                    break;
                } else if (nx, ny) == portal.exit {
                    nx = portal.entry.0 + self.dir.0;
                    ny = portal.entry.1 + self.dir.1;
                    self.screen_shake = 4.0;
                    for _ in 0..15 {
                        self.particles.push(SnakeParticle {
                            x: (portal.entry.0 as f32) * 20.0 + 10.0,
                            y: (portal.entry.1 as f32) * 20.0 + 10.0,
                            vx: rand::gen_range(-3.5, 3.5),
                            vy: rand::gen_range(-3.5, 3.5),
                            color: portal.color,
                            life: 0.5,
                            max_life: 0.5,
                            size: 3.5,
                        });
                    }
                    break;
                }
            }

            let new_head = (nx, ny);

            // Colisão com o Próprio Corpo (Ignorada se poder Fantasma estiver ativo)
            if self.body.contains(&new_head) && self.ghost_timer <= 0.0 {
                self.game_over = true;
                self.screen_shake = 8.0;
                self.state = PlayState::GameOver { timer: 0.0 };
                return;
            }

            // Insere a nova cabeça
            self.body.insert(0, new_head);

            // Verifica se comeu alguma fruta
            let mut eaten_idx = None;
            for (idx, f) in self.foods.iter().enumerate() {
                if f.pos == new_head {
                    eaten_idx = Some((idx, f.kind));
                    break;
                }
            }

            if let Some((idx, kind)) = eaten_idx {
                self.foods.remove(idx);
                self.apply_food_effect(kind, new_head);
                if kind == FoodKind::StandardApple {
                    self.spawn_standard_food();
                }
            } else {
                // Se não comeu, remove a cauda para manter o tamanho
                self.body.pop();
            }
        }
    }

    // =========================================================================
    // 5. RENDERIZAÇÃO GRÁFICA & ESTILIZAÇÃO CYBER NEON
    // =========================================================================

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let shake_x = if self.screen_shake > 0.0 { rand::gen_range(-self.screen_shake, self.screen_shake) } else { 0.0 };
        let shake_y = if self.screen_shake > 0.0 { rand::gen_range(-self.screen_shake, self.screen_shake) } else { 0.0 };

        let ox = (screen_w - 800.0) * 0.5 + shake_x;
        let oy = (screen_h - 600.0) * 0.5 + 24.0 + shake_y;
        let cell = 20.0;

        // Fundo do tabuleiro com bordas neon
        draw_rectangle(ox, oy, 800.0, 600.0, BG_COLOR);
        let border_col = if self.difficulty.wrap_borders() {
            Color::new(0.2, 0.35, 0.55, 0.5) // Borda suave que indica wrap-around
        } else {
            CORAL // Borda sólida letal
        };
        draw_rectangle_lines(ox, oy, 800.0, 600.0, 2.0, border_col);

        // Grade geométrica sutil com efeito xadrez retrô
        for i in 0..40 {
            for j in 0..30 {
                if (i + j) % 2 == 0 {
                    draw_rectangle(
                        ox + (i as f32) * cell,
                        oy + (j as f32) * cell,
                        cell,
                        cell,
                        Color::new(1.0, 1.0, 1.0, 0.015),
                    );
                }
            }
        }

        // =====================================================================
        // PORTAIS DE TELETRANSPORTE (WORMHOLES)
        // =====================================================================
        for portal in &self.portals {
            let p_pulse = (self.time * 6.0).sin() * 2.0;

            for &(px, py) in &[portal.entry, portal.exit] {
                let cx = ox + (px as f32) * cell + cell * 0.5;
                let cy = oy + (py as f32) * cell + cell * 0.5;

                // Anel externo pulsante
                draw_circle_lines(cx, cy, 10.0 + p_pulse, 2.0, portal.color);
                draw_circle(cx, cy, 7.0, portal.color);
                draw_circle(cx, cy, 3.5, TEXT_PRIMARY);
            }
        }

        // =====================================================================
        // PAREDES E OBSTÁCULOS DO LABIRINTO
        // =====================================================================
        for &(wx, wy) in &self.walls {
            let cx = ox + (wx as f32) * cell;
            let cy = oy + (wy as f32) * cell;

            draw_rectangle(cx + 1.0, cy + 1.0, cell - 2.0, cell - 2.0, SURFACE_COLOR);
            draw_rectangle_lines(cx + 1.0, cy + 1.0, cell - 2.0, cell - 2.0, 1.5, BORDER_ACTIVE);
            // Núcleo do pilar de energia
            draw_circle(cx + cell * 0.5, cy + cell * 0.5, 2.5, ICE_BLUE);
        }

        // Partículas
        for p in &self.particles {
            let a = (p.life / p.max_life).clamp(0.0, 1.0);
            let mut c = p.color;
            c.a = a;
            draw_circle(ox + p.x, oy + p.y, p.size * a, c);
        }

        // =====================================================================
        // ALIMENTOS E FRUTAS ESPECIAIS
        // =====================================================================
        for food in &self.foods {
            let ax = ox + (food.pos.0 as f32) * cell + cell * 0.5;
            let ay = oy + (food.pos.1 as f32) * cell + cell * 0.5;
            let pulse = (self.time * 7.0).sin() * 2.5;
            let col = food.kind.color();

            // Halo brilhante
            draw_circle(ax, ay, (cell * 0.45) + pulse + 3.0, Color::new(col.r, col.g, col.b, 0.22));
            // Corpo da fruta
            draw_circle(ax, ay, cell * 0.40 + pulse * 0.4, col);
            // Ponto de luz especular
            draw_circle(ax - 2.5, ay - 2.5, 2.2, TEXT_PRIMARY);

            // Indicador de folha ou coroa se for maçã
            if food.kind == FoodKind::StandardApple || food.kind == FoodKind::GoldenApple {
                draw_circle(ax + 2.0, ay - 6.5, 2.0, EMERALD);
            }

            // Arco regressivo de tempo para frutas temporárias
            if food.life_timer < 12.0 {
                let ratio = (food.life_timer / food.max_life).clamp(0.0, 1.0);
                draw_circle_lines(ax, ay, cell * 0.58, 1.5, Color::new(1.0, 1.0, 1.0, ratio));
            }
        }

        // =====================================================================
        // CORPO E CABEÇA DA COBRA
        // =====================================================================
        let is_ghost = self.ghost_timer > 0.0;
        let total = self.body.len().max(1) as f32;

        for (idx, &(bx, by)) in self.body.iter().enumerate().rev() {
            let cx = ox + (bx as f32) * cell + cell * 0.5;
            let cy = oy + (by as f32) * cell + cell * 0.5;
            let factor = 1.0 - (idx as f32 / total);

            if idx == 0 {
                // Cabeça da cobra
                let head_color = if is_ghost {
                    Color::new(0.4, 0.8, 1.0, 0.75)
                } else {
                    ICE_BLUE
                };
                draw_circle(cx, cy, cell * 0.48, head_color);
                draw_circle_lines(cx, cy, cell * 0.48, 1.5, TEXT_PRIMARY);

                // Olhos dinâmicos
                let eye_offset_x = (self.dir.0 as f32) * 3.5;
                let eye_offset_y = (self.dir.1 as f32) * 3.5;

                let (e1_ox, e1_oy, e2_ox, e2_oy) = if self.dir.0 != 0 {
                    (0.0, -4.0, 0.0, 4.0)
                } else {
                    (-4.0, 0.0, 4.0, 0.0)
                };

                draw_circle(cx + eye_offset_x + e1_ox, cy + eye_offset_y + e1_oy, 3.2, TEXT_PRIMARY);
                draw_circle(cx + eye_offset_x + e2_ox, cy + eye_offset_y + e2_oy, 3.2, TEXT_PRIMARY);

                draw_circle(cx + eye_offset_x * 1.3 + e1_ox, cy + eye_offset_y * 1.3 + e1_oy, 1.8, BG_COLOR);
                draw_circle(cx + eye_offset_x * 1.3 + e2_ox, cy + eye_offset_y * 1.3 + e2_oy, 1.8, BG_COLOR);
            } else {
                // Corpo com gradiente de cor e efeito de pulso
                let body_r = cell * (0.36 + factor * 0.08);
                let alpha = if is_ghost { 0.45 } else { 0.92 };

                let col = Color::new(
                    0.15 + factor * 0.22,
                    0.38 + factor * 0.26,
                    0.92,
                    alpha,
                );
                draw_circle(cx, cy, body_r, col);
                // Núcleo luminoso
                draw_circle(cx, cy, body_r * 0.45, Color::new(0.6, 0.8, 1.0, alpha * 0.4));
            }
        }

        // Textos flutuantes (+10, COMBO x3, etc.)
        for ft in &self.float_texts {
            let mut col = ft.color;
            col.a = ft.alpha;
            draw_text(&ft.text, ox + ft.x - 12.0, oy + ft.y, 16.0, col);
        }

        // =====================================================================
        // HUD SUPERIOR DA ARENA
        // =====================================================================
        let lvl_str = format!("FASE {}/5", self.level);
        draw_text(&lvl_str, ox + 35.0, oy + 20.0, 15.0, YELLOW_ACCENT);

        let diff_str = format!("[{}] x{:.1}", self.difficulty.name(), self.difficulty.score_multiplier());
        draw_text(&diff_str, ox + 140.0, oy + 20.0, 13.0, TEXT_MUTED);

        let len_str = format!("TAMANHO: {}", self.body.len());
        draw_text(&len_str, ox + 430.0, oy + 20.0, 14.0, EMERALD);

        if self.combo > 1 {
            let c_str = format!("COMBO x{}!", self.combo);
            draw_text(&c_str, ox + 560.0, oy + 20.0, 15.0, CORAL);
        }

        // Badges de power-up ativos no rodapé da tela
        let mut p_x = ox + 40.0;
        let p_y = oy + 540.0;
        if self.ghost_timer > 0.0 {
            let g_str = format!("FANTASMA {:.1}s", self.ghost_timer);
            draw_card_rect(p_x, p_y, 115.0, 22.0, SURFACE_COLOR, ICE_BLUE);
            draw_text(&g_str, p_x + 8.0, p_y + 16.0, 12.0, ICE_BLUE);
            p_x += 125.0;
        }
        if self.slowmo_timer > 0.0 {
            let s_str = format!("SLOW-MO {:.1}s", self.slowmo_timer);
            draw_card_rect(p_x, p_y, 100.0, 22.0, SURFACE_COLOR, Color::new(0.65, 0.40, 0.95, 1.0));
            draw_text(&s_str, p_x + 8.0, p_y + 16.0, 12.0, Color::new(0.65, 0.40, 0.95, 1.0));
            p_x += 110.0;
        }
        if self.magnet_timer > 0.0 {
            let m_str = format!("ÍMÃ {:.1}s", self.magnet_timer);
            draw_card_rect(p_x, p_y, 85.0, 22.0, SURFACE_COLOR, Color::new(0.95, 0.50, 0.15, 1.0));
            draw_text(&m_str, p_x + 8.0, p_y + 16.0, 12.0, Color::new(0.95, 0.50, 0.15, 1.0));
        }

        // =====================================================================
        // TELA DE PREPARAÇÃO (READY)
        // =====================================================================
        if self.state == PlayState::Ready {
            let title = "SNAKE CYBER NEON";
            let t_dim = measure_text(title, None, 28, 1.0);
            draw_text(title, ox + (800.0 - t_dim.width) * 0.5, oy + 210.0, 28.0, YELLOW_ACCENT);

            let prompt = "Pressione [ESPAÇO] ou qualquer seta para Iniciar";
            let p_dim = measure_text(prompt, None, 16, 1.0);
            draw_text(prompt, ox + (800.0 - p_dim.width) * 0.5, oy + 250.0, 16.0, TEXT_PRIMARY);

            let diff_prompt = "Dificuldade: [1] Casual  [2] Normal  [3] Cyberpunk";
            let d_dim = measure_text(diff_prompt, None, 14, 1.0);
            draw_text(diff_prompt, ox + (800.0 - d_dim.width) * 0.5, oy + 380.0, 14.0, ICE_BLUE);
        }

        // =====================================================================
        // TELA DE GAME OVER
        // =====================================================================
        if let PlayState::GameOver { timer: _ } = &self.state {
            draw_rectangle(ox, oy, 800.0, 600.0, Color::new(0.04, 0.05, 0.09, 0.88));

            let box_w = 480.0;
            let box_h = 220.0;
            let bx = ox + (800.0 - box_w) * 0.5;
            let by = oy + (600.0 - box_h) * 0.5;
            draw_card_rect(bx, by, box_w, box_h, SURFACE_COLOR, CORAL);

            let go_title = "SISTEMA INTERROMPIDO!";
            let go_dim = measure_text(go_title, None, 24, 1.0);
            draw_text(go_title, bx + (box_w - go_dim.width) * 0.5, by + 45.0, 24.0, CORAL);

            let s_msg = format!("Pontuação Final: {} PONTOS", self.score);
            let s_dim = measure_text(&s_msg, None, 18, 1.0);
            draw_text(&s_msg, bx + (box_w - s_dim.width) * 0.5, by + 85.0, 18.0, YELLOW_ACCENT);

            let stats = format!("Comprimento da Serpente: {}  |  Fase: {}", self.body.len(), self.level);
            let st_dim = measure_text(&stats, None, 13, 1.0);
            draw_text(&stats, bx + (box_w - st_dim.width) * 0.5, by + 120.0, 13.0, TEXT_PRIMARY);

            let r_prompt = "Pressione [R] para Reiniciar ou [ESC] para Voltar";
            let r_dim = measure_text(r_prompt, None, 13, 1.0);
            draw_text(r_prompt, bx + (box_w - r_dim.width) * 0.5, by + 175.0, 13.0, TEXT_MUTED);
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
        "Snake Cyber Neon"
    }
}
