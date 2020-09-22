use super::{Color, PieceType, TaggedPiece};

impl std::fmt::Display for TaggedPiece {
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

        write!(
            f,
            "{}",
            if self.color() == Color::White {
                c
            } else {
                c.to_ascii_lowercase()
            }
        )
    }
}
