use chess::game::Game;
use chess::Move;
use chess::Pos;
use crate::view::ViewSettings;
use piston_window::{Button, GenericEvent, MouseButton};
use std::collections::HashMap;

pub enum Ending {
    White,
    Black,
    Tie,
}

impl ToString for Ending {
    fn to_string(&self) -> String {
        match self {
            Ending::White => "White Wins!".to_string(),
            Ending::Black => "Black Wins!".to_string(),
            Ending::Tie => "It's a Tie!".to_string(),
        }

    }
}

pub enum State {
    Playing,
    Promotion(Pos),
    End(Ending),
}

pub struct GameController {
    pub game: Game,
    pub state: State,
    settings: ViewSettings,
    pub selected_square: Option<[usize; 2]>,
    pub current_moves: Option<HashMap<[usize; 2], Move>>,
    cursor_pos: [f64; 2],
}

impl GameController {
    pub fn new(game: Game, settings: ViewSettings) -> GameController {
        GameController {
            game,
            state: State::Playing,
            settings,
            selected_square: None,
            current_moves: None,
            cursor_pos: [0.0; 2],
        }
    }

    pub fn event<E: GenericEvent>(&mut self, board_pos: [f64; 2], board_size: f64, e: &E) {
        if let Some(mouse_pos) = e.mouse_cursor_args() {
            self.cursor_pos = mouse_pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            match self.state {
                State::Playing => self.play_input(),
                _ => (),
            }
        }
    }

    fn play_input(&mut self) {
        let board_size = self.settings.board_size;
        let board_pos = self.settings.board_pos();

        let x = self.cursor_pos[0] - board_pos[0] - 2.0;
        let y = self.cursor_pos[1] - board_pos[1] - 2.0;

        if x >= 0.0 && x < board_size && y >= 0.0 && y < board_size {
            let cell_x = (x / board_size * 8.0) as usize;
            let cell_y = 7 - (y / board_size * 8.0) as usize;

            if let Some(moves) = self.current_moves.clone() {
                if let Some(r#move) = moves.get(&[cell_x, cell_y]) {

                    let from = self.selected_square.unwrap();
                    let turn_result = self.game.play_xy(from[0] as u8, from[1] as u8, *r#move);

                    match turn_result {
                        GameResult::Ok => (),
                        GameResult::Checkmate => {
                            let color = match self.game.current_color() {
                                Color::White => Ending::White,
                                Color::Black => Ending::Black,
                            };
                            self.state = State::End(color);
                        }
                        GameResult::Stalemate => self.state = State::End(Ending::Tie),
                        GameResult::InvalidMove => panic!("Move was in current move but game returned InvalidMove")
                    }

                    self.selected_square = None;
                    self.current_moves = None;
                } else {
                    self.selected_square = Some([cell_x, cell_y]);
                    self.get_current_moves();
                }
            } else {
                self.selected_square = Some([cell_x, cell_y]);
                self.get_current_moves();
            }
        }
    }

    fn get_current_moves(&mut self) {
        if let Some(position) = self.selected_square {
            let current_pos = Pos::new_xy(position[0] as u8, position[1] as u8);
            self.current_moves = match self.game.moves_for_pos(current_pos) {
                Some(move_slice) => {
                    let mut moves = HashMap::new();
                    println!("debug:");
                    for r#move in move_slice.iter() {
                        println!("{:?}", r#move);
                        match r#move {
                            Move::Move(pos) => {
                                moves.insert([pos.x() as usize, pos.y() as usize], *r#move);
                            }
                            Move::EnPassant(pos) => {
                                moves.insert([pos.x() as usize, pos.y() as usize], *r#move);
                            }
                            Move::PawnPromotion(_, pos) => {
                                moves.entry([pos.x() as usize, pos.y() as usize]).or_insert(*r#move);
                            }
                            Move::KingSideCastling => {
                                moves.insert([position[0] + 2, position[1]], *r#move);
                            }
                            Move::QueenSideCastling => {
                                moves.insert([position[0] - 3, position[1]], *r#move);
                            }
                            _ => (),
                        }
                    }
                    Some(moves)
                }
                None => None,
            };
        }
    }
}
