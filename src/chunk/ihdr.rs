use crate::chunk::{ChunkTrait, CHUNK_METADATA_LENGTH};
use crate::error::PngResult;
use crate::utils::bytestream::{get_u32_be, get_u8};

pub const IHDR_CHUNK_ID: u32 = 0x49_48_44_52;
pub const IHDR_DATA_LENGTH: usize = 13;
pub const IHDR_CHUNK_LENGTH: usize = IHDR_DATA_LENGTH + CHUNK_METADATA_LENGTH;

#[derive(Debug)]
pub struct IHDRChunk {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
}

impl ChunkTrait for IHDRChunk {
    fn encode(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(IHDR_CHUNK_LENGTH);
        data.extend_from_slice(&(IHDR_DATA_LENGTH as u32).to_be_bytes());
        data.extend_from_slice(&IHDR_CHUNK_ID.to_be_bytes());
        data.extend_from_slice(&(self.width).to_be_bytes());
        data.extend_from_slice(&(self.height).to_be_bytes());
        data.push(self.bit_depth);
        data.push(self.color_type);
        data.push(self.compression_method);
        data.push(self.filter_method);
        data.push(self.interlace_method);
        data
    }

    fn decode(bytes: &[u8]) -> PngResult<Self> {
        if bytes.len() != IHDR_CHUNK_LENGTH {
            return Err(IHDRChunkError::InvalidChunkLength.into());
        }

        let width = get_u32_be(bytes, 8).unwrap();
        let height = get_u32_be(bytes, 12).unwrap();
        let bit_depth = get_u8(bytes, 16).unwrap();
        let color_type = get_u8(bytes, 17).unwrap();
        let compression_method = get_u8(bytes, 18).unwrap();
        let filter_method = get_u8(bytes, 19).unwrap();
        let interlace_method = get_u8(bytes, 20).unwrap();

        Ok(Self {
            width,
            height,
            bit_depth,
            color_type,
            compression_method,
            filter_method,
            interlace_method,
        })
    }
}

#[derive(Debug)]
pub enum IHDRChunkError {
    InvalidChunkLength,
}

impl std::fmt::Display for IHDRChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IHDRChunkError::InvalidChunkLength => write!(f, "Invalid chunk length"),
        }
    }
}

impl std::error::Error for IHDRChunkError {}
