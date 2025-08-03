use crate::color::palette::ColorPalette;
use crate::png::builder::PngBuilder;
use crate::png::types::compression_method::CompressionMethod;
use crate::png::types::filter_method::FilterMethod;
use crate::png::types::image_type::ImageType;
use crate::png::types::interlace_method::InterlaceMethod;

pub mod builder;
pub mod chunk;
pub mod invalid_chunk;
pub mod types;

pub const PNG_SIGNATURE: u64 = 0x89_50_4E_47_0D_0A_1A_0A;

#[derive(Debug, Default)]
pub struct Png {
    pub height: u32,
    pub width: u32,
    pub image_type: ImageType,
    pub compression_method: CompressionMethod,
    pub filter_method: FilterMethod,
    pub interlace_method: InterlaceMethod,
    pub color_palette: ColorPalette,
    pub compressed_data: Vec<u8>,
}

impl Png {
    pub fn builder() -> PngBuilder {
        PngBuilder::default()
    }
}
