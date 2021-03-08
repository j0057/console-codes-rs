#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Color {
    Black = 0,
    Red = 1,
    Green = 2,
    Brown = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

impl From<Color> for u16 {
    fn from(c: Color) -> Self {
        c as Self
    }
}
