use crate::error::{PngError, PngResult};
use crate::png::chunk::ihdr::{IHDRChunk, RawIHDRChunk};
use crate::png::chunk::ChunkType;
use crate::png::types::color_type::ColorType;
use crate::png::types::compression_method::CompressionMethod;
use crate::png::types::filter_method::FilterMethod;
use crate::png::types::interlace_method::InterlaceMethod;

pub fn validate_ihdr(raw: RawIHDRChunk, offset: usize) -> PngResult<IHDRChunk> {
    let color_type =
        ColorType::try_from(raw.color_type).map_err(|invalid_chunk| PngError::InvalidChunk {
            chunk_type: ChunkType::ImageHeader,
            offset,
            kind: invalid_chunk,
        })?;

    let compression_method =
        CompressionMethod::try_from(raw.compression_method).map_err(|invalid_chunk| {
            PngError::InvalidChunk {
                chunk_type: ChunkType::ImageHeader,
                offset,
                kind: invalid_chunk,
            }
        })?;

    let filter_method = FilterMethod::try_from(raw.filter_method).map_err(|invalid_chunk| {
        PngError::InvalidChunk {
            chunk_type: ChunkType::ImageHeader,
            offset,
            kind: invalid_chunk,
        }
    })?;

    let interlace_method =
        InterlaceMethod::try_from(raw.interlace_method).map_err(|invalid_chunk| {
            PngError::InvalidChunk {
                chunk_type: ChunkType::ImageHeader,
                offset,
                kind: invalid_chunk,
            }
        })?;

    Ok(IHDRChunk {
        width: raw.width,
        height: raw.height,
        bit_depth: raw.bit_depth,
        color_type,
        compression_method,
        filter_method,
        interlace_method,
    })
}
