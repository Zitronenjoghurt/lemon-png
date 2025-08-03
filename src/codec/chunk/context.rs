use crate::error::{PngError, PngResult};
use crate::png::chunk::ChunkType;
use crate::png::types::color_type::ColorType;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ChunkContextParameter {
    Width,
    Height,
    ColorType,
    BitDepth,
    Offset,
}

#[derive(Debug, Default)]
pub struct ChunkContext {
    width: Option<u32>,
    height: Option<u32>,
    color_type: Option<ColorType>,
    bit_depth: Option<u8>,
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

    pub fn get_color_type(&self) -> Option<ColorType> {
        self.color_type
    }

    pub fn set_color_type(&mut self, color_type: ColorType) {
        self.color_type = Some(color_type);
    }

    pub fn require_color_type(&self, chunk_type: ChunkType) -> PngResult<ColorType> {
        self.get_color_type().ok_or_else(|| {
            PngError::missing_context(
                chunk_type,
                self.get_offset(),
                ChunkContextParameter::ColorType,
            )
        })
    }

    pub fn get_bit_depth(&self) -> Option<u8> {
        self.bit_depth
    }

    pub fn set_bit_depth(&mut self, bit_depth: u8) {
        self.bit_depth = Some(bit_depth);
    }

    pub fn require_bit_depth(&self, chunk_type: ChunkType) -> PngResult<u8> {
        self.get_bit_depth().ok_or_else(|| {
            PngError::missing_context(
                chunk_type,
                self.get_offset(),
                ChunkContextParameter::BitDepth,
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
