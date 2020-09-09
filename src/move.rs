use crate::piece::PieceType;
use crate::pos::Pos;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Move {
    Move(PieceType, Pos),
    KingSideCastling,
    QueenSideCastling,
}
