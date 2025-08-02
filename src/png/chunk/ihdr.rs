use crate::png::types::color_type::ColorType;
use crate::png::types::compression_method::CompressionMethod;
use crate::png::types::filter_method::FilterMethod;
use crate::png::types::image_type::ImageType;
use crate::png::types::interlace_method::InterlaceMethod;

#[derive(Debug)]
/// https://www.w3.org/TR/png-3/#11IHDR
pub struct IHDRChunk {
    pub width: u32,
    pub height: u32,
    pub image_type: ImageType,
    pub compression_method: CompressionMethod,
    pub filter_method: FilterMethod,
    pub interlace_method: InterlaceMethod,
}

impl IHDRChunk {
    pub fn bit_depth(&self) -> u8 {
        self.image_type.bit_depth()
    }

    pub fn color_type(&self) -> ColorType {
        self.image_type.color_type()
    }
}
