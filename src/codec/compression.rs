use crate::error::PngResult;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use std::io::{Read, Write};

#[derive(Debug, Default)]
pub struct Compressor;

impl Compressor {
    pub fn new() -> Self {
        Self
    }

    pub fn compress(&self, data: &[u8]) -> PngResult<Vec<u8>> {
        let mut compressor = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        compressor.write_all(data)?;
        Ok(compressor.finish()?)
    }

    pub fn decompress(&self, data: &[u8]) -> PngResult<Vec<u8>> {
        let mut decompressor = ZlibDecoder::new(data);
        let mut decompressed = Vec::new();
        decompressor.read_to_end(&mut decompressed)?;
        Ok(decompressed)
    }
}
