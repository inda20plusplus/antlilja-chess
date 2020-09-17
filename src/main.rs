extern crate chess;

use chess::*;

fn parse_pos(str: &str) -> (u8, u8) {
    let mut chars = str.chars();
    return (
        chars.next().unwrap() as u8 - 97,
        chars.next().unwrap() as u8 - 49,
    );
}

fn parse_move(str: &String) -> Option<((u8, u8), (u8, u8))> {
    let mut positions = str.split_whitespace();

    let from = positions.next();
    let to = positions.next();
    if from.is_none() || to.is_none() || from.unwrap().len() != 2 || to.unwrap().len() != 2 {
        return None;
    }

    return Some((parse_pos(from.unwrap()), parse_pos(to.unwrap())));
}

fn main() {
    let mut game = Game::new();

    let mut buffer = String::new();

    loop {
        print!("\x1B[2J\x1B[1;1H");
        buffer.clear();
        game.print_ascii();
        println!("Make your move: ");
        std::io::stdin().read_line(&mut buffer).unwrap();
        let r#move = parse_move(&buffer);

        if r#move.is_none() {
            continue;
        }

        let ((x, y), (to_x, to_y)) = r#move.unwrap();

        let moves = game.get_moves_for(x, y);

        if moves.is_none() {
            continue;
        }

        let r#move = Move::move_xy(to_x, to_y);

        let result = game.play(x, y, r#move);
        match result {
            Result::InvalidMove => {
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
