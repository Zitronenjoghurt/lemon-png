use crate::png::invalid_chunk::InvalidChunk;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum CompressionMethod {
    Deflate = 0,
}

impl TryFrom<u8> for CompressionMethod {
    type Error = InvalidChunk;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Deflate),
            _ => Err(InvalidChunk::InvalidCompressionMethod { value }),
        }
    }
}
