use crate::Color;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum PieceType {
    Pawn = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Queen = 5,
    King = 6,
}

#[derive(Copy, Clone)]
pub struct TaggedPiece(u8);

impl PartialEq for TaggedPiece {
    fn eq(&self, other: &Self) -> bool {
        return (self.0 & 191) == (other.0 & 191);
    }
}

impl TaggedPiece {
    pub fn empty() -> Self {
        return TaggedPiece { 0: 0 };
    }

    pub fn new(r#type: PieceType, color: Color) -> Self {
        return TaggedPiece {
            0: (r#type as u8) ^ (color as u8),
        };
    }

    pub fn original(r#type: PieceType, color: Color) -> Self {
        return TaggedPiece {
            0: ((r#type as u8) ^ (color as u8)) ^ 64,
        };
    }

    pub fn non_original(&self) -> Self {
        return TaggedPiece { 0: self.0 & 191 };
    }

    pub fn is_empty(&self) -> bool {
        return self.0 == 0;
    }

    pub fn is_original(&self) -> bool {
        return (self.0 & 64) == 64;
    }

    pub fn get_type(&self) -> PieceType {
        unsafe {
            return std::mem::transmute(self.0 & 63);
        }
    }

    pub fn color(&self) -> Color {
        unsafe {
            return std::mem::transmute(self.0 & 128);
        }
    }

    pub fn opposite_color(&self) -> Self {
        return TaggedPiece { 0: self.0 ^ 128 };
    }
}

impl std::fmt::Debug for TaggedPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut c = if self.is_empty() {
            '.'
        } else {
            match self.get_type() {
                PieceType::Pawn => 'P',
                PieceType::Rook => 'R',
                PieceType::Knight => 'N',
                PieceType::Bishop => 'B',
                PieceType::Queen => 'Q',
                PieceType::King => 'K',
            }
        };

        if self.color() == Color::Black {
            c = c.to_ascii_lowercase();
        }

        return write!(f, "{}", c);
    }
}
