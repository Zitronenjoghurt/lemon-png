use crate::error::{PngError, PngResult};
use crate::png::chunk::{Chunk, ChunkType};
use crate::png::invalid_chunk::InvalidChunk;
use crate::png::raw_chunk::RawChunk;
use crate::validate::chunk::ihdr::validate_ihdr;

mod ihdr;

#[derive(Debug, Default)]
pub struct ChunkDecoder;

impl ChunkDecoder {
    pub fn decode(&self, raw: RawChunk) -> PngResult<Chunk> {
        let chunk_type = ChunkType::try_from(raw.chunk_type)?;

        let offset = raw.offset;
        let chunk = match chunk_type {
            ChunkType::ImageHeader => {
                Chunk::ImageHeader(validate_ihdr(ihdr::decode(raw)?, offset)?)
            }
            ChunkType::ImageEnd => Chunk::ImageEnd,
        };

        Ok(chunk)
    }
}

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
