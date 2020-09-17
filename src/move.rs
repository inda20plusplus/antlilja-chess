use crate::{PieceType, Pos};

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Move {
    None,
    Move(Pos),
    EnPassant(Pos),
    PawnPromotion(PieceType, Pos),
    KingSideCastling,
    QueenSideCastling,
}

impl Move {
    pub fn move_xy(x: u8, y: u8) -> Self {
        return Move::Move(Pos::from_xy(x, y));
    }
}
