use crate::codec::compression::Compressor;
use crate::color::Color;
use crate::error::{PngError, PngResult};
use crate::png::types::image_type::ImageType;
use crate::png::Png;
use crate::rendering::filtering::Filterer;
use crate::rendering::image_buffer::ImageBuffer;

mod filtering;
mod image_buffer;

#[derive(Debug, Default)]
pub struct PngRenderer;

impl PngRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, png: &Png) -> PngResult<ImageBuffer> {
        let decompressed_data = Compressor::new().decompress(&png.compressed_data)?;
        let unfiltered_data = Filterer::new().unfilter(
            &decompressed_data,
            png.height,
            png.scanline_width(),
            png.bytes_per_pixel(),
        );

        drop(decompressed_data);

        let total_bytes = png.image_type.bits_per_pixel() as u32 * png.height * png.width / 8;
        if unfiltered_data.len() != total_bytes as usize {
            return Err(PngError::InvalidImageData);
        }

        let colors = match png.image_type {
            ImageType::RGB24 => self.convert_rgb24(unfiltered_data),
            ImageType::RGBA32 => self.convert_rgba32(unfiltered_data),
            _ => unimplemented!(),
        };

        Ok(ImageBuffer {
            height: png.height,
            width: png.width,
            colors,
        })
    }

    fn convert_rgb24(&self, unfiltered_data: Vec<u8>) -> Vec<Color> {
        unfiltered_data
            .chunks(3)
            .map(|chunk| Color::rgb8(chunk[0], chunk[1], chunk[2]))
            .collect()
    }

    fn convert_rgba32(&self, unfiltered_data: Vec<u8>) -> Vec<Color> {
        unfiltered_data
            .chunks(4)
            .map(|chunk| Color::rgba8(chunk[0], chunk[1], chunk[2], chunk[3]))
            .collect()
    }
}
