use crate::codec::chunk::{ihdr, plte};
use crate::codec::parsed_chunk::ParsedChunk;
use crate::codec::raw_chunk::RawChunk;
use crate::codec::validate::chunk::ChunkValidator;
use crate::error::{PngError, PngResult};
use crate::png::chunk::idat::IDATChunk;
use crate::png::chunk::{Chunk, ChunkType, Chunks};
use crate::png::invalid_chunk::InvalidChunk;

#[derive(Debug, Default)]
pub struct ChunkDecoder;

impl ChunkDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn decode(&self, raw: RawChunk, validator: &impl ChunkValidator) -> PngResult<Chunk> {
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
        let chunk = validator.validate(parsed_chunk)?;
        Ok(chunk)
    }

    pub fn decode_all(
        &mut self,
        raw_chunks: impl Iterator<Item = RawChunk>,
        validator: &impl ChunkValidator,
    ) -> PngResult<Chunks> {
        let mut chunks = Vec::new();
        for raw_chunk in raw_chunks {
            let chunk = self.decode(raw_chunk, validator)?;
            chunks.push(chunk);
        }
        Ok(Chunks::new(chunks))
    }

    pub fn decode_all_skip_errors(
        &mut self,
        raw_chunks: impl Iterator<Item = RawChunk>,
        validator: &impl ChunkValidator,
    ) -> Chunks {
        let mut chunks = Vec::new();
        for raw_chunk in raw_chunks {
            if let Ok(chunk) = self.decode(raw_chunk, validator) {
                chunks.push(chunk);
            }
        }
        Chunks::new(chunks)
    }

    pub fn parse(&self, raw: RawChunk) -> PngResult<ParsedChunk> {
        let chunk_type = ChunkType::try_from(raw.chunk_type)?;
        let parsed_chunk = match chunk_type {
            ChunkType::ImageData => ParsedChunk::ImageData(IDATChunk {
                compressed: raw.data,
            }),
            ChunkType::ImageEnd => ParsedChunk::ImageEnd,
            ChunkType::ImageHeader => ParsedChunk::ImageHeader(ihdr::parse(raw)?),
            ChunkType::Palette => ParsedChunk::Palette(plte::parse(raw)?),
        };

        Ok(parsed_chunk)
    }
}
