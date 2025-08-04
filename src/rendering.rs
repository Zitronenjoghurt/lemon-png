use crate::codec::compression::Compressor;
use crate::error::PngResult;
use crate::png::Png;
use crate::rendering::filtering::Filterer;

mod filtering;

#[derive(Debug, Default)]
pub struct PngRenderer;

impl PngRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, png: &Png) -> PngResult<Vec<u8>> {
        let mut decompressed_data = Compressor::new().decompress(&png.compressed_data)?;
        Filterer::new().unfilter(&mut decompressed_data, png.width);

        todo!()
    }
}
