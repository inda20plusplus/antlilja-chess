use crate::Color;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, PartialEq)]
pub struct Pos(u8);

impl Pos {
    pub fn new_xy(x: u8, y: u8) -> Self {
        assert!(x < 8 && y < 8);
        Pos { 0: x | (y << 3) }
    }

    pub fn new_index(i: u8) -> Self {
        assert!(i < 64);
        Self { 0: i }
    }

    pub const fn invalid() -> Self {
        Self { 0: std::u8::MAX }
    }

    pub fn index(&self) -> usize {
        self.0 as usize
    }

    pub fn xy(&self) -> (u8, u8) {
        (self.x(), self.y())
    }

    pub fn x(&self) -> u8 {
        self.0 & 0b111
    }

    pub fn y(&self) -> u8 {
        self.0 >> 3
    }

    pub fn add_x(&self, x: u8) -> Option<Self> {
        let new_x = self.x() + x;
        if (0..8).contains(&new_x) {
            Some(Self { 0: self.0 + x })
        } else {
            None
        }
    }

    pub fn sub_x(&self, x: u8) -> Option<Self> {
        let new_x = self.x() as i8 - x as i8;
        if (0..8).contains(&new_x) {
            Some(Self { 0: self.0 - x })
        } else {
            None
        }
    }

    pub fn move_x(&self, x: i8) -> Option<Self> {
        let abs_x = x.abs() as u8;
        if x > 0 {
            self.add_x(abs_x)
        } else {
            self.sub_x(abs_x)
        }
    }

    pub fn add_y(&self, y: u8) -> Option<Self> {
        let new_y = self.y() + y;
        if (0..8).contains(&new_y) {
            Some(Self {
                0: self.0 + (y << 3),
            })
        } else {
            None
        }
    }

    pub fn sub_y(&self, y: u8) -> Option<Self> {
        let new_y = self.y() as i8 - y as i8;
        if (0..8).contains(&new_y) {
            Some(Self {
                0: self.0 - (y << 3),
            })
        } else {
            None
        }
    }

    pub fn move_y(&self, y: i8) -> Option<Self> {
        let abs_y = y.abs() as u8;
        if y > 0 {
            self.add_y(abs_y)
        } else {
            self.sub_y(abs_y)
        }
    }

    pub fn move_xy(&self, x: i8, y: i8) -> Option<Self> {
        if let Some(pos) = self.move_x(x) {
            pos.move_y(y)
        } else {
            None
        }
    }

    pub fn at_left_edge(&self) -> bool {
        self.0.trailing_zeros() >= 3
    }

    pub fn at_right_edge(&self) -> bool {
        self.0.trailing_ones() >= 3
    }

    pub fn at_x_edge(&self) -> bool {
        self.at_left_edge() || self.at_right_edge()
    }

    pub fn at_white_edge(&self) -> bool {
        (self.0 & 0b111000) == 0
    }

    pub fn at_black_edge(&self) -> bool {
        (self.0 & 0b111000) == 0b111000
    }

    pub fn at_y_edge(&self) -> bool {
        self.at_white_edge() || self.at_black_edge()
    }

    pub fn at_pawn_rank(&self, color: Color) -> bool {
        let y = self.y();

        if color == Color::White && y == 1 {
            true
        } else if color == Color::Black && y == 6 {
            true
        } else {
            false
        }
    }

    pub fn distance_x(&self, other: &Pos) -> u8 {
        (self.x() as i8 - other.x() as i8).abs() as u8
    }

    pub fn distance_y(&self, other: &Pos) -> u8 {
        (self.y() as i8 - other.y() as i8).abs() as u8
    }
}

impl std::fmt::Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.xy())
    }
}
