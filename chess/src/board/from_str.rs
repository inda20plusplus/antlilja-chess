use std::str::FromStr;

use super::{Board, TaggedPiece};

#[derive(Debug)]
pub enum ParseBoardError {
    InvalidSize,
    InvalidPiece,
}

impl FromStr for Board {
    type Err = ParseBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces = s.split_whitespace();

        let mut board = Self {
            0: [TaggedPiece::empty(); 64],
        };

        for (i, p) in pieces.enumerate() {
            if i > 63 {
                return Err(ParseBoardError::InvalidSize);
            }

            if let Ok(piece) = TaggedPiece::from_str(p) {
                board.0[i] = piece;
            } else {
                return Err(ParseBoardError::InvalidPiece);
            }
        }

        Ok(board)
    }
}
