use crate::codec::raw_chunk::RawChunk;
use crate::error::{PngError, PngResult};
use crate::png::chunk::ChunkType;
use crate::png::invalid_chunk::InvalidChunk;
pub mod context;
pub mod idat;
pub mod ihdr;
pub mod plte;

pub fn validate_raw_chunk_data_length(
    raw: &RawChunk,
    chunk_type: ChunkType,
    expected: usize,
) -> PngResult<()> {
    if raw.data.len() != expected {
        Err(PngError::InvalidChunk {
            chunk_type,
            offset: raw.offset,
            kind: InvalidChunk::Length {
                expected,
                actual: raw.data.len(),
            },
        })
    } else {
        Ok(())
    }
}
