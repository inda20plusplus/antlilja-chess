use crate::{Move, Pos};

const MAX_ROOK_MOVES: usize = 14;
const MAX_KNIGHT_MOVES: usize = 8;
const MAX_BISHOP_MOVES: usize = 13;
const MAX_KING_MOVES: usize = 8;
const MAX_QUEEN_MOVES: usize = MAX_ROOK_MOVES + MAX_BISHOP_MOVES;

const MAX_MOVES: usize = MAX_ROOK_MOVES * 2
    + MAX_BISHOP_MOVES * 2
    + MAX_KNIGHT_MOVES * 2
    + MAX_QUEEN_MOVES * 9
    + MAX_KING_MOVES;

const MAX_PIECES: usize = 16;

#[derive(Copy, Clone)]
struct PieceMoves {
    stack_index: u8,
    len: u8,
    board_index: u8,
}

impl PieceMoves {
    pub fn empty() -> Self {
        PieceMoves {
            stack_index: 0,
            len: 0,
            board_index: 128,
        }
    }

    pub fn new(stack_index: u8, board_index: u8) -> Self {
        PieceMoves {
            stack_index,
            len: 0,
            board_index,
        }
    }

    pub fn is_unused(&self) -> bool {
        self.board_index == 128
    }

    pub fn stack_index(&self) -> usize {
        self.stack_index as usize
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn board_index(&self) -> u8 {
        self.board_index
    }

    pub fn increment_len(&mut self) {
        self.len += 1;
    }
}

pub struct MoveMap {
    moves: [Move; MAX_MOVES],
    pieces: [PieceMoves; MAX_PIECES],
    current_stack_index: usize,
    current_piece_index: usize,
}

impl MoveMap {
    pub fn new() -> Self {
        MoveMap {
            moves: [Move::None; MAX_MOVES],
            pieces: [PieceMoves::empty(); MAX_PIECES],
            current_stack_index: 0,
            current_piece_index: 0,
        }
    }

    fn find_new_index(&self, board_index: usize) -> usize {
        let mut index = board_index % MAX_PIECES;
        loop {
            if self.pieces[index].is_unused() {
                return index;
            }

            if index == 15 {
                index = 0;
            } else {
                index += 1;
            }
        }
    }

    fn find_used_index(&self, board_index: usize) -> Option<usize> {
        let mut index = board_index % MAX_PIECES;
        let start = index;
        loop {
            if self.pieces[index].board_index() == board_index as u8 {
                return Some(index);
            }

            if self.pieces[index].is_unused() {
                return None;
            }

            if index == 15 {
                index = 0;
            } else {
                index += 1;
            }

            if index == start {
                return None;
            }
        }
    }

    pub fn current_pos_moves_len(&self) -> usize {
        self.pieces[self.current_piece_index].len()
    }

    pub fn at(&self, pos: Pos) -> Option<&[Move]> {
        let index = self.find_used_index(pos.index());

        if index.is_none() {
            return None;
        }

        let index = index.unwrap();
        let moves = self.pieces[index];
        let len = moves.len() + moves.stack_index();
        Some(&self.moves[moves.stack_index()..len])
    }

    pub fn set_current_pos(&mut self, pos: Pos) {
        let board_index = pos.index();
        self.current_piece_index = self.find_new_index(board_index);
        self.pieces[self.current_piece_index] =
            PieceMoves::new(self.current_stack_index as u8, board_index as u8);
    }

    pub fn insert(&mut self, r#move: Move) {
        self.moves[self.current_stack_index] = r#move;
        self.current_stack_index += 1;
        self.pieces[self.current_piece_index].increment_len();
    }

    pub fn clear(&mut self) {
        self.current_stack_index = 0;
        self.pieces = [PieceMoves::empty(); MAX_PIECES];
    }
}
