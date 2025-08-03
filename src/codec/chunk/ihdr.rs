use crate::codec::chunk::context::ChunkContext;
use crate::codec::chunk::validate_raw_chunk_data_length;
use crate::codec::raw_chunk::RawChunk;
use crate::error::{PngError, PngResult};
use crate::png::chunk::ihdr::IHDRChunk;
use crate::png::chunk::ChunkType;
use crate::png::types::color_type::ColorType;
use crate::png::types::compression_method::CompressionMethod;
use crate::png::types::filter_method::FilterMethod;
use crate::png::types::image_type::ImageType;
use crate::png::types::interlace_method::InterlaceMethod;
use crate::utils::bytestream::read_u32_be;

#[derive(Debug)]
pub struct ParsedIHDRChunk {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
}

pub fn parse(raw: RawChunk) -> PngResult<ParsedIHDRChunk> {
    validate_raw_chunk_data_length(&raw, ChunkType::ImageHeader, 13)?;

    let width = read_u32_be(&raw.data, 0).unwrap();
    let height = read_u32_be(&raw.data, 4).unwrap();
    let bit_depth = raw.data[8];
    let color_type = raw.data[9];
    let compression_method = raw.data[10];
    let filter_method = raw.data[11];
    let interlace_method = raw.data[12];

    Ok(ParsedIHDRChunk {
        width,
        height,
        bit_depth,
        color_type,
        compression_method,
        filter_method,
        interlace_method,
    })
}

pub fn validate(raw: ParsedIHDRChunk, context: &ChunkContext) -> PngResult<IHDRChunk> {
    let offset = context.require_offset(ChunkType::ImageHeader)?;

    let color_type = ColorType::try_from(raw.color_type).map_err(|invalid_chunk| {
        PngError::invalid_chunk(ChunkType::ImageHeader, offset, invalid_chunk)
    })?;

    let compression_method =
        CompressionMethod::try_from(raw.compression_method).map_err(|invalid_chunk| {
            PngError::invalid_chunk(ChunkType::ImageHeader, offset, invalid_chunk)
        })?;

    let filter_method = FilterMethod::try_from(raw.filter_method).map_err(|invalid_chunk| {
        PngError::invalid_chunk(ChunkType::ImageHeader, offset, invalid_chunk)
    })?;

    let interlace_method =
        InterlaceMethod::try_from(raw.interlace_method).map_err(|invalid_chunk| {
            PngError::invalid_chunk(ChunkType::ImageHeader, offset, invalid_chunk)
        })?;

    let image_type = ImageType::try_from((color_type, raw.bit_depth)).map_err(|invalid_chunk| {
        PngError::invalid_chunk(ChunkType::ImageHeader, offset, invalid_chunk)
    })?;

    Ok(IHDRChunk {
        width: raw.width,
        height: raw.height,
        image_type,
        compression_method,
        filter_method,
        interlace_method,
    })
}
