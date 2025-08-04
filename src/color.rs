use crate::color::rgb8::ColorRGB8;
use crate::color::rgba8::ColorRGBA8;

pub mod rgb8;
mod rgba8;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ColorFormat {
    RGB8,
    RGBA8,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    RGB8(ColorRGB8),
    RGBA8(ColorRGBA8),
}

impl Color {
    pub fn format(&self) -> ColorFormat {
        match self {
            Color::RGB8(_) => ColorFormat::RGB8,
            Color::RGBA8(_) => ColorFormat::RGBA8,
        }
    }

    pub fn rgb8(r: u8, g: u8, b: u8) -> Self {
        Color::RGB8(ColorRGB8::new(r, g, b))
    }

    pub fn rgba8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color::RGBA8(ColorRGBA8::new(r, g, b, a))
    }
}

impl ColorTrait for Color {
    fn as_rgb8(&self) -> ColorRGB8 {
        match self {
            Self::RGB8(color) => *color,
            Self::RGBA8(color) => color.as_rgb8(),
        }
    }

    fn as_rgba8(&self) -> ColorRGBA8 {
        match self {
            Self::RGB8(color) => color.as_rgba8(),
            Self::RGBA8(color) => *color,
        }
    }
}

pub trait ColorTrait {
    fn as_rgb8(&self) -> ColorRGB8;
    fn as_rgba8(&self) -> ColorRGBA8;
}

pub trait ColorBufferTrait {
    fn as_rgb8(&self) -> Vec<ColorRGB8>;
    fn as_rgba8(&self) -> Vec<ColorRGBA8>;
}

impl<C> ColorBufferTrait for Vec<C>
where
    C: ColorTrait,
{
    fn as_rgb8(&self) -> Vec<ColorRGB8> {
        self.iter().map(|c| c.as_rgb8()).collect()
    }

    fn as_rgba8(&self) -> Vec<ColorRGBA8> {
        self.iter().map(|c| c.as_rgba8()).collect()
    }
}
