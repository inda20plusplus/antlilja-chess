use crate::{Color, PieceType, Pos, TaggedPiece};

pub struct Board([TaggedPiece; 64]);

impl Board {
    pub fn new() -> Self {
        let mut data = Self {
            0: [TaggedPiece::empty(); 64],
        };

        for i in 0..8 {
            data.0[8 + i] = TaggedPiece::original(PieceType::Pawn, Color::White);
            data.0[8 * 6 + i] = TaggedPiece::original(PieceType::Pawn, Color::Black);
        }

        let mut place_at_both_sides = |offset, r#type| {
            data.0[offset] = TaggedPiece::original(r#type, Color::White);
            data.0[8 * 7 + offset] = TaggedPiece::original(r#type, Color::Black);
        };

        let mut place_matching_at_both_sides = |offset, r#type| {
            place_at_both_sides(offset, r#type);
            place_at_both_sides(7 - offset, r#type);
        };

        place_matching_at_both_sides(0, PieceType::Rook);
        place_matching_at_both_sides(1, PieceType::Knight);
        place_matching_at_both_sides(2, PieceType::Bishop);
        place_at_both_sides(3, PieceType::Queen);
        place_at_both_sides(4, PieceType::King);

        return data;
    }

    pub fn set_pos(&mut self, pos: Pos, piece: TaggedPiece) {
        self.0[pos.as_index()] = piece;
    }

    pub fn at_pos(&self, pos: Pos) -> TaggedPiece {
        return self.0[pos.as_index()];
    }

    pub fn at_xy(&self, x: u8, y: u8) -> TaggedPiece {
        return self.at_pos(Pos::from_xy(x, y));
    }

    pub fn at_index(&self, i: usize) -> TaggedPiece {
        return self.0[i];
    }

    pub fn move_piece(&mut self, from: Pos, to: Pos) -> bool {
        if self.at_pos(from) == TaggedPiece::empty() {
            return false;
        }

        self.set_pos(to, self.at_pos(from));
        return true;
    }

    pub fn print_ascii(&self) {
        for y in (0..8).rev() {
            print!("| ");
            for x in 0..8 {
                print!("{:?} ", self.at_xy(x, y));
            }
            println!("|")
        }
    }
}
