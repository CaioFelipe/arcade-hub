use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

pub struct TronGame {
    p1_pos: (i32, i32),
    p1_dir: (i32, i32),
    p1_trail: Vec<(i32, i32)>,
    ai_pos: (i32, i32),
    ai_dir: (i32, i32),
    ai_trail: Vec<(i32, i32)>,
    score: u32,
    timer: u32,
    game_over: bool,
    winner: i32, // 1: p1, 2: ai, 0: em jogo
}

impl TronGame {
    pub fn new() -> Self {
        Self {
            p1_pos: (20, 30),
            p1_dir: (1, 0),
            p1_trail: vec![(20, 30)],
            ai_pos: (60, 30),
            ai_dir: (-1, 0),
            ai_trail: vec![(60, 30)],
            score: 0,
            timer: 0,
            game_over: false,
            winner: 0,
        }
    }
}

impl ArcadeGame for TronGame {
    fn update(&mut self) {
        if self.game_over {
            return;
        }

        // P1 Controles
        if is_key_down(KeyCode::Up) && self.p1_dir != (0, 1) {
            self.p1_dir = (0, -1);
        } else if is_key_down(KeyCode::Down) && self.p1_dir != (0, -1) {
            self.p1_dir = (0, 1);
        } else if is_key_down(KeyCode::Left) && self.p1_dir != (1, 0) {
            self.p1_dir = (-1, 0);
        } else if is_key_down(KeyCode::Right) && self.p1_dir != (-1, 0) {
            self.p1_dir = (1, 0);
        }

        self.timer += 1;
        if self.timer >= 3 {
            self.timer = 0;

            // Movimento P1
            let next_p1 = (self.p1_pos.0 + self.p1_dir.0, self.p1_pos.1 + self.p1_dir.1);

            // IA simples (desvia de colisão imediata)
            let mut possible_dirs = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
            possible_dirs.retain(|d| *d != (-self.ai_dir.0, -self.ai_dir.1));
            
            // Prioriza manter a direção atual se for segura
            let curr_next = (self.ai_pos.0 + self.ai_dir.0, self.ai_pos.1 + self.ai_dir.1);
            let curr_safe = curr_next.0 >= 1 && curr_next.0 < 79 && curr_next.1 >= 1 && curr_next.1 < 59 &&
                !self.p1_trail.contains(&curr_next) && !self.ai_trail.contains(&curr_next);

            if !curr_safe {
                for d in possible_dirs {
                    let test_pos = (self.ai_pos.0 + d.0, self.ai_pos.1 + d.1);
                    if test_pos.0 >= 1 && test_pos.0 < 79 && test_pos.1 >= 1 && test_pos.1 < 59 &&
                       !self.p1_trail.contains(&test_pos) && !self.ai_trail.contains(&test_pos) {
                        self.ai_dir = d;
                        break;
                    }
                }
            }

            let next_ai = (self.ai_pos.0 + self.ai_dir.0, self.ai_pos.1 + self.ai_dir.1);

            // Checa Colisões P1
            if next_p1.0 <= 0 || next_p1.0 >= 80 || next_p1.1 <= 0 || next_p1.1 >= 60 ||
               self.p1_trail.contains(&next_p1) || self.ai_trail.contains(&next_p1) {
                self.game_over = true;
                self.winner = 2;
                return;
            }

            // Checa Colisões IA
            if next_ai.0 <= 0 || next_ai.0 >= 80 || next_ai.1 <= 0 || next_ai.1 >= 60 ||
               self.p1_trail.contains(&next_ai) || self.ai_trail.contains(&next_ai) {
                self.game_over = true;
                self.winner = 1;
                self.score += 150;
                return;
            }

            self.p1_pos = next_p1;
            self.p1_trail.push(next_p1);

            self.ai_pos = next_ai;
            self.ai_trail.push(next_ai);

            self.score += 1;
        }
    }

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let ox = (screen_w - 800.0) * 0.5;
        let oy = (screen_h - 600.0) * 0.5 + 24.0;
        let cell = 10.0;

        draw_rectangle(ox, oy, 800.0, 600.0, BG_COLOR);
        draw_rectangle_lines(ox, oy, 800.0, 600.0, 1.0, BORDER_COLOR);

        // Grade Neon
        for i in 0..80 {
            for j in 0..60 {
                if (i % 8 == 0) && (j % 8 == 0) {
                    draw_circle(ox + (i as f32) * cell, oy + (j as f32) * cell, 1.0, Color::new(1.0, 1.0, 1.0, 0.05));
                }
            }
        }

        // Rastro P1 (Azul Cobalto)
        for &(tx, ty) in &self.p1_trail {
            draw_rectangle(ox + (tx as f32) * cell, oy + (ty as f32) * cell, cell - 1.0, cell - 1.0, ICE_BLUE);
        }
        // Moto P1
        draw_rectangle(ox + (self.p1_pos.0 as f32) * cell, oy + (self.p1_pos.1 as f32) * cell, cell, cell, TEXT_PRIMARY);

        // Rastro IA (Coral / Laranja)
        for &(tx, ty) in &self.ai_trail {
            draw_rectangle(ox + (tx as f32) * cell, oy + (ty as f32) * cell, cell - 1.0, cell - 1.0, CORAL);
        }
        // Moto IA
        draw_rectangle(ox + (self.ai_pos.0 as f32) * cell, oy + (self.ai_pos.1 as f32) * cell, cell, cell, YELLOW_ACCENT);

        if self.game_over {
            let msg = if self.winner == 1 { "VITÓRIA DO JOGADOR AZUL!" } else { "A IA VENCEU O DUELO!" };
            let col = if self.winner == 1 { EMERALD } else { CORAL };
            let dim = measure_text(msg, None, 28, 1.0);
            draw_text(msg, ox + (800.0 - dim.width) * 0.5, oy + 300.0, 28.0, col);
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
        "Tron Lightcycles"
    }
}
