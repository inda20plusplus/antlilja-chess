#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Piece {
    None = 0,

    WhitePawn = 1,
    WhiteRook = 2,
    WhiteKnight = 3,
    WhiteBishop = 4,
    WhiteQueen = 5,
    WhiteKing = 6,

    BlackPawn = 129,
    BlackRook = 130,
    BlackKnight = 131,
    BlackBishop = 132,
    BlackQueen = 133,
    BlackKing = 134,
}

impl Piece {
    pub fn opposite(&self) -> Self {
        if *self == Piece::None {
            return Piece::None;
        }

        unsafe {
            return std::mem::transmute(*self as u8 ^ 128);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposite_piece_black_to_white() {
        let piece = Piece::BlackBishop;
        assert_eq!(piece.opposite(), Piece::WhiteBishop);
    }

    #[test]
    fn opposite_piece_white_to_black() {
        let piece = Piece::WhiteBishop;
        assert_eq!(piece.opposite(), Piece::BlackBishop);
    }

    #[test]
    fn opposite_piece_none() {
        let piece = Piece::None;
        assert_eq!(piece.opposite(), Piece::None);
    }
}
