mod inner {
    use crate::{Color, Game, Move, PieceType, Pos, TaggedPiece};
    impl Game {
        pub fn add_pawn_moves(&mut self, x: u8, y: u8) {
            let dir: i8 = if self.color == Color::White { 1 } else { -1 };
            let from = Pos::from_xy(x, y);
            let color = self.color;

            let y_forward = (y as i8 + dir) as u8;
            if self.at_xy(x, y_forward).is_empty() {
                let to = Pos::from_xy(x, y_forward);
                
                // Promotion
                if y_forward == 0 || y_forward == 7 {
                    let r#move = Move::PawnPromotion(PieceType::Queen, to);
                    
                    if !self.king_in_danger_after_move(from, r#move) {
                        self.move_map.insert(r#move);
                        self.move_map.insert(Move::PawnPromotion(PieceType::Knight, to));
                        self.move_map.insert(Move::PawnPromotion(PieceType::Bishop, to));
                        self.move_map.insert(Move::PawnPromotion(PieceType::Rook, to));
                    }
                }
                // Standard forward
                else {
                    let r#move = Move::Move(to);
                    if !self.king_in_danger_after_move(from, r#move) {
                        self.move_map.insert(r#move);
                    }
                }

                // First move, double forward
                if (y == 1 && color == Color::White) || (y == 6 && color == Color::Black)
                {
                    let y_off = y as i8 + dir * 2;
                    if (0..8).contains(&y_off) && self.at_xy(x, y_off as u8).is_empty() {
                        let r#move = Move::move_xy(x, y_off as u8);
                        if !self.king_in_danger_after_move(from, r#move) {
                            self.move_map.insert(r#move);
                        }
                    }
                }
            }

            let mut add_pawn_take = |x: u8, y: u8| {
                let space = self.at_xy(x, y);

                let last = self.history.last();
                let r#move = 
                // Standard diagonal pawn take
                if !space.is_empty() && space.get_color() != self.color {
                    Move::move_xy(x, y)
                } 
                // En passant
                else if last.is_some() {
                    let (last_from, last_move) = last.unwrap();
                    let mut r#move = Move::None;
                    
                    if let Move::Move(to_pos) = last_move {
                        let (from_x, from_y) = last_from.to_xy();
                        let (_, to_y) = to_pos.to_xy();
                        if (from_y as i8 - to_y as i8).abs() == 2 
                            && from_x == x 
                            && y == (from_y as i8 + (dir * -1)) as u8 
                        {
                            r#move = Move::EnPassant(Pos::from_xy(x, y));
                        }
                    }
                    r#move
                }
                else {
                    Move::None
                };

                if r#move != Move::None && !self.king_in_danger_after_move(from, r#move) {
                    self.move_map.insert(r#move);
                }
            };

            if x != 7 {
                add_pawn_take(x + 1, y_forward);
            }

            if x != 0 {
                add_pawn_take(x - 1, y_forward);
            }
        }

        pub fn add_straight_moves(&mut self, x: u8, y: u8) {
            let from = Pos::from_xy(x, y);

            let mut loop_internal = |x, y| {
                let space = self.board.at_xy(x, y);
                if space.is_empty() || space.get_color() != self.color {
                    let r#move = Move::move_xy(x, y);
                    if !self.king_in_danger_after_move(from, r#move) {
                        self.move_map.insert(r#move);
                    }
                }
                
                return space.is_empty();
            };
            
            // Right
            for x in (x + 1)..8 {
                if !loop_internal(x, y) {
                    break;
                }
            }
            
            // Left
            for dist in 1..(x+1) {
                if !loop_internal(x-dist, y) {
                     break;
                }
            }

            // Up
            for y in (y + 1)..8 {
                if !loop_internal(x, y) {
                    break;
                }
            }

            // Down
            for dist in 1..(y+1) {
                if !loop_internal(x, y-dist) {
                    break;
                }
            }
        }

        pub fn add_diagonal_moves(&mut self, x: u8, y: u8) {
            use std::cmp::min;

            let from = Pos::from_xy(x, y);
            let mut check = |x_dir: i8, y_dir: i8, dist: u8| {
                for off in 1..dist {
                    let x = (x as i8 + (off as i8 * x_dir as i8)) as u8;
                    let y = (y as i8 + (off as i8 * y_dir as i8)) as u8;
                    let piece = self.at_xy(x, y);

                    if piece.is_empty() || piece.get_color() != self.color {
                        let r#move = Move::move_xy(x, y);
                        if !self.king_in_danger_after_move(from, r#move) {
                            self.move_map.insert(r#move);
                        }
                    }

                    if !piece.is_empty() {
                        break;
                    }
                }
            };

            // SW
            check(-1, -1, min(x, y) + 1);

            // SE
            check(1, -1, min(8 - x, y));

            // NE
            check(1, 1, min(8 - x, 8 - y));

            // NW
            check(-1, 1, min(x, 8 - y));
        }

        pub fn add_knight_moves(&mut self, x: u8, y: u8) {
            let from = Pos::from_xy(x, y);

            let mut add_move = |x_dir: i8, y_dir: i8| {
                let mut add = |to_x, to_y| {
                    if (0..8).contains(&to_x) && (0..8).contains(&to_y) {
                        let x = to_x as u8;
                        let y = to_y as u8;
                        let piece = self.at_xy(x, y);
                        if piece.is_empty() || piece.get_color() != self.color {
                            let r#move = Move::move_xy(x, y);
                            if !self.king_in_danger_after_move(from, r#move) {
                                self.move_map.insert(r#move);
                            }
                        }
                    };
                };

                add(x as i8 + x_dir, y as i8 + y_dir * 2);
                add(x as i8 + x_dir * 2, y as i8 + y_dir);
            };

            add_move(1, 1);
            add_move(1, -1);
            add_move(-1, -1);
            add_move(-1, 1);
        }

        pub fn add_king_moves(&mut self, x: u8, y: u8) {
            let from = Pos::from_xy(x, y);

            let mut check = |x_dir: i8, y_dir: i8| {
                let to_x = x as i8 + x_dir;
                let to_y = y as i8 + y_dir;
                if (0..8).contains(&to_x) && (0..8).contains(&to_y) {
                    let x = to_x as u8;
                    let y = to_y as u8;

                    let piece = self.at_xy(x, y);
                    
                    if piece.is_empty() || piece.get_color() != self.color {
                        
                        let r#move = Move::move_xy(x, y);   
                        let board_after_move = self.board.board_after_move(from, r#move, self.color);
                        
                        if !board_after_move.pos_in_danger(x, y, self.color) {
                            self.move_map.insert(r#move);
                        }
                    }
                }
            };

            check(1, 1);
            check(1, 0);
            check(0, 1);
            check(0, -1);
            check(-1, 0);
            check(-1, 1);
            check(-1, -1);
            check(1, -1);
        }

        pub fn add_castling_moves(&mut self) {
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
                self.move_map.insert(r#move);
            }
        }
    }
}
