use chess::game::{Game, GameResult};
use chess::{Pos, Move};
use piston_window::GenericEvent;

pub struct GameController {
    pub game: Game,
    pub selected_square: Option<Pos>,
    pub current_moves: Option<Vec<Move>>,
}

impl GameController {
    pub fn new(game: Game) -> GameController {
        GameController {
            game,
            selected_square: None,
            current_moves: None,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {

    }
}
