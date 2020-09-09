#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MoveType {
    None,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,

    KingSideCastling,
    QueenSideCastling,
}

#[derive(Copy, Clone)]
pub struct Move {
    pos: u8,
    r#type: MoveType,
}

impl Move {
    pub fn new(x: u8, y: u8, r#type: MoveType) -> Self {
        assert!(x < 8 && y < 8);
        return Move {
            pos: x | (y << 4),
            r#type: r#type,
        };
    }

    pub fn from_tuple(t: (u8, u8, MoveType)) -> Self {
        return Self::new(t.0, t.1, t.2);
    }

    pub fn to_xy(&self) -> (u8, u8) {
        return (self.pos & 15, self.pos >> 4);
    }

    pub fn is_type(&self) -> bool {
        return (self.r#type as u8) < (MoveType::KingSideCastling as u8);
    }

    pub fn is_castling(&self) -> bool {
        return (self.r#type as u8) > (MoveType::King as u8);
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "[{:?}: {:?}]", self.r#type, self.to_xy());
    }
}

const MAX_SINGE_PIECE_MOVES: usize = 21;
pub struct Moves(u8, [Move; MAX_SINGE_PIECE_MOVES]);

impl Moves {
    pub fn empty() -> Self {
        return Moves {
            0: 0,
            1: [Move::new(0, 0, MoveType::None); MAX_SINGE_PIECE_MOVES],
        };
    }

    pub fn add(&mut self, x: u8, y: u8, r#type: MoveType) {
        self.1[self.0 as usize] = Move::new(x, y, r#type);
        self.0 += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_coords_and_back() {
        for y in 0..8 {
            for x in 0..8 {
                let m = Move::new(x, y, MoveType::None);
                let pos = m.to_xy();
                assert_eq!(x, pos.0);
                assert_eq!(y, pos.1);
            }
        }
    }
}
