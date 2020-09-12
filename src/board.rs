use crate::move_map::{MoveArray, MoveMap};
use crate::piece::{Color, PieceType, TaggedPiece};
use crate::pos::Pos;
use crate::r#move::Move;

pub struct Board {
    data: [TaggedPiece; 64],
    move_map: MoveMap,
    history: Vec<Move>,
    color: Color,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            data: [TaggedPiece::empty(); 64],
            move_map: MoveMap::new(),
            history: Vec::<Move>::with_capacity(50),
            color: Color::White,
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

    pub fn flip_color(&mut self) {
        self.color.flip();
    }

    pub fn play(&mut self, x: u8, y: u8, index: usize) -> bool {
        let pos = Pos::from_xy(x, y);

        if pos.as_index() >= 64 || self.at_pos(pos).is_empty() {
            return false;
        }
        let moves: &MoveArray = if self.move_map.at(pos).is_empty() {
            self.get_moves_for(x, y);
            &self.move_map.at(pos)
        } else {
            &self.move_map.at(pos)
        };

        if index >= moves.size() {
            return false;
        }

        let r#move = moves.at(index);

        match r#move {
            Move::Move(from, to) => {
                let to = to.as_index();
                let from = from.as_index();
                self.data[to] = self.data[from];
                self.data[from] = TaggedPiece::empty();
            }
            _ => {
                return false;
            }
        }

        self.history.push(*r#move);
        self.move_map.clear();
        self.color.flip();
        return true;
    }

    pub fn at(&self, x: u8, y: u8) -> TaggedPiece {
        return self.data[(y * 8 + x) as usize];
    }

    pub fn at_pos(&self, pos: Pos) -> TaggedPiece {
        return self.data[pos.as_index()];
    }

    pub fn at_index(&self, i: u8) -> TaggedPiece {
        return self.data[i as usize];
    }

    pub fn get_moves_for(&mut self, x: u8, y: u8) -> Option<&MoveArray> {
        let pos = Pos::from_xy(x, y);
        if !self.move_map.at(pos).is_empty() {
            return Some(&self.move_map.at(pos));
        }

        let piece = self.at_pos(pos);

        if piece.get_color() != self.color {
            return None;
        }

        let mut moves = MoveArray::empty();

        match piece.get_type() {
            PieceType::Pawn => {
                self.add_pawn_moves(&mut moves, x, y);
            }
            PieceType::Rook => {
                self.add_direction_moves(&mut moves, x, y, (0, 1), true);
                self.add_direction_moves(&mut moves, x, y, (1, 0), true);
            }
            PieceType::Knight => {
                self.add_direction_moves(&mut moves, x, y, (1, 2), false);
                self.add_direction_moves(&mut moves, x, y, (2, 1), false);
            }
            PieceType::Bishop => {
                self.add_direction_moves(&mut moves, x, y, (1, 1), true);
            }
            PieceType::Queen => {
                self.add_direction_moves(&mut moves, x, y, (0, 1), true);
                self.add_direction_moves(&mut moves, x, y, (1, 0), true);
                self.add_direction_moves(&mut moves, x, y, (1, 1), true);
            }
            PieceType::King => {
                self.add_direction_moves(&mut moves, x, y, (1, 0), false);
                self.add_direction_moves(&mut moves, x, y, (0, 1), false);
                self.add_castling_moves(&mut moves);
            }
            _ => return None,
        };

        return Some(self.move_map.insert(pos, moves));
    }

    fn add_pawn_moves(&self, buffer: &mut MoveArray, x: u8, y: u8) {
        let from = Pos::from_xy(x, y);

        let mut add_pawn_move = |to| {
            buffer.push(Move::Move(from, to));
        };

        let dir: i8 = if self.color == Color::White { 1 } else { -1 };

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
            if !space.is_empty() && space.get_color() != self.color {
                add_pawn_move(Pos::from_xy(x, y));
            }
        };

        if x != 7 {
            add_pawn_take(x + 1, y_forward);
        }

        if x != 0 {
            add_pawn_take(x - 1, y_forward);
        }
    }

    fn add_direction_moves(
        &self,
        buffer: &mut MoveArray,
        x: u8,
        y: u8,
        step: (i8, i8),
        r#loop: bool,
    ) {
        let from = Pos::from_xy(x, y);

        let mut add_move = |dir: &(i8, i8)| {
            let x = x as i8 + (dir.0 * step.0);
            let y = y as i8 + (dir.1 * step.1);

            if !(0..8).contains(&x) || !(0..8).contains(&y) {
                return false;
            }

            let x = x as u8;
            let y = y as u8;

            let piece = self.at(x, y);

            if piece.is_empty() || piece.get_color() != self.color {
                buffer.push(Move::Move(from, Pos::from_xy(x, y)));
            }

            return piece.is_empty();
        };

        let directions: [(i8, i8); 4] = [(-1, -1), (1, -1), (1, 1), (-1, 1)];

        if r#loop {
            let mut add_moves = |dir: &(i8, i8)| {
                let mut i: i8 = 1;
                loop {
                    let new_dir = (dir.0 * i, dir.1 * i);
                    if !add_move(&new_dir) {
                        break;
                    }
                    i += 1;
                }
            };

            for d in directions.iter() {
                add_moves(d);
            }
        } else {
            for d in directions.iter() {
                add_move(d);
            }
        }
    }

    fn add_castling_moves(&self, buffer: &mut MoveArray) {
        let empty_at = |x, y| {
            return self.at(x, y).is_empty();
        };

        let mut add_castling_move = |y| {
            if self.at(0, y).is_original() && empty_at(1, y) && empty_at(2, y) && empty_at(3, y) {
                buffer.push(Move::QueenSideCastling);
            }

            if self.at(7, y).is_original() && empty_at(5, y) && empty_at(6, y) {
                buffer.push(Move::KingSideCastling);
            }
        };

        if self.color == Color::White {
            add_castling_move(0);
        } else {
            add_castling_move(7);
        }
    }

    pub fn print_ascii(&self) {
        for y in (0..8).rev() {
            print!("| ");
            for x in 0..8 {
                print!("{:?} ", self.at(x, y));
            }
            println!("|")
        }
    }
}
