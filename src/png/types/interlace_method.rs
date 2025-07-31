use crate::png::invalid_chunk::InvalidChunk;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum InterlaceMethod {
    None = 0,
    Adam7 = 1,
}

impl TryFrom<u8> for InterlaceMethod {
    type Error = InvalidChunk;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Adam7),
            _ => Err(InvalidChunk::InvalidInterlaceMethod { value }),
        }
    }
}
