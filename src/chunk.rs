use crate::chunk::ihdr::{IHDRChunk, IHDRChunkError, IHDR_CHUNK_ID};
use crate::error::{PngError, PngResult};
use crate::utils::bytestream::get_u32_be;

pub mod ihdr;

pub const CHUNK_METADATA_LENGTH: usize = 12;

#[derive(Debug, Copy, Clone)]
pub enum ChunkType {
    ImageHeader,
}

#[derive(Debug)]
pub enum Chunk {
    ImageHeader(IHDRChunk),
}

impl Chunk {
    pub fn get_type(&self) -> ChunkType {
        match self {
            Chunk::ImageHeader(_) => ChunkType::ImageHeader,
        }
    }
}

impl ChunkTrait for Chunk {
    fn encode(&self) -> Vec<u8> {
        match self {
            Chunk::ImageHeader(ihdr) => ihdr.encode(),
        }
    }

    fn decode(bytes: &[u8]) -> PngResult<Self> {
        let chunk_identifier = get_u32_be(bytes, 4).ok_or(PngError::UnidentifiableChunk)?;

        match chunk_identifier {
            IHDR_CHUNK_ID => Ok(Chunk::ImageHeader(IHDRChunk::decode(bytes)?)),
            _ => Err(PngError::invalid_chunk_type(chunk_identifier)),
        }
    }
}

pub trait ChunkTrait: Sized {
    fn encode(&self) -> Vec<u8>;
    fn decode(bytes: &[u8]) -> PngResult<Self>;
}

#[derive(Debug)]
pub enum ChunkError {
    InvalidCRC,
    InvalidIHDRChunk(IHDRChunkError),
}

impl std::fmt::Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCRC => write!(f, "Invalid CRC"),
            Self::InvalidIHDRChunk(e) => write!(f, "Invalid IHDR chunk: {}", e),
        }
    }
}

impl std::error::Error for ChunkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidIHDRChunk(e) => Some(e),
            _ => None,
        }
    }
}

impl From<IHDRChunkError> for ChunkError {
    fn from(e: IHDRChunkError) -> Self {
        Self::InvalidIHDRChunk(e)
    }
}
