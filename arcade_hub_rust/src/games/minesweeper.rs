use super::ArcadeGame;
use crate::theme::*;
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
struct Cell {
    is_mine: bool,
    revealed: bool,
    flagged: bool,
    adjacent: u8,
}

pub struct MinesweeperGame {
    grid: [[Cell; 10]; 10],
    first_click: bool,
    game_over: bool,
    won: bool,
    flags_left: i32,
    score: u32,
}

impl MinesweeperGame {
    pub fn new() -> Self {
        Self {
            grid: [[Cell { is_mine: false, revealed: false, flagged: false, adjacent: 0 }; 10]; 10],
            first_click: true,
            game_over: false,
            won: false,
            flags_left: 15,
            score: 0,
        }
    }

    fn init_mines(&mut self, safe_r: usize, safe_c: usize) {
        let mut mines_placed = 0;
        while mines_placed < 15 {
            let r = rand::gen_range(0, 10);
            let c = rand::gen_range(0, 10);
            if (r == safe_r && c == safe_c) || self.grid[r][c].is_mine {
                continue;
            }
            self.grid[r][c].is_mine = true;
            mines_placed += 1;
        }

        // Calcula adjacentes
        for r in 0..10 {
            for c in 0..10 {
                if !self.grid[r][c].is_mine {
                    let mut count = 0;
                    for dr in -1..=1 {
                        for dc in -1..=1 {
                            let nr = r as i32 + dr;
                            let nc = c as i32 + dc;
                            if nr >= 0 && nr < 10 && nc >= 0 && nc < 10 {
                                if self.grid[nr as usize][nc as usize].is_mine {
                                    count += 1;
                                }
                            }
                        }
                    }
                    self.grid[r][c].adjacent = count;
                }
            }
        }
    }

    fn reveal(&mut self, r: usize, c: usize) {
        if self.grid[r][c].revealed || self.grid[r][c].flagged {
            return;
        }

        self.grid[r][c].revealed = true;
        self.score += 10;

        if self.grid[r][c].is_mine {
            self.game_over = true;
            return;
        }

        // Cascata se for 0
        if self.grid[r][c].adjacent == 0 {
            for dr in -1..=1 {
                for dc in -1..=1 {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < 10 && nc >= 0 && nc < 10 {
                        self.reveal(nr as usize, nc as usize);
                    }
                }
            }
        }

        self.check_win();
    }

    fn check_win(&mut self) {
        let mut unrevealed_safe = 0;
        for r in 0..10 {
            for c in 0..10 {
                if !self.grid[r][c].is_mine && !self.grid[r][c].revealed {
                    unrevealed_safe += 1;
                }
            }
        }
        if unrevealed_safe == 0 {
            self.won = true;
            self.game_over = true;
            self.score += 500;
        }
    }
}

impl ArcadeGame for MinesweeperGame {
    fn update(&mut self) {
        if self.game_over {
            return;
        }

        let mpos = mouse_position();
        let screen_w = screen_width();
        let screen_h = screen_height();
        let board_ox = (screen_w - 440.0) * 0.5;
        let board_oy = (screen_h - 440.0) * 0.5 + 30.0;
        let cell_size = 40.0;
        let gap = 4.0;

        let left_click = is_mouse_button_pressed(MouseButton::Left);
        let right_click = is_mouse_button_pressed(MouseButton::Right);

        if left_click || right_click {
            for r in 0..10 {
                for c in 0..10 {
                    let cx = board_ox + (c as f32) * (cell_size + gap);
                    let cy = board_oy + (r as f32) * (cell_size + gap);

                    if mpos.0 >= cx && mpos.0 <= cx + cell_size && mpos.1 >= cy && mpos.1 <= cy + cell_size {
                        if left_click {
                            if self.first_click {
                                self.init_mines(r, c);
                                self.first_click = false;
                            }
                            self.reveal(r, c);
                        } else if right_click && !self.grid[r][c].revealed {
                            self.grid[r][c].flagged = !self.grid[r][c].flagged;
                            self.flags_left += if self.grid[r][c].flagged { -1 } else { 1 };
                        }
                        return;
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

        let board_ox = (screen_w - 440.0) * 0.5;
        let board_oy = (screen_h - 440.0) * 0.5 + 30.0;
        let cell_size = 40.0;
        let gap = 4.0;

        draw_card_rect(board_ox - 10.0, board_oy - 10.0, 460.0, 460.0, SURFACE_COLOR, BORDER_COLOR);

        for r in 0..10 {
            for c in 0..10 {
                let cell = &self.grid[r][c];
                let cx = board_ox + (c as f32) * (cell_size + gap);
                let cy = board_oy + (r as f32) * (cell_size + gap);

                if cell.revealed {
                    if cell.is_mine {
                        draw_card_rect(cx, cy, cell_size, cell_size, CORAL, TEXT_PRIMARY);
                        draw_circle(cx + cell_size * 0.5, cy + cell_size * 0.5, 10.0, BG_COLOR);
                    } else {
                        draw_card_rect(cx, cy, cell_size, cell_size, Color::new(0.08, 0.12, 0.20, 1.0), BORDER_COLOR);
                        if cell.adjacent > 0 {
                            let text = format!("{}", cell.adjacent);
                            let col = match cell.adjacent {
                                1 => ICE_BLUE,
                                2 => EMERALD,
                                3 => YELLOW_ACCENT,
                                _ => CORAL,
                            };
                            let dim = measure_text(&text, None, 22, 1.0);
                            draw_text(&text, cx + (cell_size - dim.width) * 0.5, cy + (cell_size + dim.height) * 0.5 - 2.0, 22.0, col);
                        }
                    }
                } else {
                    let bg = if cell.flagged { Color::new(0.2, 0.1, 0.15, 1.0) } else { SURFACE_HOVER };
                    let border = if cell.flagged { CORAL } else { BORDER_COLOR };
                    draw_card_rect(cx, cy, cell_size, cell_size, bg, border);

                    if cell.flagged {
                        // Bandeira estilizada
                        draw_triangle(
                            Vec2::new(cx + 14.0, cy + 10.0),
                            Vec2::new(cx + 28.0, cy + 16.0),
                            Vec2::new(cx + 14.0, cy + 22.0),
                            CORAL,
                        );
                        draw_line(cx + 14.0, cy + 10.0, cx + 14.0, cy + 32.0, 2.0, TEXT_PRIMARY);
                    }
                }
            }
        }

        // HUD
        draw_text(&format!("BANDEIRAS: {}", self.flags_left), ox + 40.0, oy + 40.0, 20.0, CORAL);
        draw_text(&format!("PONTOS: {}", self.score), ox + 650.0, oy + 40.0, 20.0, YELLOW_ACCENT);

        if self.game_over {
            let msg = if self.won { "CAMPO DESMINADO COM SUCESSO!" } else { "FIM DE JOGO - MINA ATIVADA!" };
            let col = if self.won { EMERALD } else { CORAL };
            let dim = measure_text(msg, None, 24, 1.0);
            draw_text(msg, ox + (800.0 - dim.width) * 0.5, oy + 540.0, 24.0, col);
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
        "Minesweeper Grid"
    }
}
