use crate::png::invalid_chunk::InvalidChunk;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum FilterMethod {
    AdaptiveFiveTypes = 0,
}

impl TryFrom<u8> for FilterMethod {
    type Error = InvalidChunk;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AdaptiveFiveTypes),
            _ => Err(InvalidChunk::InvalidFilterMethod { value }),
        }
    }
}
