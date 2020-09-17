use crate::{Board, Color, Move, MoveArray, MoveMap, PieceType, Pos, TaggedPiece};

#[derive(PartialEq)]
pub enum Result {
    Ok,
    Checkmate,
    Stalemate,
}

pub struct Game {
    board: Board,
    move_map: MoveMap,
    history: Vec<Move>,
    color: Color,
    king_pos: (u8, u8),
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            board: Board::new(),
            move_map: MoveMap::new(),
            history: Vec::<Move>::with_capacity(50),
            color: Color::White,
            king_pos: (4, 0),
        };

        game.calculate_all_moves();

        return game;
    }

    pub fn switch_side(&mut self) -> bool {
        self.move_map.clear();
        self.color.flip();
        self.king_pos = self.board.find_king(self.color).to_xy();
        return self.calculate_all_moves();
    }

    pub fn current_color(&self) -> Color {
        return self.color;
    }

    pub fn play(&mut self, x: u8, y: u8, index: usize) -> Result {
        let pos = Pos::from_xy(x, y);

        assert!(pos.as_index() < 64);
        assert!(!self.at_pos(pos).is_empty());

        let moves: &MoveArray = if self.move_map.at(pos).is_empty() {
            self.get_moves_for(x, y);
            &self.move_map.at(pos)
        } else {
            &self.move_map.at(pos)
        };

        assert!(index < moves.size());

        let r#move = moves.at(index);

        self.board = self.board.board_after_move(pos, r#move, self.color);

        self.history.push(r#move);
        if self.switch_side() {
            let (x, y) = self.king_pos;
            if self.board.pos_in_danger(x, y, self.color) {
                return Result::Checkmate;
            } else {
                return Result::Stalemate;
            }
        } else {
            return Result::Ok;
        }
    }

    pub fn at_xy(&self, x: u8, y: u8) -> TaggedPiece {
        return self.board.at_xy(x, y);
    }

    pub fn at_pos(&self, pos: Pos) -> TaggedPiece {
        return self.board.at_pos(pos);
    }

    pub fn at_index(&self, i: usize) -> TaggedPiece {
        return self.board.at_index(i);
    }

    fn calculate_all_moves(&mut self) -> bool {
        let mut checkmate = true;
        for y in 0..8 {
            for x in 0..8 {
                if self.calculate_moves_for(x, y) > 0 {
                    checkmate = false;
                }
            }
        }

        return checkmate;
    }

    fn calculate_moves_for(&mut self, x: u8, y: u8) -> usize {
        let pos = Pos::from_xy(x, y);
        let piece = self.at_pos(pos);

        if piece.get_color() != self.color {
            return 0;
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
                self.add_direction_moves(&mut moves, x, y, (1, 1), false);
                self.add_castling_moves(&mut moves);
            }
            _ => {
                return 0;
            }
        };

        self.move_map.insert(pos, moves);
        return moves.size();
    }

    pub fn get_moves_for(&self, x: u8, y: u8) -> Option<&MoveArray> {
        let pos = Pos::from_xy(x, y);

        if self.at_pos(pos).is_empty() {
            return None;
        }

        return Some(&self.move_map.at(pos));
    }

    fn add_pawn_moves(&self, buffer: &mut MoveArray, x: u8, y: u8) {
        let dir: i8 = if self.color == Color::White { 1 } else { -1 };
        let from = Pos::from_xy(x, y);

        let mut add_move = |r#move| {
            buffer.push(r#move);
        };

        let is_safe_move = |r#move| {
            let board_after_move = self.board.board_after_move(from, r#move, self.color);
            return !board_after_move.pos_in_danger(self.king_pos.0, self.king_pos.1, self.color);
        };

        let y_forward = (y as i8 + dir) as u8;
        if self.at_xy(x, y_forward).is_empty() {
            let to = Pos::from_xy(x, y_forward);
            if y_forward == 0 || y_forward == 7 {
                let r#move = Move::PawnPromotion(PieceType::Queen, to);

                if is_safe_move(r#move) {
                    add_move(r#move);
                    add_move(Move::PawnPromotion(PieceType::Knight, to));
                    add_move(Move::PawnPromotion(PieceType::Bishop, to));
                    add_move(Move::PawnPromotion(PieceType::Rook, to));
                }

                let board_after_move = self.board.board_after_move(from, r#move, self.color);
                if !board_after_move.pos_in_danger(self.king_pos.0, self.king_pos.1, self.color) {
                    buffer.push(r#move);
                }
            } else {
                let r#move = Move::Move(to);
                if is_safe_move(r#move) {
                    buffer.push(r#move);
                }
            }

            let y_off = y as i8 + dir * 2;
            if (0..8).contains(&y_off) {
                let y_off = y_off as u8;

                if ((y == 1 && self.color == Color::White)
                    || (y == 6 && self.color == Color::Black))
                    && self.at_xy(x, y_off).is_empty()
                {
                    let r#move = Move::Move(Pos::from_xy(x, y_off));
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

    fn add_direction_moves(
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

    fn add_castling_moves(&self, buffer: &mut MoveArray) {
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

    pub fn print_ascii(&self) {
        self.board.print_ascii(self.color);
    }
}
