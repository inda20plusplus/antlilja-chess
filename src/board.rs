use crate::piece::{Color, ColoredPiece, PieceType};
use crate::r#move::{Move, MoveType};

#[allow(dead_code)]
pub struct Board([ColoredPiece; 64]);

#[allow(dead_code)]
impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            0: [ColoredPiece::empty(); 64],
        };

        for i in 0..8 {
            board.0[8 + i] = ColoredPiece::new(PieceType::Pawn, Color::White);
            board.0[8 * 6 + i] = ColoredPiece::new(PieceType::Pawn, Color::Black);
        }

        board.place_matching_at(0, PieceType::Rook);
        board.place_matching_at(1, PieceType::Knight);
        board.place_matching_at(2, PieceType::Bishop);
        board.place_at(3, PieceType::Queen);
        board.place_at(4, PieceType::King);

        return board;
    }

    pub fn at(&self, x: u8, y: u8) -> ColoredPiece {
        return self.0[(y * 8 + x) as usize];
    }

    pub fn get_moves_for(&self, buffer: &mut Vec<Move>, x: u8, y: u8) -> usize {
        let piece = self.at(x, y);

        match piece.get_type() {
            PieceType::Pawn => {
                return self.add_pawn_moves(buffer, piece.get_color(), x, y);
            }
            _ => {
                return 0;
            }
        }
    }

    fn add_pawn_moves(&self, buffer: &mut Vec<Move>, color: Color, x: u8, y: u8) -> usize {
        if y == 0 || y == 7 {
            return 0;
        }

        let mut count: usize = 0;

        let mut add_move = |r#move: Move| {
            buffer.push(r#move);
            count += 1;
        };

        let dir: i8 = if color == Color::White { 1 } else { -1 };

        let y_forward = (y as i8 + dir) as u8;
        if self.at(x, y_forward).is_empty() {
            add_move(Move::new(x, y_forward, MoveType::Pawn));

            let y_off = y as i8 + dir * 2;
            if y_off >= 0 && y_off <= 7 {
                let y_off = y_off as u8;

                if y == 1 && self.at(x, y_off).is_empty() {
                    add_move(Move::new(x, y_off, MoveType::Pawn));
                }
            }
        }

        let mut add_pawn_take = |x: u8, y: u8| {
            let space = self.at(x, y);
            if !space.is_empty() && space.get_color() != color {
                add_move(Move::new(x, y, MoveType::Pawn));
            }
        };

        if x != 7 {
            add_pawn_take(x + 1, y_forward);
        }

        if x != 0 {
            add_pawn_take(x - 1, y_forward);
        }

        return count;
    }

    pub fn print(&self) {
        for i in 0..8 {
            let start = i * 8;
            let end = start + 8;
            println!("{:?}", &self.0[start..end]);
        }
    }

    fn place_at(&mut self, offset: usize, r#type: PieceType) {
        self.0[offset] = ColoredPiece::new(r#type, Color::White);
        self.0[8 * 7 + offset] = ColoredPiece::new(r#type, Color::White);
    }

    fn place_matching_at(&mut self, offset: usize, r#type: PieceType) {
        self.place_at(offset, r#type);
        self.place_at(7 - offset, r#type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exists_at(board: &Board, offset: usize, r#type: PieceType) {
        assert_eq!(board.0[offset], ColoredPiece::new(r#type, Color::White));
        assert_eq!(
            board.0[8 * 7 + offset],
            ColoredPiece::new(r#type, Color::White)
        );
    }

    fn exists_matching_at(board: &Board, offset: usize, piece: PieceType) {
        exists_at(board, offset, piece);
        exists_at(board, 7 - offset, piece);
    }
    #[test]
    fn new_board() {
        let board = Board::new();

        for i in 0..8 {
            assert_eq!(
                board.0[8 + i],
                ColoredPiece::new(PieceType::Pawn, Color::White)
            );
            assert_eq!(
                board.0[8 * 6 + i],
                ColoredPiece::new(PieceType::Pawn, Color::Black)
            );
        }

        for i in 8 * 2..8 * 6 {
            assert_eq!(board.0[i], ColoredPiece::empty());
        }

        exists_matching_at(&board, 0, PieceType::Rook);
        exists_matching_at(&board, 1, PieceType::Knight);
        exists_matching_at(&board, 2, PieceType::Bishop);
        exists_at(&board, 3, PieceType::Queen);
        exists_at(&board, 4, PieceType::King);
    }
}
