use crate::{Board, Color, Move, MoveArray, MoveMap, PieceType, Pos, TaggedPiece};

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
    king_pos: (u8, u8),
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            board: Board::new(),
            move_map: MoveMap::new(),
            history: Vec::<(Pos, Move)>::with_capacity(50),
            color: Color::White,
            king_pos: (4, 0),
        };

        game.calculate_all_moves();

        return game;
    }

    pub fn switch_side(&mut self) -> bool {
        self.move_map.clear();
        self.color.flip();
        self.king_pos = self.board.find_king(self.color).to_xy();
        return self.calculate_all_moves();
    }

    pub fn current_color(&self) -> Color {
        return self.color;
    }

    fn king_in_danger_after_move(&self, from: Pos, r#move: Move) -> bool {
        let board_after_move = self.board.board_after_move(from, r#move, self.color);
        return board_after_move.pos_in_danger(self.king_pos.0, self.king_pos.1, self.color);
    }

    pub fn play(&mut self, x: u8, y: u8, r#move: Move) -> Result {
        let pos = Pos::from_xy(x, y);

        assert!(pos.as_index() < 64);
        assert!(!self.at_pos(pos).is_empty());

        let moves = self.move_map.at(pos);

        if !moves.exists(r#move) {
            return Result::InvalidMove;
        }

        self.board = self.board.board_after_move(pos, r#move, self.color);

        self.history.push((pos, r#move));
        if self.switch_side() {
            let (x, y) = self.king_pos;
            if self.board.pos_in_danger(x, y, self.color) {
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
        for y in 0..8 {
            for x in 0..8 {
                moves += self.calculate_moves_for(x, y);
                }
            }

        return moves == 0;
    }

    fn calculate_moves_for(&mut self, x: u8, y: u8) -> usize {
        let pos = Pos::from_xy(x, y);
        let piece = self.at_pos(pos);

        if piece.is_empty() || piece.get_color() != self.color {
            return 0;
        }

        let mut moves = MoveArray::empty();

        match piece.get_type() {
            PieceType::Pawn => {
                self.add_pawn_moves(&mut moves, x, y);
            }
            PieceType::Rook => {
                self.add_straight_moves(&mut moves, x, y);
            }
            PieceType::Knight => {
                self.add_knight_moves(&mut moves, x, y);
            }
            PieceType::Bishop => {
                self.add_diagonal_moves(&mut moves, x, y);
            }
            PieceType::Queen => {
                self.add_diagonal_moves(&mut moves, x, y);
                self.add_straight_moves(&mut moves, x, y);
            }
            PieceType::King => {
                self.add_king_moves(&mut moves, x, y);
                self.add_castling_moves(&mut moves);
            }
            _ => {
                return 0;
            }
        };

        self.move_map.insert(pos, moves);
        return moves.size();
    }

    pub fn get_moves_for(&self, x: u8, y: u8) -> Option<&MoveArray> {
        let pos = Pos::from_xy(x, y);

        if self.at_pos(pos).is_empty() {
            return None;
        }

        return Some(&self.move_map.at(pos));
    }

    pub fn print_ascii(&self) {
        self.board.print_ascii(self.color);
    }
}
