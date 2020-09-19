extern crate chess;

use chess::*;

fn main() {
    let mut game = Game::new();

    let mut buffer = String::new();

    let stdin = std::io::stdin();

    loop {
        buffer.clear();
        game.print_ascii();
        println!("Move in pgn format: ");
        stdin.read_line(&mut buffer).unwrap();

        let r#move = game.parse_pgn_move(&buffer);
        let (from, r#move) = r#move;

        // Clear the screen
        print!("\x1B[2J\x1B[1;1H");

        let result = game.play(from, r#move);
        match result {
            Result::InvalidMove => {
                println!("Invalid move: {}", buffer);
                continue;
            }
            Result::Checkmate => {
                println!("{:?} lost :(", game.current_color());
                break;
            }
            Result::Stalemate => {
                println!("Stalemate!");
                break;
            }
            _ => {}
        }
    }
}
