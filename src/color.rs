use crate::color::rgb::ColorRGB8;

pub mod rgb;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    RGB8(ColorRGB8),
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ColorFormat {
    RGB8,
}
