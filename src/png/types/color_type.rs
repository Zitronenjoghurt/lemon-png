use crate::png::invalid_chunk::InvalidChunk;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// https://www.w3.org/TR/png-3/#4Concepts.PNGImage
pub enum ColorType {
    /// Each pixel consists of a single grey sample, which represents overall luminance (on a scale from black to white).
    /// An optional alpha channel can be specified as a single grey sample: pixels of the image whose grey sample is identical
    /// to the grey sample of the alpha channel are fully transparent; others are fully opaque.
    /// If the alpha channel is not present, all pixels are fully opaque.
    Greyscale = 0,
    /// Each pixel consists of a triplet of samples: red, green, blue.
    /// An optional alpha channel can be specified as a single triplet of red, green, blue samples: pixels of the image whose red, green, blue samples are identical
    /// to the red, green, blue samples of the alpha channel are fully transparent;
    /// others are fully opaque. If the alpha channel is not present, all pixels are fully opaque.
    Truecolor = 2,
    /// Each pixel consists of an index into a palette (and into an associated table of alpha values, if present).
    Indexed = 3,
    /// Each pixel consists of two samples: a grey sample and an alpha sample.
    GreyscaleAlpha = 4,
    /// Each pixel consists of four samples: red, green, blue and alpha.
    TruecolorAlpha = 6,
}

impl std::fmt::Display for ColorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Greyscale => write!(f, "Greyscale"),
            Self::Truecolor => write!(f, "Truecolor"),
            Self::Indexed => write!(f, "Indexed"),
            Self::GreyscaleAlpha => write!(f, "GreyscaleAlpha"),
            Self::TruecolorAlpha => write!(f, "TruecolorAlpha"),
        }
    }
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
