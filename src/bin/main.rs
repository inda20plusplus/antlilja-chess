extern crate chess;

use chess::*;

fn main() {
    let game = Game::new();

    game.print_ascii();
}
