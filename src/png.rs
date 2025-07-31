use chunk::Chunk;

pub mod chunk;
pub mod invalid_chunk;
pub mod raw_chunk;
pub mod types;

pub const PNG_SIGNATURE: u64 = 0x89_50_4E_47_0D_0A_1A_0A;

#[derive(Debug)]
pub struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    pub fn new(chunks: Vec<Chunk>) -> Self {
        Self { chunks }
    }
}
