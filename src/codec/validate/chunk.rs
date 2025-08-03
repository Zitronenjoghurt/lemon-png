use crate::codec::chunk::*;
use crate::codec::parsed_chunk::ParsedChunk;
use crate::error::PngResult;
use crate::png::chunk::Chunk;
use std::fmt::Debug;

pub trait ChunkValidator: Send + Sync + Sized + Debug {
    fn validate(&self, parsed_chunk: ParsedChunk) -> PngResult<Chunk>;
}

#[derive(Debug, Default)]
pub struct DefaultChunkValidator;

impl ChunkValidator for DefaultChunkValidator {
    fn validate(&self, parsed_chunk: ParsedChunk) -> PngResult<Chunk> {
        let chunk = match parsed_chunk {
            ParsedChunk::ImageData(chunk) => Chunk::ImageData(chunk),
            ParsedChunk::ImageEnd => Chunk::ImageEnd,
            ParsedChunk::ImageHeader(chunk) => Chunk::ImageHeader(ihdr::validate(chunk)?),
            ParsedChunk::Palette(chunk) => Chunk::Palette(plte::validate(chunk)?),
        };
        Ok(chunk)
    }
}
