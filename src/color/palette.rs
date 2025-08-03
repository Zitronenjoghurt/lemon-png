use crate::color::Color;

#[derive(Debug, Default, Clone)]
pub struct ColorPalette(Vec<Color>);

impl ColorPalette {
    pub fn new(colors: Vec<Color>) -> Self {
        Self(colors)
    }
}
