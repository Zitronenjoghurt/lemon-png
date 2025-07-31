use crate::chunk::{Chunk, ChunkTrait, CHUNK_METADATA_LENGTH};
use crate::error::{PngError, PngResult};
use crate::utils::bytestream::{get_u32_be, get_u64_be};
use std::path::Path;

pub const PNG_SIGNATURE: u64 = 0x89_50_4E_47_0D_0A_1A_0A;

#[derive(Debug)]
pub struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    pub fn load_from_path(path: &Path) -> PngResult<Self> {
        let bytes = std::fs::read(path)?;
        Self::decode(&bytes)
    }

    pub fn decode(bytes: &[u8]) -> PngResult<Self> {
        let signature = get_u64_be(bytes, 0).ok_or(PngError::MissingSignature)?;
        if signature != PNG_SIGNATURE {
            return Err(PngError::MissingSignature);
        }

        let mut offset = 8;
        let mut chunks = Vec::new();
        while offset < bytes.len() {
            let field_chunk_length =
                get_u32_be(bytes, offset).ok_or(PngError::UnidentifiableChunk)? as usize;
            let chunk_size = field_chunk_length + CHUNK_METADATA_LENGTH;

            if let Ok(chunk) = Chunk::decode(&bytes[offset..offset + chunk_size]) {
                chunks.push(chunk);
            }

            offset += chunk_size;
        }

        Ok(Png { chunks })
    }
}
