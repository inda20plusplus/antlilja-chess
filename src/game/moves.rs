mod inner {
    use crate::{Color, Game, Move, MoveArray, PieceType, Pos, TaggedPiece};
    impl Game {
        pub fn add_pawn_moves(&self, buffer: &mut MoveArray, x: u8, y: u8) {
            let dir: i8 = if self.color == Color::White { 1 } else { -1 };
            let from = Pos::from_xy(x, y);

            let mut add_move = |r#move| {
                buffer.push(r#move);
            };

            let is_safe_move = |r#move| {
                let board_after_move = self.board.board_after_move(from, r#move, self.color);
                return !board_after_move.pos_in_danger(
                    self.king_pos.0,
                    self.king_pos.1,
                    self.color,
                );
            };

            let y_forward = (y as i8 + dir) as u8;
            if self.at_xy(x, y_forward).is_empty() {
                let to = Pos::from_xy(x, y_forward);

                // Promotion
                if y_forward == 0 || y_forward == 7 {
                    let r#move = Move::PawnPromotion(PieceType::Queen, to);

                    if is_safe_move(r#move) {
                        add_move(r#move);
                        add_move(Move::PawnPromotion(PieceType::Knight, to));
                        add_move(Move::PawnPromotion(PieceType::Bishop, to));
                        add_move(Move::PawnPromotion(PieceType::Rook, to));
                    }

                    let board_after_move = self.board.board_after_move(from, r#move, self.color);
                    if !board_after_move.pos_in_danger(self.king_pos.0, self.king_pos.1, self.color)
                    {
                        buffer.push(r#move);
                    }
                }
                // Standard forward
                else {
                    let r#move = Move::Move(to);
                    if is_safe_move(r#move) {
                        buffer.push(r#move);
                    }
                }

                // First move, double forward
                if (y == 1 && self.color == Color::White) || (y == 6 && self.color == Color::Black)
                {
                    let y_off = y as i8 + dir * 2;
                    if (0..8).contains(&y_off) && self.at_xy(x, y_off as u8).is_empty() {
                        let r#move = Move::Move(Pos::from_xy(x, y_off as u8));
                        if is_safe_move(r#move) {
                            buffer.push(r#move);
                        }
                    }
                }
            }

            let mut add_pawn_take = |x: u8, y: u8| {
                let space = self.at_xy(x, y);
                if !space.is_empty() && space.get_color() != self.color {
                    let r#move = Move::Move(Pos::from_xy(x, y));
                    if is_safe_move(r#move) {
                        buffer.push(r#move);
                    }
                }
            };

            if x != 7 {
                add_pawn_take(x + 1, y_forward);
            }

            if x != 0 {
                add_pawn_take(x - 1, y_forward);
            }
        }

        pub fn add_direction_moves(
            &self,
            buffer: &mut MoveArray,
            x: u8,
            y: u8,
            step: (i8, i8),
            r#loop: bool,
        ) {
            let from = Pos::from_xy(x, y);
            let is_king = self.at_pos(from).get_type() == PieceType::King;

            let mut add_move = |dir: &(i8, i8)| {
                let x = x as i8 + (dir.0 * step.0);
                let y = y as i8 + (dir.1 * step.1);

                if !(0..8).contains(&x) || !(0..8).contains(&y) {
                    return false;
                }

                let x = x as u8;
                let y = y as u8;

                let piece = self.at_xy(x, y);

                if piece.is_empty() || piece.get_color() != self.color {
                    let to = Pos::from_xy(x, y);
                    let r#move = Move::Move(to);

                    let (king_x, king_y) = if is_king { (x, y) } else { self.king_pos };

                    let board_with_move = self.board.board_after_move(from, r#move, self.color);
                    if !board_with_move.pos_in_danger(king_x, king_y, self.color) {
                        buffer.push(r#move);
                    }
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

        pub fn add_castling_moves(&self, buffer: &mut MoveArray) {
            let y = if self.color == Color::White { 0 } else { 7 };
            let king_pos = Pos::from_xy(4, y);

            let mut board_without_king = self.board;
            board_without_king.set_pos(king_pos, TaggedPiece::empty());

            let empty_and_not_in_check = |x| {
                if !self.at_xy(x, y).is_empty() {
                    return false;
                }

                if board_without_king.pos_in_danger(x, y, self.color) {
                    return false;
                }

                return true;
            };

            let (r#move, king_x) = if self.at_xy(0, y).is_original()
                && empty_and_not_in_check(1)
                && empty_and_not_in_check(2)
                && empty_and_not_in_check(3)
            {
                (Move::QueenSideCastling, 3)
            } else if self.at_xy(7, y).is_original()
                && empty_and_not_in_check(5)
                && empty_and_not_in_check(6)
            {
                (Move::KingSideCastling, 6)
            } else {
                (Move::None, 0)
            };

            if r#move == Move::None {
                return;
            }

            let board_with_move = self.board.board_after_move(king_pos, r#move, self.color);
            if !board_with_move.pos_in_danger(king_x, y, self.color) {
                buffer.push(r#move);
            }
        }
    }
}
