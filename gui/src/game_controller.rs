use chess::Pos;
use chess::Move;
use chess::game::{Game, GameResult};
use std::collections::HashMap;
use piston_window::{GenericEvent, Button, MouseButton};

pub struct GameController {
    pub game: Game,
    pub selected_square: Option<[usize; 2]>,
    pub current_moves: Option<HashMap<[usize; 2], Move>>,
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

    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
          self.cursor_pos = pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            let x = self.cursor_pos[0] - pos[0] - 2.0;
            let y = self.cursor_pos[1] - pos[1] - 2.0;

            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                let cell_x = (x / size * 8.0) as usize;
                let cell_y = 7 - (y / size * 8.0) as usize;

                self.selected_square = Some([cell_x, cell_y]);

                self.current_moves = match self.game.moves_for_pos(Pos::new_xy(cell_x as u8, cell_y as u8)) {
                    Some(move_slice) => {
                        let mut moves = HashMap::new();
                        for r#move in move_slice.iter() {
                            match r#move {
                                Move::Move(pos) => {
                                    moves.insert(
                                        [pos.x() as usize, pos.y() as usize],
                                        *r#move
                                    );
                                }
                                Move::EnPassant(pos) => {
                                    moves.insert(
                                        [pos.x() as usize, pos.y() as usize],
                                        *r#move
                                    );
                                }
                                Move::PawnPromotion(_, pos) => {
                                    moves.insert(
                                        [pos.x() as usize, pos.y() as usize],
                                        *r#move
                                    );
                                }
                                Move::KingSideCastling => {
                                    moves.insert(
                                        [cell_x + 2, cell_y],
                                        *r#move
                                    );
                                }
                                Move::QueenSideCastling => {
                                    moves.insert(
                                        [cell_x - 3, cell_y],
                                        *r#move
                                    );
                                }
                                _ => (),
                            }
                        }
                        Some(moves)
                    }
                    None => None
                };

            }
        }
    }
}
