use crate::{Color, Move, PieceType, Pos, TaggedPiece};

pub mod debug;
pub mod from_str;

pub use from_str::*;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone)]
pub struct Board([TaggedPiece; 64]);

impl PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
        for i in 0..64 {
            if self.0[i] != other.0[i] {
                return false;
            }
        }

        true
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Self {
            0: [TaggedPiece::empty(); 64],
        };

        for i in 0..8 {
            board.0[8 + i] = TaggedPiece::original(PieceType::Pawn, Color::White);
            board.0[8 * 6 + i] = TaggedPiece::original(PieceType::Pawn, Color::Black);
        }

        let mut place_at_both_sides = |offset, r#type| {
            board.0[offset] = TaggedPiece::original(r#type, Color::White);
            board.0[8 * 7 + offset] = TaggedPiece::original(r#type, Color::Black);
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

        board
    }
}

impl Board {
    pub fn set_pos(&mut self, pos: Pos, piece: TaggedPiece) {
        self.0[pos.index()] = piece;
    }

    pub fn at_pos(&self, pos: Pos) -> TaggedPiece {
        self.0[pos.index()]
    }

    pub fn at_xy(&self, x: u8, y: u8) -> TaggedPiece {
        self.at_pos(Pos::new_xy(x, y))
    }

    pub fn at_index(&self, i: usize) -> TaggedPiece {
        self.0[i]
    }

    pub fn move_piece(&mut self, from: Pos, to: Pos) -> bool {
        if self.at_pos(from) == TaggedPiece::empty() {
            return false;
        }

        self.set_pos(to, self.at_pos(from).non_original());
        self.set_pos(from, TaggedPiece::empty());
        true
    }

    pub fn find_first_of_type(&self, r#type: PieceType, color: Color) -> Option<Pos> {
        let piece = TaggedPiece::new(r#type, color);
        for (i, p) in self.0.iter().enumerate() {
            if p == &piece {
                return Some(Pos::new_index(i as u8));
            }
        }

        None
    }

    pub fn find_king(&self, color: Color) -> Pos {
        self.find_first_of_type(PieceType::King, color).unwrap()
    }

    pub fn after_move(&self, from: Pos, r#move: Move, color: Color) -> Self {
        let mut board = *self;
        match r#move {
            Move::Move(to) => {
                board.move_piece(from, to);
            }
            Move::KingSideCastling => {
                let y: u8 = if color == Color::White { 0 } else { 7 };

                let rook_pos = Pos::new_xy(7, y);
                board.move_piece(rook_pos, Pos::new_xy(5, y));

                let king_pos = Pos::new_xy(4, y);
                board.move_piece(king_pos, Pos::new_xy(6, y));
            }
            Move::QueenSideCastling => {
                let y: u8 = if color == Color::White { 0 } else { 7 };

                let rook_pos = Pos::new_xy(0, y);
                board.move_piece(rook_pos, Pos::new_xy(3, y));

                let king_pos = Pos::new_xy(4, y);
                board.move_piece(king_pos, Pos::new_xy(2, y));
            }
            Move::PawnPromotion(r#type, to) => {
                board.move_piece(from, to);
                board.set_pos(to, TaggedPiece::new(r#type, color));
            }
            Move::EnPassant(to) => {
                let dir = if color == Color::White { -1 } else { 1 };
                board.move_piece(from, to);

                let (x, y) = to.xy();
                let remove_pos = Pos::new_xy(x, (y as i8 + dir) as u8);
                board.set_pos(remove_pos, TaggedPiece::empty());
            }
            _ => panic!("Unimplemented move {:?}", r#move),
        }

        board
    }

    pub fn pos_in_danger(&self, pos: Pos, color: Color) -> bool {
        let first_enemy_piece = |dir_x: i8, dir_y: i8| {
            let mut i: i8 = 1;
            loop {
                if let Some(new_pos) = pos.move_xy(dir_x * i, dir_y * i) {
                    let piece = self.at_pos(new_pos);

                    if !piece.is_empty() {
                        if piece.color() == color {
                            return None;
                        } else {
                            return Some(new_pos);
                        }
                    }
                } else {
                    return None;
                }

                i += 1;
            }
        };

        let diag = |config: &(i8, i8, Color)| {
            if let Some(hit) = first_enemy_piece(config.0, config.1) {
                let piece = self.at_pos(hit);
                let ptype = piece.get_type();
                if ptype == PieceType::Bishop || ptype == PieceType::Queen {
                    return true;
                }

                let dist_x = hit.distance_x(&pos);
                let dist_y = hit.distance_y(&pos);

                if dist_x == 1
                    && dist_y == 1
                    && (ptype == PieceType::King || (color == config.2 && ptype == PieceType::Pawn))
                {
                    return true;
                }
            }
            false
        };

        let dirs: [(i8, i8, Color); 4] = [
            (1, 1, Color::White),
            (-1, 1, Color::White),
            (1, -1, Color::Black),
            (-1, -1, Color::Black),
        ];

        for d in dirs.iter() {
            if diag(d) {
                return true;
            }
        }

        let straight = |dir: &(i8, i8)| {
            if let Some(hit) = first_enemy_piece(dir.0, dir.1) {
                let piece = self.at_pos(hit);
                let ptype = piece.get_type();

                if ptype == PieceType::Rook || ptype == PieceType::Queen {
                    return true;
                }

                let dist_x = hit.distance_x(&pos);
                let dist_y = hit.distance_y(&pos);
                if (dist_x + dist_y == 1) && ptype == PieceType::King {
                    return true;
                }
            }
            false
        };

        let dirs: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        for d in dirs.iter() {
            if straight(d) {
                return true;
            }
        }

        let dirs: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

        for (x, y) in dirs.iter() {
            let dir_move = |off_x, off_y| {
                if let Some(new_pos) = pos.move_xy(off_x, off_y) {
                    let piece = self.at_pos(new_pos);
                    if !piece.is_empty()
                        && piece.color() != color
                        && piece.get_type() == PieceType::Knight
                    {
                        return true;
                    }
                }
                false
            };

            if dir_move(x * 2, *y) {
                return true;
            }

            if dir_move(*x, y * 2) {
                return true;
            }
        }

        false
    }

    pub fn print_ascii(&self, color: Color) {
        let internal_loop = |y| {
            print!("{}| ", y + 1);
            for x in 0..8 {
                print!("{} ", self.at_xy(x, y));
            }
            println!("|");
        };

        if color == Color::White {
            for y in (0..8).rev() {
                internal_loop(y);
            }
        } else {
            for y in 0..8 {
                internal_loop(y);
            }
        }
        println!("   - - - - - - - -");
        println!("   a b c d e f g h");
    }
}
