use crate::png::chunk::idat::IDATChunk;
use crate::png::chunk::ihdr::IHDRChunk;
use crate::png::chunk::plte::PLTEChunk;
use crate::png::chunk::Chunk;
use crate::png::types::compression_method::CompressionMethod;
use crate::png::types::filter_method::FilterMethod;
use crate::png::types::image_type::ImageType;
use crate::png::types::interlace_method::InterlaceMethod;
use crate::png::types::palette::Palette;
use crate::png::Png;

#[derive(Debug, Default)]
pub struct PngBuilder {
    png: Png,
}

impl PngBuilder {
    pub fn palette(mut self, palette: Palette) -> Self {
        self.png.color_palette = palette;
        self
    }

    pub fn compressed_data(mut self, compressed_data: Vec<u8>) -> Self {
        self.png.compressed_data.extend(compressed_data);
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

    pub fn compression_method(mut self, compression_method: CompressionMethod) -> Self {
        self.png.compression_method = compression_method;
        self
    }

    pub fn filter_method(mut self, filter_method: FilterMethod) -> Self {
        self.png.filter_method = filter_method;
        self
    }

    pub fn interlace_method(mut self, interlace_method: InterlaceMethod) -> Self {
        self.png.interlace_method = interlace_method;
        self
    }

    pub fn chunks(self, chunks: impl IntoIterator<Item = Chunk>) -> Self {
        chunks
            .into_iter()
            .fold(self, |builder, chunk| builder.chunk(chunk))
    }

    pub fn chunk(self, chunk: Chunk) -> Self {
        match chunk {
            Chunk::ImageHeader(header) => self.header_chunk(header),
            Chunk::Palette(palette) => self.palette_chunk(palette),
            Chunk::ImageData(data) => self.data_chunk(data),
            _ => self,
        }
    }

    pub fn data_chunk(self, chunk: IDATChunk) -> Self {
        self.compressed_data(chunk.compressed)
    }

    pub fn header_chunk(self, chunk: IHDRChunk) -> Self {
        self.width(chunk.width)
            .height(chunk.height)
            .image_type(chunk.image_type)
            .compression_method(chunk.compression_method)
            .filter_method(chunk.filter_method)
            .interlace_method(chunk.interlace_method)
    }

    pub fn palette_chunk(self, chunk: PLTEChunk) -> Self {
        self.palette(chunk.colors)
    }

    pub fn build(self) -> Png {
        self.png
    }
}
