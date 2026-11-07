use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    alive: bool,
}

struct Target {
    y: f32,
    speed: f32,
    radius: f32,
}

pub struct CannonGame {
    angle: f32,
    power: f32,
    balls: Vec<Ball>,
    targets: Vec<Target>,
    score: u32,
    game_over: bool,
    shots_left: i32,
}

impl CannonGame {
    pub fn new() -> Self {
        let targets = vec![
            Target { y: 150.0, speed: 2.0, radius: 24.0 },
            Target { y: 280.0, speed: -2.5, radius: 20.0 },
            Target { y: 400.0, speed: 3.0, radius: 26.0 },
        ];

        Self {
            angle: 45.0,
            power: 14.0,
            balls: Vec::new(),
            targets,
            score: 0,
            game_over: false,
            shots_left: 15,
        }
    }
}

impl ArcadeGame for CannonGame {
    fn update(&mut self) {
        if self.game_over {
            return;
        }

        // Ajuste de Ângulo
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.angle = (self.angle + 1.2).min(85.0);
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.angle = (self.angle - 1.2).max(10.0);
        }

        // Ajuste de Potência
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.power = (self.power + 0.2).min(24.0);
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.power = (self.power - 0.2).max(8.0);
        }

        // Disparo
        if (is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left)) && self.shots_left > 0 {
            self.shots_left -= 1;
            let rad = self.angle.to_radians();
            self.balls.push(Ball {
                x: 60.0 + rad.cos() * 45.0,
                y: 530.0 - rad.sin() * 45.0,
                vx: rad.cos() * self.power,
                vy: -rad.sin() * self.power,
                alive: true,
            });
        }

        // Atualiza Bolas
        for ball in &mut self.balls {
            if ball.alive {
                ball.x += ball.vx;
                ball.y += ball.vy;
                ball.vy += 0.35; // Gravidade

                if ball.x > 800.0 || ball.y > 600.0 {
                    ball.alive = false;
                }
            }
        }

        // Movimento Alvos
        for target in &mut self.targets {
            target.y += target.speed;
            if target.y <= 80.0 || target.y >= 500.0 {
                target.speed *= -1.0;
            }

            // Checa Colisão
            let tx = 680.0;
            for ball in &mut self.balls {
                if ball.alive {
                    let dx = ball.x - tx;
                    let dy = ball.y - target.y;
                    if (dx * dx + dy * dy).sqrt() < target.radius + 8.0 {
                        ball.alive = false;
                        self.score += 50;
                        target.y = rand::gen_range(100.0, 480.0);
                    }
                }
            }
        }

        self.balls.retain(|b| b.alive);

        if self.shots_left <= 0 && self.balls.is_empty() {
            self.game_over = true;
        }
    }

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let ox = (screen_w - 800.0) * 0.5;
        let oy = (screen_h - 600.0) * 0.5 + 24.0;

        draw_rectangle(ox, oy, 800.0, 600.0, BG_COLOR);
        draw_rectangle_lines(ox, oy, 800.0, 600.0, 1.0, BORDER_COLOR);

        // Chão
        draw_rectangle(ox, oy + 560.0, 800.0, 40.0, SURFACE_COLOR);
        draw_line(ox, oy + 560.0, ox + 800.0, oy + 560.0, 1.0, BORDER_COLOR);

        // Canhão
        let cx = ox + 60.0;
        let cy = oy + 540.0;
        let rad = self.angle.to_radians();
        let barrel_len = 45.0;
        draw_line(
            cx,
            cy,
            cx + rad.cos() * barrel_len,
            cy - rad.sin() * barrel_len,
            12.0,
            COBALT_ACCENT,
        );
        draw_circle(cx, cy, 18.0, ICE_BLUE);

        // Linha de mira sutil
        let mut sx = cx + rad.cos() * barrel_len;
        let mut sy = cy - rad.sin() * barrel_len;
        let svx = rad.cos() * self.power;
        let mut svy = -rad.sin() * self.power;
        for _ in 0..12 {
            sx += svx;
            sy += svy;
            svy += 0.35;
            draw_circle(sx, sy, 2.0, Color::new(0.38, 0.65, 0.98, 0.3));
        }

        // Alvos
        let tx = ox + 680.0;
        for target in &self.targets {
            draw_circle(tx, oy + target.y, target.radius, CORAL);
            draw_circle(tx, oy + target.y, target.radius * 0.6, TEXT_PRIMARY);
            draw_circle(tx, oy + target.y, target.radius * 0.3, YELLOW_ACCENT);
        }

        // Bolas de Canhão
        for ball in &self.balls {
            if ball.alive {
                draw_circle(ox + ball.x, oy + ball.y, 8.0, YELLOW_ACCENT);
                draw_circle(ox + ball.x, oy + ball.y, 4.0, TEXT_PRIMARY);
            }
        }

        // HUD Disparos
        draw_text(&format!("DISPAROS: {}", self.shots_left), ox + 30.0, oy + 35.0, 18.0, TEXT_MUTED);
        draw_text(&format!("ÂNGULO: {:.0}°", self.angle), ox + 180.0, oy + 35.0, 18.0, ICE_BLUE);
        draw_text(&format!("FORÇA: {:.0}", self.power), ox + 320.0, oy + 35.0, 18.0, YELLOW_ACCENT);
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
        "Cannon Ballistics"
    }
}
