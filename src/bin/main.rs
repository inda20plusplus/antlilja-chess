extern crate chess;

use chess::*;

fn main() {
    let board = Board::new();
    let mut moves = Vec::<Move>::with_capacity(5);

    board.get_moves_for(&mut moves, 0, 6);

    println!("{:?}", moves);
}
