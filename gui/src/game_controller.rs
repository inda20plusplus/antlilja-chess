use chess::game::{Game, GameResult};
use chess::{Pos, Move};
use piston_window::GenericEvent;

pub struct GameController {
    pub game: Game,
    pub selected_square: Option<Pos>,
    pub current_moves: Option<Vec<Move>>,
    cursor_pos: [f64; 2],
}

impl GameController {
    pub fn new(game: Game) -> GameController {
        GameController {
            game,
            selected_square: None,
            current_moves: None,
            cursor_pos: [0.0; 2],
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {

    }
}
