extern crate chess;

use chess::board::Board;

fn main() {
    let board = Board::new();
    board.print();
}
