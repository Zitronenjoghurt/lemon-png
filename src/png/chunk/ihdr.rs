use crate::png::types::color_type::ColorType;
use crate::png::types::compression_method::CompressionMethod;
use crate::png::types::filter_method::FilterMethod;
use crate::png::types::interlace_method::InterlaceMethod;

pub const IHDR_CHUNK_ID: u32 = 0x49_48_44_52;
pub const IHDR_DATA_LENGTH: usize = 13;

#[derive(Debug)]
pub struct IHDRChunk {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: ColorType,
    pub compression_method: CompressionMethod,
    pub filter_method: FilterMethod,
    pub interlace_method: InterlaceMethod,
}

#[derive(Debug)]
pub struct RawIHDRChunk {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
}
