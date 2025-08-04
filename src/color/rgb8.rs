use crate::color::rgba8::ColorRGBA8;
use crate::color::ColorTrait;

#[derive(Debug, Default, Clone, Copy)]
pub struct ColorRGB8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRGB8 {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl ColorTrait for ColorRGB8 {
    fn as_rgb8(&self) -> ColorRGB8 {
        *self
    }

    fn as_rgba8(&self) -> ColorRGBA8 {
        ColorRGBA8::new(self.r, self.g, self.b, 255)
    }
}
