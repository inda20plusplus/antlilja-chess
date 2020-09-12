use crate::{Move, Pos};

const MOVE_ARRAY_SIZE: usize = 28;

#[derive(Copy, Clone)]
pub struct MoveArray {
    size_uninit: u8,
    data: [Move; MOVE_ARRAY_SIZE],
}

impl PartialEq for MoveArray {
    fn eq(&self, other: &Self) -> bool {
        if self.size_uninit != other.size_uninit {
            return false;
        }

        for i in 0..self.size() {
            if !self.data.contains(&other.at(i)) {
                return false;
            }
        }

        return true;
    }
}

impl MoveArray {
    pub fn uninitalized() -> Self {
        return MoveArray {
            size_uninit: 128,
            data: [Move::None; MOVE_ARRAY_SIZE],
        };
    }

    pub fn empty() -> Self {
        return MoveArray {
            size_uninit: 0,
            data: [Move::None; MOVE_ARRAY_SIZE],
        };
    }

    pub fn from_slice(slice: &[Move]) -> Self {
        let mut arr = Self::empty();

        arr.size_uninit = slice.len() as u8;
        for i in 0..slice.len() {
            arr.data[i] = slice[i];
        }

        return arr;
    }

    pub fn is_empty(&self) -> bool {
        return self.size() == 0;
    }

    pub fn is_initalized(&self) -> bool {
        return (self.size_uninit & 128) == 0;
    }

    pub fn size(&self) -> usize {
        return (self.size_uninit & 127) as usize;
    }

    pub fn at(&self, index: usize) -> Move {
        return self.data[index];
    }

    pub fn find(&self, r#move: Move) -> Option<usize> {
        for (i, m) in self.data.iter().enumerate() {
            if &r#move == m {
                return Some(i);
            }
        }

        return None;
    }

    pub fn push(&mut self, r#move: Move) {
        assert_ne!(self.size(), MOVE_ARRAY_SIZE);

        self.data[self.size()] = r#move;
        self.size_uninit += 1;
    }
}

impl std::fmt::Debug for MoveArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[").unwrap();
        for i in 0..self.size() {
            write!(f, "{:?}", self.at(i)).unwrap();
            if i != self.size() - 1 {
                write!(f, ", ").unwrap();
            }
        }

        return write!(f, "]");
    }
}

pub struct MoveMap {
    moves: [MoveArray; 64],
}

impl MoveMap {
    pub fn new() -> Self {
        return MoveMap {
            moves: [MoveArray::uninitalized(); 64],
        };
    }

    pub fn insert(&mut self, pos: Pos, array: MoveArray) -> &MoveArray {
        let index = pos.as_index();
        self.moves[index] = array;
        return &self.moves[index];
    }

    pub fn at(&self, pos: Pos) -> &MoveArray {
        return &self.moves[pos.as_index()];
    }

    pub fn clear(&mut self) {
        for i in 0..64 {
            self.moves[i] = MoveArray::uninitalized();
        }
    }
}
