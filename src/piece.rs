#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum PieceType {
    None = 0,
    Pawn = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Queen = 5,
    King = 6,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 128,
}

impl Color {
    pub fn flip(&mut self) {
        unsafe {
            *self = std::mem::transmute((*self as u8) ^ 128);
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

    pub fn get_color(&self) -> Color {
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
        let mut c = match self.get_type() {
            PieceType::Pawn => 'P',
            PieceType::Rook => 'R',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
            _ => '.',
        };

        if self.get_color() == Color::Black {
            c = c.to_ascii_lowercase();
        }

        return write!(f, "{}", c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let empty = TaggedPiece::empty();
        assert_eq!(empty.0, 0);
    }

    #[test]
    fn get_type() {
        fn piece_type_persists(r#type: PieceType) {
            assert_eq!(
                TaggedPiece::original(r#type, Color::White).get_type(),
                r#type
            );
            assert_eq!(
                TaggedPiece::original(r#type, Color::Black).get_type(),
                r#type
            );
        }

        piece_type_persists(PieceType::Pawn);
        piece_type_persists(PieceType::Rook);
        piece_type_persists(PieceType::Knight);
        piece_type_persists(PieceType::Bishop);
        piece_type_persists(PieceType::Queen);
        piece_type_persists(PieceType::King);
    }

    #[test]
    fn get_color() {
        fn color_persists(r#type: PieceType) {
            assert_eq!(
                TaggedPiece::original(r#type, Color::White).get_color(),
                Color::White
            );
            assert_eq!(
                TaggedPiece::original(r#type, Color::Black).get_color(),
                Color::Black
            );
        }

        color_persists(PieceType::Pawn);
        color_persists(PieceType::Rook);
        color_persists(PieceType::Knight);
        color_persists(PieceType::Bishop);
        color_persists(PieceType::Queen);
        color_persists(PieceType::King);
    }
}
