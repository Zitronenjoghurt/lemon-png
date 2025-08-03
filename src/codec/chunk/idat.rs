use crate::codec::chunk::context::ChunkContext;
use crate::codec::raw_chunk::RawChunk;
use crate::error::PngResult;
use crate::png::chunk::idat::IDATChunk;
use crate::png::chunk::ChunkType;

#[derive(Debug)]
pub struct ParsedIDATChunk {
    pub compressed: Vec<u8>,
}

pub fn parse(raw: RawChunk) -> PngResult<ParsedIDATChunk> {
    Ok(ParsedIDATChunk {
        compressed: raw.data,
    })
}

pub fn validate(raw: ParsedIDATChunk, context: &ChunkContext) -> PngResult<IDATChunk> {
    let offset = context.require_offset(ChunkType::ImageData)?;

    todo!()
}
