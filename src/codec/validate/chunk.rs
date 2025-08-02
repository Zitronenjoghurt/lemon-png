use crate::codec::parsed_chunk::ParsedChunk;
use crate::error::PngResult;
use crate::png::chunk::Chunk;

mod ihdr;
mod plte;

pub struct ChunkValidator;

impl ChunkValidator {
    pub fn validate(&self, parsed_chunk: ParsedChunk, offset: usize) -> PngResult<Chunk> {
        let chunk = match parsed_chunk {
            ParsedChunk::ImageHeader(chunk) => Chunk::ImageHeader(ihdr::validate(chunk, offset)?),
            ParsedChunk::Palette(chunk) => Chunk::Palette(plte::validate(chunk, offset)?),
            ParsedChunk::ImageEnd => Chunk::ImageEnd,
        };
        Ok(chunk)
    }
}
