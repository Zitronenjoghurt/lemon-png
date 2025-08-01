use crate::png::invalid_chunk::InvalidChunk;
use crate::png::types::color_type::ColorType;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// https://www.w3.org/TR/png-3/#table111
pub enum ImageType {
    /// Greyscale, bit depth: 1
    G1 = 0,
    /// Greyscale, bit depth: 2
    G2 = 1,
    /// Greyscale, bit depth: 4
    G4 = 2,
    /// Greyscale, bit depth: 8
    G8 = 3,
    /// Greyscale, bit depth: 16
    G16 = 4,
    /// Greyscale with alpha, bit depth: 8
    GA16 = 5,
    /// Greyscale with alpha, bit depth: 16
    GA32 = 6,
    /// Indexed-color, bit depth: 1
    PAL1 = 7,
    /// Indexed-color, bit depth: 2
    PAL2 = 8,
    /// Indexed-color, bit depth: 4
    PAL4 = 9,
    /// Indexed-color, bit depth: 8
    PAL8 = 10,
    /// Truecolor, bit depth: 8
    RGB24 = 11,
    /// Truecolor, bit depth: 16
    RGB48 = 12,
    /// Truecolor with alpha, bit depth: 8
    RGBA32 = 13,
    /// Truecolor with alpha, bit depth: 16
    RGBA64 = 14,
}

impl ImageType {
    pub fn bit_depth(&self) -> u8 {
        match self {
            Self::G1 => 1,
            Self::G2 => 2,
            Self::G4 => 4,
            Self::G8 => 8,
            Self::G16 => 16,
            Self::GA16 => 8,
            Self::GA32 => 16,
            Self::PAL1 => 1,
            Self::PAL2 => 2,
            Self::PAL4 => 4,
            Self::PAL8 => 8,
            Self::RGB24 => 8,
            Self::RGB48 => 16,
            Self::RGBA32 => 8,
            Self::RGBA64 => 16,
        }
    }

    pub fn color_type(&self) -> ColorType {
        match self {
            Self::G1 => ColorType::Greyscale,
            Self::G2 => ColorType::Greyscale,
            Self::G4 => ColorType::Greyscale,
            Self::G8 => ColorType::Greyscale,
            Self::G16 => ColorType::Greyscale,
            Self::GA16 => ColorType::GreyscaleAlpha,
            Self::GA32 => ColorType::GreyscaleAlpha,
            Self::PAL1 => ColorType::Indexed,
            Self::PAL2 => ColorType::Indexed,
            Self::PAL4 => ColorType::Indexed,
            Self::PAL8 => ColorType::Indexed,
            Self::RGB24 => ColorType::Truecolor,
            Self::RGB48 => ColorType::Truecolor,
            Self::RGBA32 => ColorType::TruecolorAlpha,
            Self::RGBA64 => ColorType::TruecolorAlpha,
        }
    }
}

impl TryFrom<(ColorType, u8)> for ImageType {
    type Error = InvalidChunk;

    fn try_from(value: (ColorType, u8)) -> Result<Self, Self::Error> {
        match value {
            (ColorType::Greyscale, 1) => Ok(Self::G1),
            (ColorType::Greyscale, 2) => Ok(Self::G2),
            (ColorType::Greyscale, 4) => Ok(Self::G4),
            (ColorType::Greyscale, 8) => Ok(Self::G8),
            (ColorType::Greyscale, 16) => Ok(Self::G16),
            (ColorType::GreyscaleAlpha, 8) => Ok(Self::GA16),
            (ColorType::GreyscaleAlpha, 16) => Ok(Self::GA32),
            (ColorType::Indexed, 1) => Ok(Self::PAL1),
            (ColorType::Indexed, 2) => Ok(Self::PAL2),
            (ColorType::Indexed, 4) => Ok(Self::PAL4),
            (ColorType::Indexed, 8) => Ok(Self::PAL8),
            (ColorType::Truecolor, 8) => Ok(Self::RGB24),
            (ColorType::Truecolor, 16) => Ok(Self::RGB48),
            (ColorType::TruecolorAlpha, 8) => Ok(Self::RGBA32),
            (ColorType::TruecolorAlpha, 16) => Ok(Self::RGBA64),
            _ => Err(InvalidChunk::InvalidImageType {
                color_type: value.0,
                bit_depth: value.1,
            }),
        }
    }
}
