use crate::decode::chunk::validate_raw_chunk_data_length;
use crate::error::PngResult;
use crate::png::chunk::ihdr::RawIHDRChunk;
use crate::png::chunk::ChunkType;
use crate::png::raw_chunk::RawChunk;
use crate::utils::bytestream::read_u32_be;

pub fn decode(raw: RawChunk) -> PngResult<RawIHDRChunk> {
    validate_raw_chunk_data_length(&raw, ChunkType::ImageHeader, 13)?;

    let width = read_u32_be(&raw.data, 0).unwrap();
    let height = read_u32_be(&raw.data, 4).unwrap();
    let bit_depth = raw.data[8];
    let color_type = raw.data[9];
    let compression_method = raw.data[10];
    let filter_method = raw.data[11];
    let interlace_method = raw.data[12];

    Ok(RawIHDRChunk {
        width,
        height,
        bit_depth,
        color_type,
        compression_method,
        filter_method,
        interlace_method,
    })
}
