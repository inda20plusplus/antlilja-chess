use crate::{Color, Move, PieceType, Pos, TaggedPiece};

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

        self.set_pos(to, self.at_pos(from).non_original());
        self.set_pos(from, TaggedPiece::empty());
        return true;
    }

    pub fn find_first_of_type(&self, r#type: PieceType, color: Color) -> Option<Pos> {
        let piece = TaggedPiece::new(r#type, color);
        for (i, p) in self.0.iter().enumerate() {
            if p == &piece {
                return Some(Pos::from_index(i as u8));
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

                let rook_pos = Pos::from_xy(7, y);
                board.move_piece(rook_pos, Pos::from_xy(5, y));

                let king_pos = Pos::from_xy(4, y);
                board.move_piece(king_pos, Pos::from_xy(6, y));
            }
            Move::QueenSideCastling => {
                let y: u8 = if color == Color::White { 0 } else { 7 };

                let rook_pos = Pos::from_xy(0, y);
                board.move_piece(rook_pos, Pos::from_xy(3, y));

                let king_pos = Pos::from_xy(4, y);
                board.move_piece(king_pos, Pos::from_xy(2, y));
            }
            Move::PawnPromotion(r#type, to) => {
                board.move_piece(from, to);
                board.set_pos(to, TaggedPiece::new(r#type, color));
            }
            _ => {}
        }

        return board;
    }

    pub fn pos_in_danger(&self, x: u8, y: u8, color: Color) -> bool {
        let first_enemy_piece = |dir_x: i8, dir_y: i8| {
            let mut i: i8 = 1;
            loop {
                let new_x = x as i8 + dir_x * i;
                let new_y = y as i8 + dir_y * i;

                if !(0..8).contains(&new_x) || !(0..8).contains(&new_y) {
                    return None;
                }

                let pos = Pos::from_xy(new_x as u8, new_y as u8);
                let piece = self.at_pos(pos);

                if !piece.is_empty() {
                    if piece.get_color() == color {
                        return None;
                    } else {
                        return Some((new_x as u8, new_y as u8));
                    }
                }

                i += 1;
            }
        };

        let diag = |config: &(i8, i8, Color)| {
            let xy = first_enemy_piece(config.0, config.1);
            if xy.is_some() {
                let xy = xy.unwrap();
                let pos = Pos::from_xy(xy.0, xy.1);
                let piece = self.at_pos(pos);
                let ptype = piece.get_type();
                if ptype == PieceType::Bishop || ptype == PieceType::Queen {
                    return true;
                }

                use std::cmp::{max, min};

                let dist_x = max(x, xy.0) - min(x, xy.0);
                let dist_y = max(y, xy.1) - min(y, xy.1);
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
            let xy = first_enemy_piece(dir.0, dir.1);
            if xy.is_some() {
                let xy = xy.unwrap();
                let pos = Pos::from_xy(xy.0, xy.1);
                let piece = self.at_pos(pos);
                let ptype = piece.get_type();

                if ptype == PieceType::Rook || ptype == PieceType::Queen {
                    return true;
                }

                use std::cmp::{max, min};

                let dist_x = max(x, xy.0) - min(x, xy.0);
                let dist_y = max(y, xy.1) - min(y, xy.1);
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
            print!("| ");
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
    }
}
