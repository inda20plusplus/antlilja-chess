#[derive(Copy, Clone)]
pub struct Move(u8);

impl Move {
    pub fn new(x: u8, y: u8) -> Self {
        assert!(x < 8 && y < 8);
        return Move { 0: x | (y << 4) };
    }

    pub fn to_xy(&self) -> (u8, u8) {
        return (self.0 & 15, self.0 >> 4);
    }
}

const MAX_SINGE_PIECE_MOVES: usize = 21;
pub struct Moves(u8, [Move; MAX_SINGE_PIECE_MOVES]);

impl Moves {
    pub fn empty() -> Self {
        return Moves {
            0: 0,
            1: [Move::new(0, 0); MAX_SINGE_PIECE_MOVES],
        };
    }

    pub fn add(&mut self, x: u8, y: u8) {
        self.1[self.0 as usize] = Move::new(x, y);
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
                let m = Move::new(x, y);
                let pos = m.to_xy();
                assert_eq!(x, pos.0);
                assert_eq!(y, pos.1);
            }
        }
    }
}
