use crate::codec::chunk::{ihdr, plte};
use crate::codec::parsed_chunk::ParsedChunk;
use crate::codec::raw_chunk::RawChunk;
use crate::codec::validate::chunk::ChunkValidator;
use crate::error::{PngError, PngResult};
use crate::png::chunk::{Chunk, ChunkType};
use crate::png::invalid_chunk::InvalidChunk;

#[derive(Debug, Default)]
pub struct ChunkDecoder;

impl ChunkDecoder {
    pub fn decode(&self, raw: RawChunk, validator: &ChunkValidator) -> PngResult<Chunk> {
        let chunk_type = ChunkType::try_from(raw.chunk_type)?;
        let offset = raw.offset;
        if !raw.validate_crc() {
            return Err(PngError::invalid_chunk(
                chunk_type,
                offset,
                InvalidChunk::CRC,
            ));
        }

        let parsed_chunk = self.parse(raw)?;
        let chunk = validator.validate(parsed_chunk, offset)?;
        Ok(chunk)
    }

    pub fn parse(&self, raw: RawChunk) -> PngResult<ParsedChunk> {
        let chunk_type = ChunkType::try_from(raw.chunk_type)?;

        let parsed_chunk = match chunk_type {
            ChunkType::ImageHeader => ParsedChunk::ImageHeader(ihdr::decode(raw)?),
            ChunkType::Palette => ParsedChunk::Palette(plte::decode(raw)?),
            ChunkType::ImageEnd => ParsedChunk::ImageEnd,
        };

        Ok(parsed_chunk)
    }
}
