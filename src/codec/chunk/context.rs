use crate::error::{PngError, PngResult};
use crate::png::chunk::ChunkType;
use crate::png::types::image_type::ImageType;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ChunkContextParameter {
    Width,
    Height,
    ImageType,
    Offset,
}

#[derive(Debug, Default)]
pub struct ChunkContext {
    width: Option<u32>,
    height: Option<u32>,
    image_type: Option<ImageType>,
    offset: Option<usize>,
}

impl ChunkContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_width(&self) -> Option<u32> {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = Some(width);
    }

    pub fn require_width(&self, chunk_type: ChunkType) -> PngResult<u32> {
        self.get_width().ok_or_else(|| {
            PngError::missing_context(chunk_type, self.get_offset(), ChunkContextParameter::Width)
        })
    }

    pub fn get_height(&self) -> Option<u32> {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = Some(height);
    }

    pub fn require_height(&self, chunk_type: ChunkType) -> PngResult<u32> {
        self.get_height().ok_or_else(|| {
            PngError::missing_context(chunk_type, self.get_offset(), ChunkContextParameter::Height)
        })
    }

    pub fn get_image_type(&self) -> Option<ImageType> {
        self.image_type
    }

    pub fn set_image_type(&mut self, image_type: ImageType) {
        self.image_type = Some(image_type);
    }

    pub fn require_image_type(&self, chunk_type: ChunkType) -> PngResult<ImageType> {
        self.get_image_type().ok_or_else(|| {
            PngError::missing_context(
                chunk_type,
                self.get_offset(),
                ChunkContextParameter::ImageType,
            )
        })
    }

    pub fn get_offset(&self) -> Option<usize> {
        self.offset
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.offset = Some(offset);
    }

    pub fn require_offset(&self, chunk_type: ChunkType) -> PngResult<usize> {
        self.get_offset().ok_or_else(|| {
            PngError::missing_context(chunk_type, self.get_offset(), ChunkContextParameter::Offset)
        })
    }
}
