use crate::{Color, Move, PieceType, Pos, TaggedPiece};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone)]
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
        self.0[pos.index()] = piece;
    }

    pub fn at_pos(&self, pos: Pos) -> TaggedPiece {
        return self.0[pos.index()];
    }

    pub fn at_xy(&self, x: u8, y: u8) -> TaggedPiece {
        return self.at_pos(Pos::new_xy(x, y));
    }

    pub fn at_index(&self, i: usize) -> TaggedPiece {
        return self.0[i];
    }

    pub fn move_piece(&mut self, from: Pos, to: Pos) -> bool {
        if self.at_pos(from) == TaggedPiece::empty() {
            return false;
        }

        self.set_pos(to, self.at_pos(from).non_original());
        self.set_pos(from, TaggedPiece::empty());
        return true;
    }

    pub fn find_first_of_type(&self, r#type: PieceType, color: Color) -> Option<Pos> {
        let piece = TaggedPiece::new(r#type, color);
        for (i, p) in self.0.iter().enumerate() {
            if p == &piece {
                return Some(Pos::new_index(i as u8));
            }
        }

        return None;
    }

    pub fn find_king(&self, color: Color) -> Pos {
        return self.find_first_of_type(PieceType::King, color).unwrap();
    }

    pub fn board_after_move(&self, from: Pos, r#move: Move, color: Color) -> Self {
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

        return board;
    }

    pub fn pos_in_danger(&self, pos: Pos, color: Color) -> bool {
        let first_enemy_piece = |dir_x: i8, dir_y: i8| {
            let mut i: i8 = 1;
            loop {
                if let Some(new_pos) = pos.move_xy(dir_x * i, dir_y * i) {
                    let piece = self.at_pos(pos);

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

            return false;
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
                if dist_x == 1 && dist_y == 1 && ptype == PieceType::King {
                    return true;
                }
            }
            return false;
        };

        let dirs: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        for d in dirs.iter() {
            if straight(d) {
                return true;
            }
        }

        return false;
    }

    pub fn print_ascii(&self, color: Color) {
        let internal_loop = |y| {
            print!("{}| ", y + 1);
            for x in 0..8 {
                print!("{:?} ", self.at_xy(x, y));
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
