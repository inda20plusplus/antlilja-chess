use crate::piece::{Color, PieceType, TaggedPiece};
use crate::pos::Pos;
use crate::r#move::Move;

pub struct Board {
    data: [TaggedPiece; 64],
    history: Vec<Move>,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            data: [TaggedPiece::empty(); 64],
            history: Vec::<Move>::with_capacity(64),
        };

        for i in 0..8 {
            board.data[8 + i] = TaggedPiece::original(PieceType::Pawn, Color::White);
            board.data[8 * 6 + i] = TaggedPiece::original(PieceType::Pawn, Color::Black);
        }

        let mut place_at_both_sides = |offset, r#type| {
            board.data[offset] = TaggedPiece::original(r#type, Color::White);
            board.data[8 * 7 + offset] = TaggedPiece::original(r#type, Color::Black);
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

        return board;
    }

    pub fn at(&self, x: u8, y: u8) -> TaggedPiece {
        return self.data[(y * 8 + x) as usize];
    }

    pub fn at_index(&self, i: u8) -> TaggedPiece {
        return self.data[i as usize];
    }

    pub fn get_moves_for(&self, buffer: &mut Vec<Move>, x: u8, y: u8) -> Option<usize> {
        let piece = self.at(x, y);

        return match piece.get_type() {
            PieceType::Pawn => Some(self.add_pawn_moves(buffer, piece.get_color(), x, y)),
            PieceType::Rook => Some(self.add_rook_moves(buffer, piece.get_color(), x, y)),
            PieceType::Knight => Some(self.add_knight_moves(buffer, piece.get_color(), x, y)),
            PieceType::Bishop => Some(self.add_bishop_moves(buffer, piece.get_color(), x, y)),
            PieceType::Queen => Some(
                self.add_rook_moves(buffer, piece.get_color(), x, y)
                    + self.add_bishop_moves(buffer, piece.get_color(), x, y),
            ),
            PieceType::King => Some(self.add_king_moves(buffer, piece.get_color(), x, y)),
            _ => return None,
        };
    }

    fn add_pawn_moves(&self, buffer: &mut Vec<Move>, color: Color, x: u8, y: u8) -> usize {
        if y == 0 || y == 7 {
            return 0;
        }

        let from = Pos::from_xy(x, y);

        let mut count: usize = 0;
        let mut add_pawn_move = |to| {
            buffer.push(Move::Move(from, to));
            count += 1;
        };

        let dir: i8 = if color == Color::White { 1 } else { -1 };

        let y_forward = (y as i8 + dir) as u8;
        if self.at(x, y_forward).is_empty() {
            add_pawn_move(Pos::from_xy(x, y_forward));

            let y_off = y as i8 + dir * 2;
            if (0..8).contains(&y_off) {
                let y_off = y_off as u8;

                if (y == 1 || y == 6) && self.at(x, y_off).is_empty() {
                    add_pawn_move(Pos::from_xy(x, y_off));
                }
            }
        }

        let mut add_pawn_take = |x: u8, y: u8| {
            let space = self.at(x, y);
            if !space.is_empty() && space.get_color() != color {
                add_pawn_move(Pos::from_xy(x, y));
            }
        };

        if x != 7 {
            add_pawn_take(x + 1, y_forward);
        }

        if x != 0 {
            add_pawn_take(x - 1, y_forward);
        }

        return count;
    }

    fn add_rook_moves(&self, buffer: &mut Vec<Move>, color: Color, x: u8, y: u8) -> usize {
        let from = Pos::from_xy(x, y);

        let mut count = 0;
        let mut loop_internal = |x, y| {
            let i = y * 8 + x;
            let space = self.data[i as usize];
            if space.is_empty() || space.get_color() != color {
                buffer.push(Move::Move(from, Pos::from_xy(x, y)));
                count += 1;
            }

            return space.is_empty();
        };

        for x in (x + 1)..8 {
            if !loop_internal(x, y) {
                break;
            }
        }

        for x in x..0 {
            if !loop_internal(x, y) {
                break;
            }
        }

        for y in (y + 1)..8 {
            if !loop_internal(x, y) {
                break;
            }
        }

        for y in y..0 {
            if !loop_internal(x, y) {
                break;
            }
        }

        return count;
    }

    fn add_knight_moves(&self, buffer: &mut Vec<Move>, color: Color, x: u8, y: u8) -> usize {
        let from = Pos::from_xy(x, y);

        let mut count = 0;
        let mut add_move = |x_dir: i8, y_dir: i8| {
            let to_x = x as i8 + x_dir;
            let to_y = y as i8 + y_dir * 2;
            if (0..8).contains(&to_x) && (0..8).contains(&to_y) {
                let to_x = to_x as u8;
                let to_y = to_y as u8;
                let piece = self.data[(to_y * 8 + to_x) as usize];
                if piece.is_empty() || piece.get_color() != color {
                    buffer.push(Move::Move(from, Pos::from_xy(to_x, to_y)));
                    count += 1;
                }
            };
        };

        add_move(1, 1);
        add_move(1, -1);
        add_move(-1, -1);
        add_move(-1, 1);

        return count;
    }

    fn add_bishop_moves(&self, buffer: &mut Vec<Move>, color: Color, x: u8, y: u8) -> usize {
        use std::cmp::min;

        let from = Pos::from_xy(x, y);
        let mut count = 0;
        let mut check = |x_dir: i8, y_dir: i8, dist: u8| {
            for off in 1..dist {
                let x = (x as i8 + (off as i8 * x_dir as i8)) as u8;
                let y = (y as i8 + (off as i8 * y_dir as i8)) as u8;
                let piece = self.at(x, y);

                if piece.is_empty() || piece.get_color() != color {
                    buffer.push(Move::Move(from, Pos::from_xy(x, y)));
                    count += 1;
                }

                if !piece.is_empty() {
                    break;
                }
            }
        };

        // NW
        check(-1, -1, min(x, y));

        // NE
        check(1, -1, min(8 - x, y));

        // SE
        check(1, 1, min(8 - x, 8 - y));

        // SW
        check(-1, 1, min(x, 8 - y));

        return count;
    }

    fn add_king_moves(&self, buffer: &mut Vec<Move>, color: Color, x: u8, y: u8) -> usize {
        let mut count = 0;

        let mut check = |x_dir: i8, y_dir: i8| {
            let to_x = x as i8 + x_dir;
            let to_y = y as i8 + y_dir;
            if (0..8).contains(&to_x) && (0..8).contains(&to_y) {
                let piece = self.at(to_x as u8, to_y as u8);

                if piece.is_empty() || piece.get_color() != color {
                    buffer.push(Move::Move(
                        Pos::from_xy(x, y),
                        Pos::from_xy(to_x as u8, to_y as u8),
                    ));
                    count += 1;
                }
            }
        };

        check(1, 1);
        check(-1, 1);
        check(-1, -1);
        check(1, -1);

        return count;
    }

    pub fn print(&self) {
        for i in 0..8 {
            let start = i * 8;
            let end = start + 8;
            println!("{:?}", &self.data[start..end]);
        }
    }
}
