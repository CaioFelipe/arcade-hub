use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

struct Brick {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: Color,
    alive: bool,
    points: u32,
}

struct BreakoutParticle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
    life: f32,
}

struct FloatText {
    x: f32,
    y: f32,
    text: String,
    alpha: f32,
}

pub struct BreakoutGame {
    paddle_x: f32,
    ball_x: f32,
    ball_y: f32,
    ball_vx: f32,
    ball_vy: f32,
    ball_trail: Vec<(f32, f32)>,
    bricks: Vec<Brick>,
    particles: Vec<BreakoutParticle>,
    float_texts: Vec<FloatText>,
    score: u32,
    lives: i32,
    game_over: bool,
    time: f32,
}

impl BreakoutGame {
    pub fn new() -> Self {
        let mut bricks = Vec::new();
        let cols = 10;
        let rows = 5;
        let bw = 70.0;
        let bh = 22.0;

        let row_colors = [
            (CORAL, 50),
            (YELLOW_ACCENT, 40),
            (EMERALD, 30),
            (ICE_BLUE, 20),
            (COBALT_ACCENT, 10),
        ];

        for r in 0..rows {
            let (color, pts) = row_colors[r % row_colors.len()];
            for c in 0..cols {
                bricks.push(Brick {
                    x: 45.0 + (c as f32) * (bw + 6.0),
                    y: 75.0 + (r as f32) * (bh + 7.0),
                    w: bw,
                    h: bh,
                    color,
                    alive: true,
                    points: pts,
                });
            }
        }

        Self {
            paddle_x: 350.0,
            ball_x: 400.0,
            ball_y: 490.0,
            ball_vx: 5.0,
            ball_vy: -5.0,
            ball_trail: Vec::new(),
            bricks,
            particles: Vec::new(),
            float_texts: Vec::new(),
            score: 0,
            lives: 3,
            game_over: false,
            time: 0.0,
        }
    }
}

impl ArcadeGame for BreakoutGame {
    fn update(&mut self) {
        self.time += get_frame_time();

        // Atualiza partículas
        self.particles.retain_mut(|p| {
            p.x += p.vx;
            p.y += p.vy;
            p.life -= 0.035;
            p.life > 0.0
        });

        // Atualiza textos flutuantes
        self.float_texts.retain_mut(|ft| {
            ft.y -= 0.9;
            ft.alpha -= 0.025;
            ft.alpha > 0.0
        });

        if self.game_over {
            return;
        }

        // Movimento da raquete com controle suave
        let speed = 9.5;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.paddle_x = (self.paddle_x - speed).max(40.0);
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.paddle_x = (self.paddle_x + speed).min(660.0);
        }

        // Histórico de rastro da bola
        self.ball_trail.push((self.ball_x, self.ball_y));
        if self.ball_trail.len() > 8 {
            self.ball_trail.remove(0);
        }

        // Atualizar física da bola
        self.ball_x += self.ball_vx;
        self.ball_y += self.ball_vy;

        // Paredes laterais com faíscas
        if self.ball_x <= 46.0 {
            self.ball_x = 46.0;
            self.ball_vx = self.ball_vx.abs();
            self.spawn_wall_sparks(self.ball_x, self.ball_y);
        } else if self.ball_x >= 754.0 {
            self.ball_x = 754.0;
            self.ball_vx = -self.ball_vx.abs();
            self.spawn_wall_sparks(self.ball_x, self.ball_y);
        }

        // Teto
        if self.ball_y <= 26.0 {
            self.ball_y = 26.0;
            self.ball_vy = self.ball_vy.abs();
            self.spawn_wall_sparks(self.ball_x, self.ball_y);
        }

        // Raquete (Física angular baseada na zona de impacto)
        let paddle_w = 100.0;
        let paddle_h = 16.0;
        let paddle_y = 538.0;

        if self.ball_y >= paddle_y - 8.0 && self.ball_y <= paddle_y + paddle_h &&
           self.ball_x >= self.paddle_x - 6.0 && self.ball_x <= self.paddle_x + paddle_w + 6.0 &&
           self.ball_vy > 0.0
        {
            self.ball_vy = -self.ball_vy.abs();
            let offset = (self.ball_x - (self.paddle_x + paddle_w * 0.5)) / (paddle_w * 0.5);
            self.ball_vx = (offset * 7.0).clamp(-8.0, 8.0);

            // Faíscas azuis de impacto na raquete
            for _ in 0..12 {
                self.particles.push(BreakoutParticle {
                    x: self.ball_x,
                    y: paddle_y,
                    vx: rand::gen_range(-3.0, 3.0),
                    vy: rand::gen_range(-4.0, -1.0),
                    color: ICE_BLUE,
                    life: 0.6,
                });
            }
        }

        // Colisão com Tijolos
        let mut hit_idx: Option<usize> = None;
        for (i, brick) in self.bricks.iter().enumerate() {
            if brick.alive {
                if self.ball_x >= brick.x - 7.0 && self.ball_x <= brick.x + brick.w + 7.0 &&
                   self.ball_y >= brick.y - 7.0 && self.ball_y <= brick.y + brick.h + 7.0
                {
                    hit_idx = Some(i);
                    break;
                }
            }
        }

        if let Some(idx) = hit_idx {
            let brick = &mut self.bricks[idx];
            brick.alive = false;
            self.score += brick.points;

            // Inversão vertical do vetor da bola
            self.ball_vy *= -1.0;

            // Explosão de partículas com a cor do bloco
            let bx = brick.x + brick.w * 0.5;
            let by = brick.y + brick.h * 0.5;
            for _ in 0..18 {
                self.particles.push(BreakoutParticle {
                    x: bx,
                    y: by,
                    vx: rand::gen_range(-4.0, 4.0),
                    vy: rand::gen_range(-4.0, 4.0),
                    color: brick.color,
                    life: 0.7,
                });
            }

            // Texto flutuante de pontuação
            self.float_texts.push(FloatText {
                x: bx,
                y: by,
                text: format!("+{}", brick.points),
                alpha: 1.0,
            });
        }

        // Bola perdida na parte inferior
        if self.ball_y > 610.0 {
            self.lives -= 1;
            if self.lives <= 0 {
                self.game_over = true;
            } else {
                self.ball_x = self.paddle_x + 50.0;
                self.ball_y = 490.0;
                self.ball_vx = if rand::gen_range(0, 2) == 0 { 5.0 } else { -5.0 };
                self.ball_vy = -5.0;
                self.ball_trail.clear();
            }
        }

        // Vitória se todos os tijolos forem destruídos
        if self.bricks.iter().all(|b| !b.alive) {
            self.game_over = true;
        }
    }

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let ox = (screen_w - 800.0) * 0.5;
        let oy = (screen_h - 600.0) * 0.5 + 24.0;

        // Fundo com bordas neon
        draw_rectangle(ox, oy, 800.0, 600.0, BG_COLOR);
        draw_rectangle_lines(ox, oy, 800.0, 600.0, 1.5, BORDER_COLOR);

        // Grade de coordenadas de fundo
        for i in 0..10 {
            draw_line(ox + 40.0 + (i as f32) * 72.0, oy + 20.0, ox + 40.0 + (i as f32) * 72.0, oy + 580.0, 1.0, Color::new(1.0, 1.0, 1.0, 0.015));
        }

        // Partículas de explosão
        for p in &self.particles {
            draw_circle(ox + p.x, oy + p.y, p.life * 4.0, p.color);
        }

        // Tijolos com estilo 3D Neon e chanfro superior
        for b in &self.bricks {
            if b.alive {
                let bx = ox + b.x;
                let by = oy + b.y;

                // Sombra suave do bloco
                draw_rectangle(bx + 2.0, by + 3.0, b.w, b.h, Color::new(0.0, 0.0, 0.0, 0.4));

                // Corpo do bloco
                draw_rectangle(bx, by, b.w, b.h, b.color);

                // Brilho do topo (efeito vidro 3D)
                draw_rectangle(bx, by, b.w, 4.0, Color::new(1.0, 1.0, 1.0, 0.45));

                // Borda de contraste
                draw_rectangle_lines(bx, by, b.w, b.h, 1.0, Color::new(1.0, 1.0, 1.0, 0.25));
            }
        }

        // Rastro cometa da bola
        for (i, &(tx, ty)) in self.ball_trail.iter().enumerate() {
            let alpha = (i as f32 / self.ball_trail.len() as f32) * 0.4;
            let r = 3.0 + (i as f32 * 0.4);
            draw_circle(ox + tx, oy + ty, r, Color::new(0.38, 0.65, 0.98, alpha));
        }

        // Bola com núcleo brilhante
        let bx = ox + self.ball_x;
        let by = oy + self.ball_y;
        draw_circle(bx, by, 8.0, Color::new(0.38, 0.65, 0.98, 0.25)); // Halo
        draw_circle(bx, by, 6.0, TEXT_PRIMARY);
        draw_circle(bx - 1.5, by - 1.5, 2.0, ICE_BLUE);

        // Raquete do Jogador: Barra aerodinâmica metálica com iluminação Cobalto
        let px = ox + self.paddle_x;
        let py = oy + 538.0;
        let pw = 100.0;
        let ph = 16.0;

        // Base da raquete
        draw_rectangle(px, py, pw, ph, COBALT_ACCENT);
        // Friso neon superior
        draw_rectangle(px + 4.0, py, pw - 8.0, 3.0, ICE_BLUE);
        // Indicador central
        draw_circle(px + pw * 0.5, py + ph * 0.5, 3.5, EMERALD);
        draw_rectangle_lines(px, py, pw, ph, 1.0, Color::new(0.5, 0.7, 1.0, 0.5));

        // Textos flutuantes (+50, +40)
        for ft in &self.float_texts {
            draw_text(
                &ft.text,
                ox + ft.x - 12.0,
                oy + ft.y,
                18.0,
                Color::new(1.0, 0.85, 0.2, ft.alpha),
            );
        }

        // Indicador de vidas no canto superior direito
        for i in 0..3 {
            let col = if i < self.lives { CORAL } else { Color::new(0.2, 0.25, 0.35, 0.5) };
            draw_circle(ox + 730.0 + (i as f32) * 20.0, oy + 40.0, 6.0, col);
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

impl BreakoutGame {
    fn spawn_wall_sparks(&mut self, x: f32, y: f32) {
        for _ in 0..8 {
            self.particles.push(BreakoutParticle {
                x,
                y,
                vx: rand::gen_range(-2.5, 2.5),
                vy: rand::gen_range(-2.5, 2.5),
                color: ICE_BLUE,
                life: 0.45,
            });
        }
    }
}
