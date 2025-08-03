use crate::error::PngError;
use crate::png::chunk::idat::IDATChunk;
use crate::png::chunk::ihdr::IHDRChunk;
use crate::png::chunk::plte::PLTEChunk;
use std::fmt::Display;

pub mod idat;
pub mod ihdr;
pub mod plte;

#[derive(Debug)]
pub enum Chunk {
    ImageData(IDATChunk),
    ImageEnd,
    ImageHeader(IHDRChunk),
    Palette(PLTEChunk),
}

impl Chunk {
    pub fn get_type(&self) -> ChunkType {
        match self {
            Chunk::ImageData(_) => ChunkType::ImageData,
            Chunk::ImageEnd => ChunkType::ImageEnd,
            Chunk::ImageHeader(_) => ChunkType::ImageHeader,
            Chunk::Palette(_) => ChunkType::Palette,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ChunkType {
    ImageData,
    ImageEnd,
    ImageHeader,
    Palette,
}

impl TryFrom<u32> for ChunkType {
    type Error = PngError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x49_44_41_54 => Ok(Self::ImageData),
            0x49_45_4E_44 => Ok(Self::ImageEnd),
            0x49_48_44_52 => Ok(Self::ImageHeader),
            0x50_4C_54_45 => Ok(Self::Palette),
            _ => Err(PngError::UnknownChunkType(value)),
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkType::ImageData => write!(f, "IDAT"),
            ChunkType::ImageEnd => write!(f, "IEND"),
            ChunkType::ImageHeader => write!(f, "IHDR"),
            ChunkType::Palette => write!(f, "PLTE"),
        }
    }
}
