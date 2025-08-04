use crate::png::types::palette::Palette;

#[derive(Debug)]
/// https://www.w3.org/TR/png-3/#11PLTE
pub struct PLTEChunk {
    pub colors: Palette,
}
