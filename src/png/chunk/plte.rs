use crate::color::palette::ColorPalette;

#[derive(Debug)]
/// https://www.w3.org/TR/png-3/#11PLTE
pub struct PLTEChunk {
    pub colors: ColorPalette,
}
