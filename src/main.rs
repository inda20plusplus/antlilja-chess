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
        buffer.clear();
        game.print_ascii();
        println!("Make your move: ");
        std::io::stdin().read_line(&mut buffer).unwrap();
        let r#move = parse_move(&buffer);

        if r#move.is_none() {
            println!("Not a valid move: {}", buffer);
            continue;
        }

        let r#move = r#move.unwrap();
        let from = r#move.0;
        let to = r#move.1;

        let moves = game.get_moves_for(from.0, from.1);

        if moves.is_none() {
            println!("Not a valid move: {}", buffer);
            continue;
        }

        let r#move = Move::move_xy(to.0, to.1);

        let moves = moves.unwrap();

        let move_index = moves.find(r#move);

        if move_index.is_none() {
            println!("Not a valid move: {}", buffer);
            continue;
        }

        let move_index = move_index.unwrap();

        if game.play(from.0, from.1, move_index) == Result::Checkmate {
            println!("{:?} lost :(", game.current_color());
            break;
        }
    }
}
