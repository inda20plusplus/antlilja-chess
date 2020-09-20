use super::{Color, PieceType, TaggedPiece};

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

        if self.is_original() {
            write!(f, "{}*", c)
        } else {
            write!(f, "{}", c)
        }
    }
}
