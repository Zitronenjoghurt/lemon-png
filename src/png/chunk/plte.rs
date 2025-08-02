use crate::color::rgb::ColorRGB8;

#[derive(Debug)]
/// https://www.w3.org/TR/png-3/#11PLTE
pub struct PLTEChunk {
    pub colors: Vec<ColorRGB8>,
}

#[derive(Debug)]
pub struct RawPLTEChunk {
    pub colors: Vec<u8>,
}
