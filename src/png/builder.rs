use crate::color::palette::ColorPalette;
use crate::png::chunk::ihdr::IHDRChunk;
use crate::png::chunk::plte::PLTEChunk;
use crate::png::types::image_type::ImageType;
use crate::png::types::interlace_method::InterlaceMethod;
use crate::png::Png;

#[derive(Debug, Default)]
pub struct PngBuilder {
    png: Png,
}

impl PngBuilder {
    pub fn color_palette(mut self, color_palette: ColorPalette) -> Self {
        self.png.color_palette = color_palette;
        self
    }

    pub fn compressed_data(mut self, compressed_data: Vec<u8>) -> Self {
        self.png.compressed_data = compressed_data;
        self
    }

    pub fn dimensions(mut self, width: u32, height: u32) -> Self {
        self.png.width = width;
        self.png.height = height;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.png.height = height;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.png.width = width;
        self
    }

    pub fn image_type(mut self, image_type: ImageType) -> Self {
        self.png.image_type = image_type;
        self
    }

    pub fn interlace_method(mut self, interlace_method: InterlaceMethod) -> Self {
        self.png.interlace_method = interlace_method;
        self
    }

    pub fn header_chunk(mut self, chunk: &IHDRChunk) -> Self {
        self.png.width = chunk.width;
        self.png.height = chunk.height;
        self.png.image_type = chunk.image_type;
        self.png.compression_method = chunk.compression_method;
        self.png.filter_method = chunk.filter_method;
        self.png.interlace_method = chunk.interlace_method;
        self
    }

    pub fn palette_chunk(mut self, chunk: &PLTEChunk) -> Self {
        self.png.color_palette = chunk.colors.clone();
        self
    }

    pub fn build(self) -> Png {
        self.png
    }
}
