#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 128,
}

impl Color {
    pub fn flip(&mut self) {
        unsafe {
            *self = std::mem::transmute((*self as u8) ^ 128);
        }
    }
}
