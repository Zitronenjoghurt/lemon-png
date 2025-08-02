use crate::codec::chunk::ihdr::ParsedIHDRChunk;
use crate::codec::chunk::plte::ParsedPLTEChunk;
use crate::png::chunk::ChunkType;

#[derive(Debug)]
pub enum ParsedChunk {
    ImageHeader(ParsedIHDRChunk),
    Palette(ParsedPLTEChunk),
    ImageEnd,
}

impl ParsedChunk {
    pub fn get_type(&self) -> ChunkType {
        match self {
            Self::ImageHeader(_) => ChunkType::ImageHeader,
            Self::Palette(_) => ChunkType::Palette,
            Self::ImageEnd => ChunkType::ImageEnd,
        }
    }
}
