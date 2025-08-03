use crate::codec::raw_chunk::RawChunk;
use crate::color::palette::ColorPalette;
use crate::color::Color;
use crate::error::{PngError, PngResult};
use crate::png::chunk::plte::PLTEChunk;
use crate::png::chunk::ChunkType;
use crate::png::invalid_chunk::InvalidChunk;

#[derive(Debug)]
pub struct ParsedPLTEChunk {
    pub offset: usize,
    pub colors: Vec<u8>,
}

pub fn parse(raw: RawChunk) -> PngResult<ParsedPLTEChunk> {
    Ok(ParsedPLTEChunk {
        offset: raw.offset,
        colors: raw.data,
    })
}

pub fn validate(raw: ParsedPLTEChunk) -> PngResult<PLTEChunk> {
    if raw.colors.len() % 3 != 0 {
        return Err(PngError::invalid_chunk(
            ChunkType::Palette,
            raw.offset,
            InvalidChunk::Length {
                expected: raw.colors.len() / 3,
                actual: raw.colors.len(),
            },
        ));
    }

    let colors = raw
        .colors
        .chunks(3)
        .map(|c| Color::rgb8(c[0], c[1], c[2]))
        .collect();

    Ok(PLTEChunk {
        colors: ColorPalette::new(colors),
    })
}
