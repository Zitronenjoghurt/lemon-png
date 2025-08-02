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
