use crate::{Board, Color, Move, MoveMap, PieceType, Pos, TaggedPiece};

mod moves;
mod pgn;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum Result {
    InvalidMove,
    Ok,
    Checkmate,
    Stalemate,
}

pub struct Game {
    board: Board,
    move_map: MoveMap,
    history: Vec<(Pos, Move)>,
    color: Color,
    king_pos: Pos,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            board: Board::new(),
            move_map: MoveMap::new(),
            history: Vec::<(Pos, Move)>::with_capacity(50),
            color: Color::White,
            king_pos: Pos::new_xy(4, 0),
        };

        game.calculate_all_moves();

        return game;
    }

    pub fn switch_side(&mut self) -> bool {
        self.move_map.clear();
        self.color.flip();
        self.king_pos = self.board.find_king(self.color);
        return self.calculate_all_moves();
    }

    pub fn current_color(&self) -> Color {
        return self.color;
    }

    fn king_in_danger_after_move(&self, from: Pos, r#move: Move) -> bool {
        let board_after_move = self.board.board_after_move(from, r#move, self.color);
        return board_after_move.pos_in_danger(self.king_pos, self.color);
    }

    pub fn play(&mut self, from: Pos, r#move: Move) -> Result {
        if r#move == Move::None {
            return Result::InvalidMove;
        }

        assert!(from.index() < 64);
        assert!(!self.at_pos(from).is_empty());

        let moves = self.move_map.at(from);

        if moves.is_none() {
            return Result::InvalidMove;
        }

        let moves = moves.unwrap();

        if !moves.contains(&r#move) {
            return Result::InvalidMove;
        }

        self.board = self.board.board_after_move(from, r#move, self.color);

        self.history.push((from, r#move));
        if self.switch_side() {
            if self.board.pos_in_danger(self.king_pos, self.color) {
                return Result::Checkmate;
            } else {
                return Result::Stalemate;
            }
        } else {
            return Result::Ok;
        }
    }

    pub fn at_xy(&self, x: u8, y: u8) -> TaggedPiece {
        return self.board.at_xy(x, y);
    }

    pub fn at_pos(&self, pos: Pos) -> TaggedPiece {
        return self.board.at_pos(pos);
    }

    pub fn at_index(&self, i: usize) -> TaggedPiece {
        return self.board.at_index(i);
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    fn calculate_all_moves(&mut self) -> bool {
        let mut moves: usize = 0;
        for i in 0..64 {
            moves += self.calculate_moves_for(Pos::new_index(i));
        }

        return moves == 0;
    }

    fn calculate_moves_for(&mut self, pos: Pos) -> usize {
        let piece = self.at_pos(pos);

        if piece.is_empty() || piece.get_color() != self.color {
            return 0;
        }

        self.move_map.set_current_pos(pos);

        match piece.get_type() {
            PieceType::Pawn => {
                self.add_pawn_moves(pos);
            }
            PieceType::Rook => {
                self.add_straight_moves(pos);
            }
            PieceType::Knight => {
                self.add_knight_moves(pos);
            }
            PieceType::Bishop => {
                self.add_diagonal_moves(pos);
            }
            PieceType::Queen => {
                self.add_diagonal_moves(pos);
                self.add_straight_moves(pos);
            }
            PieceType::King => {
                self.add_king_moves(pos);
                self.add_castling_moves();
            }
            _ => {
                return 0;
            }
        };

        return self.move_map.current_pos_moves_len();
    }

    pub fn get_moves_for(&self, pos: Pos) -> Option<&[Move]> {
        return self.move_map.at(pos);
    }

    pub fn print_ascii(&self) {
        self.board.print_ascii(self.color);
    }
}
