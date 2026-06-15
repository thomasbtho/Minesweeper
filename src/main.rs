mod board;
mod cell;
mod game;
mod input;
mod render;

fn main() {
    let mut game = game::Game::new();
    game.start();
}
