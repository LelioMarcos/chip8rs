use chip8rs::core::*;
use std::env;

fn main() {
    let game: Vec<String> = env::args().collect();
    let game = &game[1];

    let mut c = Chip8::init();

    c.run_game(game);
}
