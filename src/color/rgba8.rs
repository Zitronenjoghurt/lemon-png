use crate::color::rgb8::ColorRGB8;
use crate::color::ColorTrait;

#[derive(Debug, Default, Clone, Copy)]
pub struct ColorRGBA8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRGBA8 {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn as_argb_u32(&self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

impl ColorTrait for ColorRGBA8 {
    fn as_rgb8(&self) -> ColorRGB8 {
        ColorRGB8::new(self.r, self.g, self.b)
    }

    fn as_rgba8(&self) -> ColorRGBA8 {
        *self
    }
}
