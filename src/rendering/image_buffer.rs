use crate::color::{Color, ColorBufferTrait};

#[derive(Debug, Default)]
pub struct ImageBuffer {
    pub height: u32,
    pub width: u32,
    pub colors: Vec<Color>,
}

impl ImageBuffer {
    pub fn get_argb_buffer(&self) -> Vec<u32> {
        self.colors
            .as_rgba8()
            .iter()
            .map(|color| color.as_argb_u32())
            .collect()
    }
}
