use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

struct Pipe {
    x: f32,
    gap_y: f32,
    scored: bool,
}

pub struct FlappyGame {
    bird_y: f32,
    velocity: f32,
    pipes: Vec<Pipe>,
    score: u32,
    game_over: bool,
    spawn_timer: u32,
}

impl FlappyGame {
    pub fn new() -> Self {
        let mut game = Self {
            bird_y: 300.0,
            velocity: 0.0,
            pipes: Vec::new(),
            score: 0,
            game_over: false,
            spawn_timer: 0,
        };
        game.spawn_pipe();
        game
    }

    fn spawn_pipe(&mut self) {
        self.pipes.push(Pipe {
            x: 850.0,
            gap_y: rand::gen_range(150.0, 450.0),
            scored: false,
        });
    }
}

impl ArcadeGame for FlappyGame {
    fn update(&mut self) {
        if self.game_over {
            return;
        }

        // Pulo / Impulso
        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
            self.velocity = -7.5;
        }

        // Gravidade
        self.velocity += 0.38;
        self.bird_y += self.velocity;

        // Limite superior / chão
        if self.bird_y <= 20.0 {
            self.bird_y = 20.0;
            self.velocity = 0.0;
        }
        if self.bird_y >= 570.0 {
            self.game_over = true;
        }

        // Spawn canos
        self.spawn_timer += 1;
        if self.spawn_timer >= 95 {
            self.spawn_timer = 0;
            self.spawn_pipe();
        }

        // Movimento canos
        let bird_x = 200.0;
        let gap_size = 140.0;
        for pipe in &mut self.pipes {
            pipe.x -= 3.5;

            // Colisão
            if (bird_x + 15.0 >= pipe.x && bird_x - 15.0 <= pipe.x + 55.0) &&
               (self.bird_y - 12.0 <= pipe.gap_y - gap_size * 0.5 || self.bird_y + 12.0 >= pipe.gap_y + gap_size * 0.5) {
                self.game_over = true;
            }

            // Ponto
            if !pipe.scored && pipe.x < bird_x {
                pipe.scored = true;
                self.score += 10;
            }
        }

        self.pipes.retain(|p| p.x > -80.0);
    }

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let ox = (screen_w - 800.0) * 0.5;
        let oy = (screen_h - 600.0) * 0.5 + 24.0;
        let gap_size = 140.0;

        draw_rectangle(ox, oy, 800.0, 600.0, BG_COLOR);
        draw_rectangle_lines(ox, oy, 800.0, 600.0, 1.0, BORDER_COLOR);

        // Canos
        for pipe in &self.pipes {
            let px = ox + pipe.x;
            let top_h = pipe.gap_y - gap_size * 0.5;
            let bot_y = pipe.gap_y + gap_size * 0.5;

            // Cano Superior
            draw_rectangle(px, oy, 55.0, top_h, SURFACE_HOVER);
            draw_rectangle_lines(px, oy, 55.0, top_h, 1.5, EMERALD);
            draw_rectangle(px - 4.0, oy + top_h - 18.0, 63.0, 18.0, EMERALD);

            // Cano Inferior
            let bot_h = 600.0 - bot_y;
            draw_rectangle(px, oy + bot_y, 55.0, bot_h, SURFACE_HOVER);
            draw_rectangle_lines(px, oy + bot_y, 55.0, bot_h, 1.5, EMERALD);
            draw_rectangle(px - 4.0, oy + bot_y, 63.0, 18.0, EMERALD);
        }

        // Pássaro / Jetpack (Vetor estilizado)
        let bx = ox + 200.0;
        let by = oy + self.bird_y;

        // Fogo da propulsão
        if self.velocity < 0.0 {
            draw_triangle(
                Vec2::new(bx - 14.0, by - 4.0),
                Vec2::new(bx - 14.0, by + 4.0),
                Vec2::new(bx - 26.0, by),
                YELLOW_ACCENT,
            );
        }

        // Corpo
        draw_circle(bx, by, 14.0, COBALT_ACCENT);
        draw_circle(bx + 4.0, by - 3.0, 5.0, TEXT_PRIMARY);
        draw_circle(bx + 5.0, by - 3.0, 2.5, BG_COLOR);
        // Bico / Viseira
        draw_triangle(
            Vec2::new(bx + 8.0, by - 2.0),
            Vec2::new(bx + 8.0, by + 6.0),
            Vec2::new(bx + 18.0, by + 2.0),
            YELLOW_ACCENT,
        );
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
        "Flappy Jetpack"
    }
}
