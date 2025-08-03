use crate::codec::chunk::context::ChunkContext;
use crate::codec::raw_chunk::RawChunk;
use crate::color::rgb::ColorRGB8;
use crate::error::{PngError, PngResult};
use crate::png::chunk::plte::PLTEChunk;
use crate::png::chunk::ChunkType;
use crate::png::invalid_chunk::InvalidChunk;

#[derive(Debug)]
pub struct ParsedPLTEChunk {
    pub colors: Vec<u8>,
}

pub fn parse(raw: RawChunk) -> PngResult<ParsedPLTEChunk> {
    Ok(ParsedPLTEChunk { colors: raw.data })
}

pub fn validate(raw: ParsedPLTEChunk, context: &ChunkContext) -> PngResult<PLTEChunk> {
    let offset = context.require_offset(ChunkType::Palette)?;

    if raw.colors.len() % 3 != 0 {
        return Err(PngError::invalid_chunk(
            ChunkType::Palette,
            offset,
            InvalidChunk::Length {
                expected: raw.colors.len() / 3,
                actual: raw.colors.len(),
            },
        ));
    }

    let colors = raw
        .colors
        .chunks(3)
        .map(|c| ColorRGB8::new(c[0], c[1], c[2]))
        .collect();

    Ok(PLTEChunk { colors })
}
