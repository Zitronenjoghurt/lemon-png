use crate::codec::chunk::ihdr::ParsedIHDRChunk;
use crate::codec::chunk::plte::ParsedPLTEChunk;
use crate::png::chunk::idat::IDATChunk;
use crate::png::chunk::ChunkType;

#[derive(Debug)]
pub enum ParsedChunk {
    ImageData(IDATChunk),
    ImageEnd,
    ImageHeader(ParsedIHDRChunk),
    Palette(ParsedPLTEChunk),
}

impl ParsedChunk {
    pub fn get_type(&self) -> ChunkType {
        match self {
            Self::ImageData(_) => ChunkType::ImageData,
            Self::ImageEnd => ChunkType::ImageEnd,
            Self::ImageHeader(_) => ChunkType::ImageHeader,
            Self::Palette(_) => ChunkType::Palette,
        }
    }
}
