use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

pub struct Connect4Game {
    board: [[i32; 7]; 6], // 0: vazio, 1: azul, 2: vermelho (IA)
    current_col: usize,
    turn: i32,
    score: u32,
    winner: i32,
    game_over: bool,
}

impl Connect4Game {
    pub fn new() -> Self {
        Self {
            board: [[0; 7]; 6],
            current_col: 3,
            turn: 1,
            score: 0,
            winner: 0,
            game_over: false,
        }
    }

    fn drop_chip(&mut self, col: usize, player: i32) -> bool {
        for r in (0..6).rev() {
            if self.board[r][col] == 0 {
                self.board[r][col] = player;
                if self.check_win(r, col, player) {
                    self.winner = player;
                    self.game_over = true;
                    if player == 1 {
                        self.score += 200;
                    }
                }
                return true;
            }
        }
        false
    }

    fn check_win(&self, r: usize, c: usize, p: i32) -> bool {
        let directions = [(0, 1), (1, 0), (1, 1), (1, -1)];
        for (dr, dc) in directions {
            let mut count = 1;
            // Positivo
            let mut step = 1;
            while step <= 3 {
                let nr = r as i32 + dr * step;
                let nc = c as i32 + dc * step;
                if nr >= 0 && nr < 6 && nc >= 0 && nc < 7 && self.board[nr as usize][nc as usize] == p {
                    count += 1;
                    step += 1;
                } else {
                    break;
                }
            }
            // Negativo
            step = 1;
            while step <= 3 {
                let nr = r as i32 - dr * step;
                let nc = c as i32 - dc * step;
                if nr >= 0 && nr < 6 && nc >= 0 && nc < 7 && self.board[nr as usize][nc as usize] == p {
                    count += 1;
                    step += 1;
                } else {
                    break;
                }
            }
            if count >= 4 {
                return true;
            }
        }
        false
    }

    fn ai_play(&mut self) {
        if self.game_over {
            return;
        }
        // IA tenta jogar na coluna que der vitória ou na coluna central
        let cols_order = [3, 2, 4, 1, 5, 0, 6];
        for &col in &cols_order {
            if self.drop_chip(col, 2) {
                self.turn = 1;
                break;
            }
        }
    }
}

impl ArcadeGame for Connect4Game {
    fn update(&mut self) {
        if self.game_over {
            return;
        }

        if self.turn == 1 {
            if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
                if self.current_col > 0 { self.current_col -= 1; }
            }
            if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
                if self.current_col < 6 { self.current_col += 1; }
            }
            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Down) || is_mouse_button_pressed(MouseButton::Left) {
                if self.drop_chip(self.current_col, 1) {
                    if !self.game_over {
                        self.turn = 2;
                        self.ai_play();
                    }
                }
            }
        }
    }

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let ox = (screen_w - 800.0) * 0.5;
        let oy = (screen_h - 600.0) * 0.5 + 24.0;

        draw_rectangle(ox, oy, 800.0, 600.0, BG_COLOR);
        draw_rectangle_lines(ox, oy, 800.0, 600.0, 1.0, BORDER_COLOR);

        // Tabuleiro
        let board_ox = (screen_w - 560.0) * 0.5;
        let board_oy = (screen_h - 480.0) * 0.5 + 40.0;
        let slot_size = 72.0;
        let gap = 8.0;

        // Indicador da coluna do jogador
        let ind_x = board_ox + (self.current_col as f32) * (slot_size + gap) + slot_size * 0.5;
        draw_circle(ind_x, board_oy - 25.0, 14.0, ICE_BLUE);
        draw_triangle(
            Vec2::new(ind_x - 8.0, board_oy - 10.0),
            Vec2::new(ind_x + 8.0, board_oy - 10.0),
            Vec2::new(ind_x, board_oy - 2.0),
            ICE_BLUE,
        );

        draw_card_rect(board_ox - 12.0, board_oy - 12.0, 584.0, 504.0, SURFACE_COLOR, BORDER_COLOR);

        for r in 0..6 {
            for c in 0..7 {
                let cx = board_ox + (c as f32) * (slot_size + gap) + slot_size * 0.5;
                let cy = board_oy + (r as f32) * (slot_size + gap) + slot_size * 0.5;

                let (bg, border) = match self.board[r][c] {
                    1 => (COBALT_ACCENT, ICE_BLUE),
                    2 => (CORAL, YELLOW_ACCENT),
                    _ => (BG_COLOR, BORDER_COLOR),
                };

                draw_circle(cx, cy, slot_size * 0.44, bg);
                draw_circle_lines(cx, cy, slot_size * 0.44, 1.5, border);
            }
        }

        // HUD
        let turn_msg = if self.turn == 1 { "SUA VEZ (AZUL)" } else { "VEZ DA IA (VERMELHO)" };
        let turn_col = if self.turn == 1 { ICE_BLUE } else { CORAL };
        draw_text(turn_msg, ox + 40.0, oy + 40.0, 20.0, turn_col);

        if self.game_over {
            let msg = if self.winner == 1 { "VITÓRIA DO JOGADOR AZUL!" } else { "A IA VENCEU A PARTIDA!" };
            let col = if self.winner == 1 { EMERALD } else { CORAL };
            let dim = measure_text(msg, None, 24, 1.0);
            draw_text(msg, ox + (800.0 - dim.width) * 0.5, oy + 575.0, 24.0, col);
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
        "Four in a Row"
    }
}
