pub mod space_odyssey;
pub mod snake;
pub mod pong;
pub mod breakout;
pub mod flappy;
pub mod tron;
pub mod cannon;
pub mod tiles;
pub mod minesweeper;
pub mod connect4;
pub mod pacman;

pub trait ArcadeGame {
    fn update(&mut self);
    fn draw(&self, screen_w: f32, screen_h: f32);
    fn reset(&mut self);
    fn get_score(&self) -> u32;
    fn is_game_over(&self) -> bool;
    fn get_title(&self) -> &str;
}

pub fn create_game(id: &str) -> Option<Box<dyn ArcadeGame>> {
    match id {
        "space_odyssey" => Some(Box::new(space_odyssey::SpaceOdyssey::new())),
        "snake" => Some(Box::new(snake::SnakeGame::new())),
        "pong" => Some(Box::new(pong::PongGame::new())),
        "breakout" => Some(Box::new(breakout::BreakoutGame::new())),
        "flappy" => Some(Box::new(flappy::FlappyGame::new())),
        "tron" => Some(Box::new(tron::TronGame::new())),
        "cannon" => Some(Box::new(cannon::CannonGame::new())),
        "tiles" => Some(Box::new(tiles::TilesGame::new())),
        "minesweeper" => Some(Box::new(minesweeper::MinesweeperGame::new())),
        "connect4" => Some(Box::new(connect4::Connect4Game::new())),
        "pacman" => Some(Box::new(pacman::PacmanGame::new())),
        _ => None,
    }
}
