use crate::codec::chunk::context::ChunkContext;
use crate::codec::chunk::*;
use crate::codec::parsed_chunk::ParsedChunk;
use crate::error::PngResult;
use crate::png::chunk::Chunk;

#[derive(Debug, Default)]
pub struct ChunkValidator;

impl ChunkValidator {
    pub fn validate(&self, context: &ChunkContext, parsed_chunk: ParsedChunk) -> PngResult<Chunk> {
        let chunk = match parsed_chunk {
            ParsedChunk::ImageData(chunk) => Chunk::ImageData(idat::validate(chunk, context)?),
            ParsedChunk::ImageEnd => Chunk::ImageEnd,
            ParsedChunk::ImageHeader(chunk) => Chunk::ImageHeader(ihdr::validate(chunk, context)?),
            ParsedChunk::Palette(chunk) => Chunk::Palette(plte::validate(chunk, context)?),
        };
        Ok(chunk)
    }
}
