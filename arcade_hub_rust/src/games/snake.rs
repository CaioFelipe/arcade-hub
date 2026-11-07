use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

struct FoodParticle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    color: Color,
}

struct FloatText {
    x: f32,
    y: f32,
    text: String,
    alpha: f32,
}

pub struct SnakeGame {
    body: Vec<(i32, i32)>,
    dir: (i32, i32),
    next_dir: (i32, i32),
    apple: (i32, i32),
    score: u32,
    timer: u32,
    speed: u32,
    game_over: bool,
    particles: Vec<FoodParticle>,
    float_texts: Vec<FloatText>,
    time: f32,
}

impl SnakeGame {
    pub fn new() -> Self {
        let mut game = Self {
            body: vec![(20, 15), (19, 15), (18, 15)],
            dir: (1, 0),
            next_dir: (1, 0),
            apple: (10, 10),
            score: 0,
            timer: 0,
            speed: 6,
            game_over: false,
            particles: Vec::new(),
            float_texts: Vec::new(),
            time: 0.0,
        };
        game.spawn_apple();
        game
    }

    fn spawn_apple(&mut self) {
        loop {
            let x = rand::gen_range(1, 39);
            let y = rand::gen_range(1, 29);
            if !self.body.contains(&(x, y)) {
                self.apple = (x, y);
                break;
            }
        }
    }
}

impl ArcadeGame for SnakeGame {
    fn update(&mut self) {
        self.time += get_frame_time();

        // Atualiza partículas
        self.particles.retain_mut(|p| {
            p.x += p.vx;
            p.y += p.vy;
            p.life -= 0.03;
            p.life > 0.0
        });

        // Atualiza textos flutuantes
        self.float_texts.retain_mut(|ft| {
            ft.y -= 0.8;
            ft.alpha -= 0.02;
            ft.alpha > 0.0
        });

        if self.game_over {
            return;
        }

        // Controles de direção com buffer
        if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::W)) && self.dir != (0, 1) {
            self.next_dir = (0, -1);
        } else if (is_key_down(KeyCode::Down) || is_key_down(KeyCode::S)) && self.dir != (0, -1) {
            self.next_dir = (0, 1);
        } else if (is_key_down(KeyCode::Left) || is_key_down(KeyCode::A)) && self.dir != (1, 0) {
            self.next_dir = (-1, 0);
        } else if (is_key_down(KeyCode::Right) || is_key_down(KeyCode::D)) && self.dir != (-1, 0) {
            self.next_dir = (1, 0);
        }

        self.timer += 1;
        let curr_speed = self.speed.saturating_sub(self.score / 60).max(2);
        if self.timer >= curr_speed {
            self.timer = 0;
            self.dir = self.next_dir;
            let (hx, hy) = self.body[0];
            let new_head = ((hx + self.dir.0 + 40) % 40, (hy + self.dir.1 + 30) % 30);

            // Colisão com o próprio corpo
            if self.body.contains(&new_head) {
                self.game_over = true;
                // Partículas de colisão
                let hpx = (hx as f32) * 20.0 + 10.0;
                let hpy = (hy as f32) * 20.0 + 10.0;
                for _ in 0..25 {
                    self.particles.push(FoodParticle {
                        x: hpx,
                        y: hpy,
                        vx: rand::gen_range(-3.5, 3.5),
                        vy: rand::gen_range(-3.5, 3.5),
                        life: 1.0,
                        color: CORAL,
                    });
                }
                return;
            }

            self.body.insert(0, new_head);

            if new_head == self.apple {
                self.score += 10;
                let apx = (self.apple.0 as f32) * 20.0 + 10.0;
                let apy = (self.apple.1 as f32) * 20.0 + 10.0;

                // Partículas douradas e menta ao comer a maçã
                for _ in 0..18 {
                    self.particles.push(FoodParticle {
                        x: apx,
                        y: apy,
                        vx: rand::gen_range(-2.8, 2.8),
                        vy: rand::gen_range(-2.8, 2.8),
                        life: 0.8,
                        color: if rand::gen_range(0, 2) == 0 { YELLOW_ACCENT } else { EMERALD },
                    });
                }

                // Texto flutuante +10
                self.float_texts.push(FloatText {
                    x: apx,
                    y: apy - 10.0,
                    text: "+10".to_string(),
                    alpha: 1.0,
                });

                self.spawn_apple();
            } else {
                self.body.pop();
            }
        }
    }

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let ox = (screen_w - 800.0) * 0.5;
        let oy = (screen_h - 600.0) * 0.5 + 24.0;
        let cell = 20.0;

        // Fundo do tabuleiro com moldura neon
        draw_rectangle(ox, oy, 800.0, 600.0, BG_COLOR);
        draw_rectangle_lines(ox, oy, 800.0, 600.0, 1.5, BORDER_COLOR);

        // Grade sutil com pontos de vértice estilo radar retrô
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

        // Partículas
        for p in &self.particles {
            draw_circle(ox + p.x, oy + p.y, p.life * 4.5, p.color);
        }

        // Maçã Pulsante Neon com folha verde
        let ax = ox + (self.apple.0 as f32) * cell + cell * 0.5;
        let ay = oy + (self.apple.1 as f32) * cell + cell * 0.5;
        let pulse = (self.time * 6.0).sin() * 2.0;

        // Halo de brilho da maçã
        draw_circle(ax, ay, (cell * 0.45) + pulse + 4.0, Color::new(0.96, 0.44, 0.44, 0.15));
        // Corpo da maçã
        draw_circle(ax, ay, cell * 0.42 + pulse * 0.5, CORAL);
        // Ponto de luz da maçã
        draw_circle(ax - 2.5, ay - 2.5, 2.5, Color::new(1.0, 0.85, 0.85, 0.9));
        // Folha verde da maçã
        draw_circle(ax + 2.0, ay - 7.0, 2.0, EMERALD);

        // Cobra: Desenho com anéis arredondados e gradiente de cor da cauda até a cabeça
        let total = self.body.len().max(1) as f32;
        for (idx, &(bx, by)) in self.body.iter().enumerate().rev() {
            let cx = ox + (bx as f32) * cell + cell * 0.5;
            let cy = oy + (by as f32) * cell + cell * 0.5;
            let factor = 1.0 - (idx as f32 / total);

            if idx == 0 {
                // Cabeça da cobra: Círculo maior com iluminação
                let head_color = ICE_BLUE;
                draw_circle(cx, cy, cell * 0.48, head_color);

                // Olhos que olham na direção do movimento
                let eye_offset_x = (self.dir.0 as f32) * 3.5;
                let eye_offset_y = (self.dir.1 as f32) * 3.5;

                // Dois olhos laterais
                let (e1_ox, e1_oy, e2_ox, e2_oy) = if self.dir.0 != 0 {
                    (0.0, -4.0, 0.0, 4.0)
                } else {
                    (-4.0, 0.0, 4.0, 0.0)
                };

                // Esclera branca
                draw_circle(cx + eye_offset_x + e1_ox, cy + eye_offset_y + e1_oy, 3.2, TEXT_PRIMARY);
                draw_circle(cx + eye_offset_x + e2_ox, cy + eye_offset_y + e2_oy, 3.2, TEXT_PRIMARY);

                // Pupila escura
                draw_circle(cx + eye_offset_x * 1.3 + e1_ox, cy + eye_offset_y * 1.3 + e1_oy, 1.8, BG_COLOR);
                draw_circle(cx + eye_offset_x * 1.3 + e2_ox, cy + eye_offset_y * 1.3 + e2_oy, 1.8, BG_COLOR);
            } else {
                // Corpo com transição de cor suave Safira -> Cobalto
                let body_r = cell * (0.36 + factor * 0.08);
                let col = Color::new(
                    0.15 + factor * 0.22,
                    0.38 + factor * 0.26,
                    0.92,
                    0.92,
                );
                draw_circle(cx, cy, body_r, col);
                // Núcleo luminoso interno
                draw_circle(cx, cy, body_r * 0.5, Color::new(0.6, 0.8, 1.0, 0.35));
            }
        }

        // Textos Flutuantes de Pontuação (+10)
        for ft in &self.float_texts {
            draw_text(
                &ft.text,
                ox + ft.x - 10.0,
                oy + ft.y,
                18.0,
                Color::new(1.0, 0.8, 0.2, ft.alpha),
            );
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
        "Snake Arcade"
    }
}
