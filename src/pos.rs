#[derive(Copy, Clone, PartialEq)]
pub struct Pos(u8);

impl Pos {
    pub fn new_xy(x: u8, y: u8) -> Self {
        Pos { 0: x | (y << 3) }
    }

    pub fn new_index(i: u8) -> Self {
        Self { 0: i }
    }

    pub fn index(&self) -> usize {
        self.0 as usize
    }

    pub fn xy(&self) -> (u8, u8) {
        (self.0 & 0b111, self.0 >> 3)
    }
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.xy())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_pos_and_back() {
        for x in 0..8 {
            for y in 0..8 {
                let pos = Pos::new_xy(x, y);
                let (pos_x, pos_y) = pos.xy();

                assert_eq!(x, pos_x);
                assert_eq!(y, pos_y);
            }
        }
    }

    #[test]
    fn pos_equal_to_index() {
        for x in 0..8 {
            for y in 0..8 {
                let index = (y * 8 + x) as usize;
                let pos_index = Pos::new_xy(x, y).index();

                assert_eq!(index, pos_index);
            }
        }
    }
}
