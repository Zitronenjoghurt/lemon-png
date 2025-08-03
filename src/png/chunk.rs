use crate::error::PngError;
use crate::png::chunk::idat::IDATChunk;
use crate::png::chunk::ihdr::IHDRChunk;
use crate::png::chunk::plte::PLTEChunk;
use std::fmt::Display;

pub mod idat;
pub mod ihdr;
pub mod plte;

#[derive(Debug)]
pub enum Chunk {
    ImageData(IDATChunk),
    ImageEnd,
    ImageHeader(IHDRChunk),
    Palette(PLTEChunk),
}

impl Chunk {
    pub fn get_type(&self) -> ChunkType {
        match self {
            Chunk::ImageData(_) => ChunkType::ImageData,
            Chunk::ImageEnd => ChunkType::ImageEnd,
            Chunk::ImageHeader(_) => ChunkType::ImageHeader,
            Chunk::Palette(_) => ChunkType::Palette,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ChunkType {
    ImageData,
    ImageEnd,
    ImageHeader,
    Palette,
}

impl TryFrom<u32> for ChunkType {
    type Error = PngError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x49_44_41_54 => Ok(Self::ImageData),
            0x49_45_4E_44 => Ok(Self::ImageEnd),
            0x49_48_44_52 => Ok(Self::ImageHeader),
            0x50_4C_54_45 => Ok(Self::Palette),
            _ => Err(PngError::UnknownChunkType(value)),
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkType::ImageData => write!(f, "IDAT"),
            ChunkType::ImageEnd => write!(f, "IEND"),
            ChunkType::ImageHeader => write!(f, "IHDR"),
            ChunkType::Palette => write!(f, "PLTE"),
        }
    }
}

pub struct Chunks(Vec<Chunk>);

impl Chunks {
    pub fn new(chunks: Vec<Chunk>) -> Self {
        Self(chunks)
    }

    pub fn push(&mut self, chunk: Chunk) {
        self.0.push(chunk);
    }

    pub fn get(&self, index: usize) -> Option<&Chunk> {
        self.0.get(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<Chunk> {
        self.0.iter()
    }

    pub fn iter_by_type(
        &self,
        chunk_type: ChunkType,
    ) -> impl Iterator<Item = (usize, &Chunk)> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter(move |(_, chunk)| chunk.get_type() == chunk_type)
    }

    pub fn get_one_by_type(&self, chunk_type: ChunkType) -> Option<&Chunk> {
        self.iter_by_type(chunk_type).next().map(|(_, chunk)| chunk)
    }

    pub fn get_one_by_type_with_index(&self, chunk_type: ChunkType) -> Option<(usize, &Chunk)> {
        self.iter_by_type(chunk_type).next()
    }

    pub fn is_type_unique(&self, chunk_type: ChunkType) -> bool {
        self.iter_by_type(chunk_type).count() == 1
    }

    pub fn is_type_present(&self, chunk_type: ChunkType) -> bool {
        self.iter_by_type(chunk_type).count() > 0
    }

    pub fn is_type_at_index(&self, chunk_type: ChunkType, index: usize) -> bool {
        self.get(index)
            .map(|chunk| chunk.get_type() == chunk_type)
            .unwrap_or(false)
    }

    pub fn is_type_at_index_and_unique(&self, chunk_type: ChunkType, index: usize) -> bool {
        self.is_type_unique(chunk_type) && self.is_type_at_index(chunk_type, index)
    }
}
