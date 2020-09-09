use crate::piece::PieceType;

#[derive(Copy, Clone)]
pub struct Move {
    pos: u8,
    piece_type: PieceType,
}

impl Move {
    pub fn new(x: u8, y: u8, piece_type: PieceType) -> Self {
        assert!(x < 8 && y < 8);
        return Move {
            pos: x | (y << 4),
            piece_type: piece_type,
        };
    }

    pub fn from_tuple(t: (u8, u8, PieceType)) -> Self {
        return Self::new(t.0, t.1, t.2);
    }

    pub fn to_xy(&self) -> (u8, u8) {
        return (self.pos & 15, self.pos >> 4);
    }

    pub fn get_piece_type(&self) -> PieceType {
        return self.piece_type;
    }
}

const MAX_SINGE_PIECE_MOVES: usize = 21;
pub struct Moves(u8, [Move; MAX_SINGE_PIECE_MOVES]);

impl Moves {
    pub fn empty() -> Self {
        return Moves {
            0: 0,
            1: [Move::new(0, 0, PieceType::None); MAX_SINGE_PIECE_MOVES],
        };
    }

    pub fn add(&mut self, x: u8, y: u8, piece: PieceType) {
        self.1[self.0 as usize] = Move::new(x, y, piece);
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
                let m = Move::new(x, y, PieceType::None);
                let pos = m.to_xy();
                assert_eq!(x, pos.0);
                assert_eq!(y, pos.1);
            }
        }
    }
}
