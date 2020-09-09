use crate::piece::{Color, PieceType, TaggedPiece};

#[allow(dead_code)]
pub struct Board([TaggedPiece; 64]);

#[allow(dead_code)]
impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            0: [TaggedPiece::empty(); 64],
        };

        for i in 0..8 {
            board.0[8 + i] = TaggedPiece::new(PieceType::Pawn, Color::White);
            board.0[8 * 6 + i] = TaggedPiece::new(PieceType::Pawn, Color::Black);
        }

        board.place_matching_at(0, PieceType::Rook);
        board.place_matching_at(1, PieceType::Knight);
        board.place_matching_at(2, PieceType::Bishop);
        board.place_at(3, PieceType::Queen);
        board.place_at(4, PieceType::King);

        return board;
    }

    pub fn print(&self) {
        for i in 0..8 {
            let start = i * 8;
            let end = start + 8;
            println!("{:?}", &self.0[start..end]);
        }
    }

    fn place_at(&mut self, offset: usize, r#type: PieceType) {
        self.0[offset] = TaggedPiece::new(r#type, Color::White);
        self.0[8 * 7 + offset] = TaggedPiece::new(r#type, Color::White);
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
        assert_eq!(board.0[offset], TaggedPiece::new(r#type, Color::White));
        assert_eq!(
            board.0[8 * 7 + offset],
            TaggedPiece::new(r#type, Color::White)
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
                TaggedPiece::new(PieceType::Pawn, Color::White)
            );
            assert_eq!(
                board.0[8 * 6 + i],
                TaggedPiece::new(PieceType::Pawn, Color::Black)
            );
        }

        for i in 8 * 2..8 * 6 {
            assert_eq!(board.0[i], TaggedPiece::empty());
        }

        exists_matching_at(&board, 0, PieceType::Rook);
        exists_matching_at(&board, 1, PieceType::Knight);
        exists_matching_at(&board, 2, PieceType::Bishop);
        exists_at(&board, 3, PieceType::Queen);
        exists_at(&board, 4, PieceType::King);
    }
}
