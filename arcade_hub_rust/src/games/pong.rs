use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

struct PongParticle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    color: Color,
}

pub struct PongGame {
    player_y: f32,
    ai_y: f32,
    ball_x: f32,
    ball_y: f32,
    ball_vx: f32,
    ball_vy: f32,
    ball_trail: Vec<(f32, f32)>,
    particles: Vec<PongParticle>,
    score: u32,
    ai_score: u32,
    rally_count: u32,
    game_over: bool,
    time: f32,
}

impl PongGame {
    pub fn new() -> Self {
        Self {
            player_y: 250.0,
            ai_y: 250.0,
            ball_x: 400.0,
            ball_y: 300.0,
            ball_vx: 6.5,
            ball_vy: 3.2,
            ball_trail: Vec::new(),
            particles: Vec::new(),
            score: 0,
            ai_score: 0,
            rally_count: 0,
            game_over: false,
            time: 0.0,
        }
    }
}

impl ArcadeGame for PongGame {
    fn update(&mut self) {
        self.time += get_frame_time();

        // Partículas
        self.particles.retain_mut(|p| {
            p.x += p.vx;
            p.y += p.vy;
            p.life -= 0.04;
            p.life > 0.0
        });

        if self.game_over {
            return;
        }

        // Movimento do Jogador (Esquerda)
        let speed = 8.5;
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.player_y = (self.player_y - speed).max(30.0);
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.player_y = (self.player_y + speed).min(470.0);
        }

        // IA Competitiva com atraso de reação humanizado
        let ai_target = self.ball_y - 50.0;
        let ai_speed = (5.5 + (self.rally_count as f32 * 0.15)).min(7.8);
        if (self.ai_y - ai_target).abs() > 8.0 {
            if self.ai_y < ai_target {
                self.ai_y = (self.ai_y + ai_speed).min(470.0);
            } else {
                self.ai_y = (self.ai_y - ai_speed).max(30.0);
            }
        }

        // Rastro cometa da bola
        self.ball_trail.push((self.ball_x, self.ball_y));
        if self.ball_trail.len() > 8 {
            self.ball_trail.remove(0);
        }

        // Movimentação da bola
        self.ball_x += self.ball_vx;
        self.ball_y += self.ball_vy;

        // Rebote nas paredes superior e inferior
        if self.ball_y <= 28.0 {
            self.ball_y = 28.0;
            self.ball_vy = self.ball_vy.abs();
            self.spawn_impact(self.ball_x, self.ball_y, ICE_BLUE);
        } else if self.ball_y >= 572.0 {
            self.ball_y = 572.0;
            self.ball_vy = -self.ball_vy.abs();
            self.spawn_impact(self.ball_x, self.ball_y, ICE_BLUE);
        }

        // Colisão com Raquete Jogador (Esquerda: x = 50.0, w = 14.0, h = 100.0)
        let ph = 100.0;
        if self.ball_x >= 40.0 && self.ball_x <= 58.0 &&
           self.ball_y >= self.player_y - 6.0 && self.ball_y <= self.player_y + ph + 6.0 &&
           self.ball_vx < 0.0
        {
            self.rally_count += 1;
            let current_speed = (6.5 + (self.rally_count as f32 * 0.3)).min(14.0);
            self.ball_vx = current_speed;
            let offset = (self.ball_y - (self.player_y + ph * 0.5)) / (ph * 0.5);
            self.ball_vy = offset * (current_speed * 0.9);
            self.score += 10;
            self.spawn_impact(54.0, self.ball_y, ICE_BLUE);
        }

        // Colisão com Raquete IA (Direita: x = 736.0, w = 14.0, h = 100.0)
        if self.ball_x >= 730.0 && self.ball_x <= 748.0 &&
           self.ball_y >= self.ai_y - 6.0 && self.ball_y <= self.ai_y + ph + 6.0 &&
           self.ball_vx > 0.0
        {
            self.rally_count += 1;
            let current_speed = (6.5 + (self.rally_count as f32 * 0.3)).min(14.0);
            self.ball_vx = -current_speed;
            let offset = (self.ball_y - (self.ai_y + ph * 0.5)) / (ph * 0.5);
            self.ball_vy = offset * (current_speed * 0.9);
            self.spawn_impact(736.0, self.ball_y, CORAL);
        }

        // Ponto da IA
        if self.ball_x < 0.0 {
            self.ai_score += 1;
            self.rally_count = 0;
            self.ball_x = 400.0;
            self.ball_y = 300.0;
            self.ball_vx = 6.5;
            self.ball_vy = rand::gen_range(-3.5, 3.5);
            self.ball_trail.clear();
            if self.ai_score >= 5 {
                self.game_over = true;
            }
        }
        // Ponto do Jogador
        else if self.ball_x > 800.0 {
            self.score += 100;
            self.rally_count = 0;
            self.ball_x = 400.0;
            self.ball_y = 300.0;
            self.ball_vx = -6.5;
            self.ball_vy = rand::gen_range(-3.5, 3.5);
            self.ball_trail.clear();
            if self.score >= 500 {
                self.game_over = true;
            }
        }
    }

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let ox = (screen_w - 800.0) * 0.5;
        let oy = (screen_h - 600.0) * 0.5 + 24.0;

        // Fundo da arena Cyberpunk
        draw_rectangle(ox, oy, 800.0, 600.0, BG_COLOR);
        draw_rectangle_lines(ox, oy, 800.0, 600.0, 1.5, BORDER_COLOR);

        // Linha divisória central neon tracejada
        for i in 0..15 {
            draw_rectangle(ox + 398.0, oy + 25.0 + (i as f32) * 38.0, 4.0, 20.0, Color::new(0.38, 0.65, 0.98, 0.25));
        }

        // Círculo central da quadra
        draw_circle_lines(ox + 400.0, oy + 300.0, 60.0, 1.0, Color::new(0.38, 0.65, 0.98, 0.15));

        // Placar gigante translúcido de fundo
        let p_score = format!("{}", self.score / 100);
        let ai_score_str = format!("{}", self.ai_score);
        draw_text(&p_score, ox + 320.0, oy + 120.0, 80.0, Color::new(0.38, 0.65, 0.98, 0.15));
        draw_text(&ai_score_str, ox + 440.0, oy + 120.0, 80.0, Color::new(0.96, 0.44, 0.44, 0.15));

        // Partículas de impacto
        for p in &self.particles {
            draw_circle(ox + p.x, oy + p.y, p.life * 4.0, p.color);
        }

        // Rastro cometa da bola
        for (i, &(tx, ty)) in self.ball_trail.iter().enumerate() {
            let alpha = (i as f32 / self.ball_trail.len() as f32) * 0.35;
            draw_circle(ox + tx, oy + ty, 3.0 + (i as f32 * 0.4), Color::new(0.38, 0.65, 0.98, alpha));
        }

        // Bola de energia
        let bx = ox + self.ball_x;
        let by = oy + self.ball_y;
        draw_circle(bx, by, 8.0, Color::new(0.38, 0.65, 0.98, 0.25));
        draw_circle(bx, by, 6.0, TEXT_PRIMARY);
        draw_circle(bx - 1.5, by - 1.5, 2.0, ICE_BLUE);

        // Raquete do Jogador (Azul Safira / Cobalto)
        let px = ox + 42.0;
        let py = oy + self.player_y;
        let pw = 14.0;
        let ph = 100.0;
        draw_rectangle(px, py, pw, ph, COBALT_ACCENT);
        draw_rectangle(px + pw - 3.0, py, 3.0, ph, ICE_BLUE);
        draw_rectangle_lines(px, py, pw, ph, 1.0, Color::new(0.5, 0.8, 1.0, 0.4));

        // Raquete da IA (Coral Vermelho Neon)
        let aix = ox + 744.0;
        let aiy = oy + self.ai_y;
        draw_rectangle(aix, aiy, pw, ph, Color::new(0.75, 0.2, 0.25, 1.0));
        draw_rectangle(aix, aiy, 3.0, ph, CORAL);
        draw_rectangle_lines(aix, aiy, pw, ph, 1.0, Color::new(1.0, 0.4, 0.4, 0.4));
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
        "Pong Cyberpunk"
    }
}

impl PongGame {
    fn spawn_impact(&mut self, x: f32, y: f32, color: Color) {
        for _ in 0..10 {
            self.particles.push(PongParticle {
                x,
                y,
                vx: rand::gen_range(-3.0, 3.0),
                vy: rand::gen_range(-3.0, 3.0),
                color,
                life: 0.5,
            });
        }
    }
}
