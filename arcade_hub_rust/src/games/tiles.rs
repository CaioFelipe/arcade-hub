use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

pub struct TilesGame {
    grid: [i32; 16],
    empty_pos: usize,
    moves: u32,
    score: u32,
    won: bool,
    game_over: bool,
}

impl TilesGame {
    pub fn new() -> Self {
        let mut game = Self {
            grid: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0],
            empty_pos: 15,
            moves: 0,
            score: 1000,
            won: false,
            game_over: false,
        };
        game.shuffle();
        game
    }

    fn shuffle(&mut self) {
        // Embaralhamento resolvível simulando movimentos válidos
        for _ in 0..120 {
            let row = self.empty_pos / 4;
            let col = self.empty_pos % 4;
            let mut neighbors = Vec::new();

            if row > 0 { neighbors.push(self.empty_pos - 4); }
            if row < 3 { neighbors.push(self.empty_pos + 4); }
            if col > 0 { neighbors.push(self.empty_pos - 1); }
            if col < 3 { neighbors.push(self.empty_pos + 1); }

            let target = neighbors[rand::gen_range(0, neighbors.len())];
            self.grid[self.empty_pos] = self.grid[target];
            self.grid[target] = 0;
            self.empty_pos = target;
        }
        self.moves = 0;
    }

    fn try_move(&mut self, target_idx: usize) -> bool {
        let row = self.empty_pos / 4;
        let col = self.empty_pos % 4;
        let trow = target_idx / 4;
        let tcol = target_idx % 4;

        let is_adjacent = (row == trow && (col as i32 - tcol as i32).abs() == 1) ||
                          (col == tcol && (row as i32 - trow as i32).abs() == 1);

        if is_adjacent {
            self.grid[self.empty_pos] = self.grid[target_idx];
            self.grid[target_idx] = 0;
            self.empty_pos = target_idx;
            self.moves += 1;
            self.score = self.score.saturating_sub(5);
            self.check_win();
            return true;
        }
        false
    }

    fn check_win(&mut self) {
        for i in 0..15 {
            if self.grid[i] != (i as i32 + 1) {
                return;
            }
        }
        self.won = true;
        self.game_over = true;
    }
}

impl ArcadeGame for TilesGame {
    fn update(&mut self) {
        if self.game_over {
            return;
        }

        let row = self.empty_pos / 4;
        let col = self.empty_pos % 4;

        // Setas
        if is_key_pressed(KeyCode::Up) && row < 3 {
            self.try_move(self.empty_pos + 4);
        } else if is_key_pressed(KeyCode::Down) && row > 0 {
            self.try_move(self.empty_pos - 4);
        } else if is_key_pressed(KeyCode::Left) && col < 3 {
            self.try_move(self.empty_pos + 1);
        } else if is_key_pressed(KeyCode::Right) && col > 0 {
            self.try_move(self.empty_pos - 1);
        }

        // Clique do Mouse
        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            let screen_w = screen_width();
            let screen_h = screen_height();
            let ox = (screen_w - 400.0) * 0.5;
            let oy = (screen_h - 400.0) * 0.5 + 30.0;
            let tile_size = 94.0;
            let gap = 8.0;

            for i in 0..16 {
                let r = i / 4;
                let c = i % 4;
                let tx = ox + (c as f32) * (tile_size + gap);
                let ty = oy + (r as f32) * (tile_size + gap);

                if mpos.0 >= tx && mpos.0 <= tx + tile_size && mpos.1 >= ty && mpos.1 <= ty + tile_size {
                    self.try_move(i);
                    break;
                }
            }
        }
    }

    fn draw(&self, screen_w: f32, screen_h: f32) {
        let ox = (screen_w - 800.0) * 0.5;
        let oy = (screen_h - 600.0) * 0.5 + 24.0;

        draw_rectangle(ox, oy, 800.0, 600.0, BG_COLOR);
        draw_rectangle_lines(ox, oy, 800.0, 600.0, 1.0, BORDER_COLOR);

        // Moldura do Quebra-Cabeça
        let board_ox = (screen_w - 400.0) * 0.5;
        let board_oy = (screen_h - 400.0) * 0.5 + 30.0;
        let tile_size = 94.0;
        let gap = 8.0;

        draw_card_rect(board_ox - 10.0, board_oy - 10.0, 420.0, 420.0, SURFACE_COLOR, BORDER_COLOR);

        for i in 0..16 {
            let val = self.grid[i];
            let r = i / 4;
            let c = i % 4;
            let tx = board_ox + (c as f32) * (tile_size + gap);
            let ty = board_oy + (r as f32) * (tile_size + gap);

            if val > 0 {
                let is_correct = val == (i as i32 + 1);
                let bg_col = if is_correct { Color::new(0.09, 0.18, 0.35, 1.0) } else { SURFACE_HOVER };
                let border_col = if is_correct { ICE_BLUE } else { BORDER_COLOR };

                draw_card_rect(tx, ty, tile_size, tile_size, bg_col, border_col);

                let text = format!("{}", val);
                let dim = measure_text(&text, None, 32, 1.0);
                draw_text(
                    &text,
                    tx + (tile_size - dim.width) * 0.5,
                    ty + (tile_size + dim.height) * 0.5 - 2.0,
                    32.0,
                    if is_correct { YELLOW_ACCENT } else { TEXT_PRIMARY },
                );
            }
        }

        // HUD
        draw_text(&format!("MOVIMENTOS: {}", self.moves), ox + 40.0, oy + 40.0, 20.0, ICE_BLUE);
        draw_text(&format!("PONTUAÇÃO: {}", self.score), ox + 600.0, oy + 40.0, 20.0, YELLOW_ACCENT);

        if self.won {
            let msg = "PARABÉNS! QUEBRA-CABEÇA RESOLVIDO!";
            let dim = measure_text(msg, None, 24, 1.0);
            draw_text(msg, ox + (800.0 - dim.width) * 0.5, oy + 540.0, 24.0, EMERALD);
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
        "Memory Tiles"
    }
}
