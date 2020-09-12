use crate::{PieceType, Pos};

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Move {
    None,
    Move(Pos),
    PawnPromotion(PieceType, Pos),
    KingSideCastling,
    QueenSideCastling,
}
