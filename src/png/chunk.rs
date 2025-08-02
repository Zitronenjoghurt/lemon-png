use crate::error::PngError;
use crate::png::chunk::ihdr::IHDRChunk;
use crate::png::chunk::plte::PLTEChunk;
use std::fmt::Display;

pub mod ihdr;
pub mod plte;

#[derive(Debug)]
pub enum Chunk {
    ImageHeader(IHDRChunk),
    Palette(PLTEChunk),
    ImageEnd,
}

impl Chunk {
    pub fn get_type(&self) -> ChunkType {
        match self {
            Chunk::ImageHeader(_) => ChunkType::ImageHeader,
            Chunk::Palette(_) => ChunkType::Palette,
            Chunk::ImageEnd => ChunkType::ImageEnd,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ChunkType {
    ImageHeader,
    Palette,
    ImageEnd,
}

impl TryFrom<u32> for ChunkType {
    type Error = PngError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x49_48_44_52 => Ok(Self::ImageHeader),
            0x50_4C_54_45 => Ok(Self::Palette),
            0x49_45_4E_44 => Ok(Self::ImageEnd),
            _ => Err(PngError::UnknownChunkType(value)),
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkType::ImageHeader => write!(f, "IHDR"),
            ChunkType::Palette => write!(f, "PLTE"),
            ChunkType::ImageEnd => write!(f, "IEND"),
        }
    }
}
