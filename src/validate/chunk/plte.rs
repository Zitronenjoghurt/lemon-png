use crate::color::rgb::ColorRGB8;
use crate::error::{PngError, PngResult};
use crate::png::chunk::plte::{PLTEChunk, RawPLTEChunk};
use crate::png::chunk::ChunkType;
use crate::png::invalid_chunk::InvalidChunk;

pub fn validate_plte(raw: RawPLTEChunk, offset: usize) -> PngResult<PLTEChunk> {
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
