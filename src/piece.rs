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
        assert_eq!(Piece::BlackPawn.opposite(), Piece::WhitePawn);
        assert_eq!(Piece::BlackRook.opposite(), Piece::WhiteRook);
        assert_eq!(Piece::BlackKnight.opposite(), Piece::WhiteKnight);
        assert_eq!(Piece::BlackBishop.opposite(), Piece::WhiteBishop);
        assert_eq!(Piece::BlackQueen.opposite(), Piece::WhiteQueen);
        assert_eq!(Piece::BlackKing.opposite(), Piece::WhiteKing);
    }

    #[test]
    fn opposite_piece_white_to_black() {
        assert_eq!(Piece::WhitePawn.opposite(), Piece::BlackPawn);
        assert_eq!(Piece::WhiteRook.opposite(), Piece::BlackRook);
        assert_eq!(Piece::WhiteKnight.opposite(), Piece::BlackKnight);
        assert_eq!(Piece::WhiteBishop.opposite(), Piece::BlackBishop);
        assert_eq!(Piece::WhiteQueen.opposite(), Piece::BlackQueen);
        assert_eq!(Piece::WhiteKing.opposite(), Piece::BlackKing);
    }

    #[test]
    fn opposite_piece_none() {
        assert_eq!(Piece::None.opposite(), Piece::None);
    }
}
