mod inner {
    use crate::{Color, Game, Move, PieceType, Pos, TaggedPiece};
    impl Game {
        pub fn add_pawn_moves(&mut self, from: Pos) {
            let y_dir: i8 = if self.player == Color::White { 1 } else { -1 };
            
            if let Some(to) = from.move_y(y_dir)  {
                if self.at_pos(to).is_empty() {
                    // Promotion
                    if to.at_y_edge() {
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
                }
            }

            // First move, double forward
            if from.at_pawn_rank(self.player) {
                if let Some(to) = from.move_y(y_dir * 2) {
                    if self.at_pos(to).is_empty() {
                        let r#move = Move::Move(to);
                        if !self.king_in_danger_after_move(from, r#move) {
                            self.move_map.insert(r#move);
                        }
                    }
                }
            }

            let mut add_pawn_take = |to| {
                let space = self.at_pos(to);

                let r#move = 
                // Standard diagonal pawn take
                if !space.is_empty() && space.color() != self.player {
                    Move::Move(to)
                } 
                // En passant
                else if let Some(last) = self.history.last() {
                    let (_, last_from, last_move) = last;
                    let mut r#move = Move::None;
                    if let Move::Move(last_to) = last_move {
                        if last_from.distance_y(last_to) == 2 && last_from.move_y_non_fail(y_dir * -1) == to {
                            r#move = Move::EnPassant(to)
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

            let y_forward = from.move_y_non_fail(y_dir);
            if !from.at_right_edge() {
                add_pawn_take(y_forward.add_x(1));
            }

            if !from.at_left_edge() {
                add_pawn_take(y_forward.sub_x(1));
            }
        }

        pub fn add_straight_moves(&mut self, from: Pos) {
            let mut loop_internal = |to| {
                let space = self.board.at_pos(to);
                if space.is_empty() || space.color() != self.player {
                    let r#move = Move::Move(to);
                    if !self.king_in_danger_after_move(from, r#move) {
                        self.move_map.insert(r#move);
                    }
                }
                
                return space.is_empty();
            };

            let (x, y) = from.xy();
            
            // Right
            for x in (x + 1)..8 {
                if !loop_internal(Pos::new_xy(x, y)) {
                    break;
                }
            }
            
            // Left
            for dist in 1..(x+1) {
                if !loop_internal(from.sub_x(dist)) {
                     break;
                }
            }

            // Up
            for y in (y + 1)..8 {
                if !loop_internal(Pos::new_xy(x, y)) {
                    break;
                }
            }

            // Down
            for dist in 1..(y+1) {
                if !loop_internal(from.sub_y(dist)) {
                    break;
                }
            }
        }

        pub fn add_diagonal_moves(&mut self, from: Pos) {
            use std::cmp::min;

            let mut check = |x_dir: i8, y_dir: i8, dist: u8| {
                let dist = dist as i8;
                for off in 1..dist {
                    let to = from.move_xy_non_fail(off * x_dir, off * y_dir);
                    let piece = self.at_pos(to);

                    if piece.is_empty() || piece.color() != self.player {
                        let r#move = Move::Move(to);
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
            check(-1, -1, min(from.x(), from.y()) + 1);

            // SE
            check(1, -1, min(8 - from.x(), from.y()));

            // NE
            check(1, 1, min(8 - from.x(), 8 - from.y()));

            // NW
            check(-1, 1, min(from.x(), 8 - from.y()));
        }

        pub fn add_knight_moves(&mut self, from: Pos) {
            let mut add_move = |x_dir: i8, y_dir: i8| {
                let mut add = |to| {
                    let piece = self.at_pos(to);
                    if piece.is_empty() || piece.color() != self.player {
                        let r#move = Move::Move(to);
                        if !self.king_in_danger_after_move(from, r#move) {
                            self.move_map.insert(r#move);
                        }
                    }

                };

                if let Some(to) = from.move_xy(x_dir, y_dir * 2) {
                    add(to);
                }

                if let Some(to) = from.move_xy(x_dir * 2, y_dir) {
                    add(to);
                }
            };

            add_move(1, 1);
            add_move(1, -1);
            add_move(-1, -1);
            add_move(-1, 1);
        }

        pub fn add_king_moves(&mut self, from: Pos) {
            let mut check = |x_dir: i8, y_dir: i8| {
                if let Some(to) = from.move_xy(x_dir, y_dir) {
                    let piece = self.at_pos(to);
                    
                    if piece.is_empty() || piece.color() != self.player {
                        let r#move = Move::Move(to);   
                        let board_after_move = self.board.board_after_move(from, r#move, self.player);
                        
                        if !board_after_move.pos_in_danger(to, self.player) {
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
            let y = if self.player == Color::White { 0 } else { 7 };
            let king_pos = Pos::new_xy(4, y);

            let mut board_without_king = self.board;
            board_without_king.set_pos(king_pos, TaggedPiece::empty());

            let empty_and_not_in_check = |x| {
                let to = Pos::new_xy(x, y);
                if !self.at_pos(to).is_empty() {
                    return false;
                }

                if board_without_king.pos_in_danger(to, self.player) {
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

            let board_with_move = self.board.board_after_move(king_pos, r#move, self.player);
            let new_king_pos = Pos::new_xy(king_x, y);
            if !board_with_move.pos_in_danger(new_king_pos, self.player) {
                self.move_map.insert(r#move);
            }
        }
    }
}
