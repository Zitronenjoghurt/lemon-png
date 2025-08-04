use crate::color::rgb8::ColorRGB8;

#[derive(Debug, Default, Clone)]
pub struct Palette(Vec<ColorRGB8>);

impl Palette {
    pub fn new(colors: Vec<ColorRGB8>) -> Self {
        Self(colors)
    }
}
