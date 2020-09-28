#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 128,
}

impl Color {
    pub fn flip(&self) -> Self {
        unsafe { std::mem::transmute::<u8, Color>((*self as u8) ^ 128) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip() {
        let color = Color::White;
        assert_eq!(color.flip(), Color::Black);
        assert_eq!(color.flip().flip(), Color::White);
    }
}
