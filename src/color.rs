use crate::color::rgb8::ColorRGB8;

pub mod rgb8;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ColorFormat {
    RGB8,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    RGB8(ColorRGB8),
}

impl Color {
    pub fn format(&self) -> ColorFormat {
        match self {
            Color::RGB8(_) => ColorFormat::RGB8,
        }
    }

    pub fn rgb8(r: u8, g: u8, b: u8) -> Self {
        Color::RGB8(ColorRGB8::new(r, g, b))
    }
}
