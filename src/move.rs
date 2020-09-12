use crate::piece::PieceType;
use crate::pos::Pos;

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum Move {
    None,
    Move(Pos, Pos),
    PawnPromotion(PieceType, Pos, Pos),
    KingSideCastling,
    QueenSideCastling,
}
