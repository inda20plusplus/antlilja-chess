use crate::Color;

pub mod debug;

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

impl PieceType {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'P' => Some(PieceType::Pawn),
            'R' => Some(PieceType::Rook),
            'N' => Some(PieceType::Knight),
            'B' => Some(PieceType::Bishop),
            'Q' => Some(PieceType::Queen),
            'K' => Some(PieceType::King),
            _ => None,
        }
    }
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
