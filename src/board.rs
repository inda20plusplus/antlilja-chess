use crate::piece::Piece;

#[allow(dead_code)]
pub struct Board([Piece; 64]);

#[allow(dead_code)]
impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            0: [Piece::None; 64],
        };

        for i in 0..8 {
            board.0[8 + i] = Piece::WhitePawn;
            board.0[8 * 6 + i] = Piece::BlackPawn;
        }

        board.place_matching_at(0, Piece::WhiteRook);
        board.place_matching_at(1, Piece::WhiteKnight);
        board.place_matching_at(2, Piece::WhiteBishop);
        board.place_at(3, Piece::WhiteQueen);
        board.place_at(4, Piece::WhiteKing);

        return board;
    }

    pub fn print(&self) {
        for i in 0..8 {
            let start = i * 8;
            let end = start + 8;
            println!("{:?}", &self.0[start..end]);
        }
    }

    fn place_at(&mut self, offset: usize, piece: Piece) {
        let opposite = piece.opposite();
        self.0[offset] = piece;
        self.0[8 * 7 + offset] = opposite;
    }

    fn place_matching_at(&mut self, offset: usize, piece: Piece) {
        self.place_at(offset, piece);
        self.place_at(7 - offset, piece);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exists_at(board: &Board, offset: usize, piece: Piece) {
        let opposite = piece.opposite();
        assert_eq!(board.0[offset], piece);
        assert_eq!(board.0[8 * 7 + offset], opposite);
    }

    fn exists_matching_at(board: &Board, offset: usize, piece: Piece) {
        exists_at(board, offset, piece);
        exists_at(board, 7 - offset, piece);
    }
    #[test]
    fn new_board() {
        let board = Board::new();

        for i in 0..8 {
            assert_eq!(board.0[8 + i], Piece::WhitePawn);
            assert_eq!(board.0[8 * 6 + i], Piece::BlackPawn);
        }

        for i in 8 * 2..8 * 6 {
            assert_eq!(board.0[i], Piece::None);
        }

        exists_matching_at(&board, 0, Piece::WhiteRook);
        exists_matching_at(&board, 1, Piece::WhiteKnight);
        exists_matching_at(&board, 2, Piece::WhiteBishop);
        exists_at(&board, 3, Piece::WhiteQueen);
        exists_at(&board, 4, Piece::WhiteKing);
    }
}
