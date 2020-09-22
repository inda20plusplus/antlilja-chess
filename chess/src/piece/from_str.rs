use super::{Color, PieceType, TaggedPiece};

pub use std::str::FromStr;

pub enum ParseTaggedPieceError {
    InvalidSize,
    InvalidType,
}

impl FromStr for TaggedPiece {
    type Err = ParseTaggedPieceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 2 || s.len() == 0 {
            return Err(ParseTaggedPieceError::InvalidSize);
        }

        if s == "." {
            return Ok(TaggedPiece::empty());
        }

        let mut chars = s.chars();

        let first = chars.next().unwrap();

        let color = if first.is_uppercase() {
            Color::White
        } else {
            Color::Black
        };

        if let Some(r#type) = PieceType::from_char(first.to_ascii_uppercase()) {
            if let Some(second) = chars.next() {
                if second == '*' {
                    Ok(TaggedPiece::original(r#type, color))
                } else {
                    Err(ParseTaggedPieceError::InvalidType)
                }
            } else {
                Ok(TaggedPiece::new(r#type, color))
            }
        } else {
            Err(ParseTaggedPieceError::InvalidType)
        }
    }
}
