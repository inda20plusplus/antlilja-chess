#[derive(Copy, Clone, PartialEq)]
pub struct Pos(u8);

impl Pos {
    pub fn from_xy(x: u8, y: u8) -> Self {
        return Pos { 0: y * 8 + x };
    }

    pub fn as_index(&self) -> usize {
        return self.0 as usize;
    }

    pub fn to_xy(&self) -> (u8, u8) {
        let y = self.0 / 8;
        let x = self.0 - (y * 8);
        return (x, y);
    }
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{:?}", self.to_xy());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_pos_and_back() {
        for x in 0..8 {
            for y in 0..8 {
                let pos = Pos::from_xy(x, y);
                let xy = pos.to_xy();

                assert_eq!(x, xy.0);
                assert_eq!(y, xy.1);
            }
        }
    }
}
