use crate::png::invalid_chunk::InvalidChunk;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ColorType {
    Greyscale = 0,
    Truecolor = 2,
    Indexed = 3,
    GreyscaleAlpha = 4,
    TruecolorAlpha = 6,
}

impl TryFrom<u8> for ColorType {
    type Error = InvalidChunk;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Greyscale),
            2 => Ok(Self::Truecolor),
            3 => Ok(Self::Indexed),
            4 => Ok(Self::GreyscaleAlpha),
            6 => Ok(Self::TruecolorAlpha),
            _ => Err(InvalidChunk::InvalidColorType { value }),
        }
    }
}
