use crate::png::types::color_type::ColorType;

#[derive(Debug)]
pub enum InvalidChunk {
    CRC,
    Length {
        expected: usize,
        actual: usize,
    },
    InvalidColorType {
        value: u8,
    },
    InvalidCompressionMethod {
        value: u8,
    },
    InvalidFilterMethod {
        value: u8,
    },
    InvalidImageType {
        color_type: ColorType,
        bit_depth: u8,
    },
    InvalidInterlaceMethod {
        value: u8,
    },
}

impl std::fmt::Display for InvalidChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidChunk::CRC => write!(f, "Invalid CRC"),
            InvalidChunk::Length { expected, actual } => write!(
                f,
                "Invalid chunk length: expected {expected}, actual: {actual}",
            ),
            InvalidChunk::InvalidColorType { value } => write!(
                f,
                "Invalid color type: expected 0, 2, 3, 4, 6, or 8, actual: {value}"
            ),
            InvalidChunk::InvalidCompressionMethod { value } => {
                write!(f, "Invalid compression method: expected 0, actual: {value}")
            }
            InvalidChunk::InvalidFilterMethod { value } => {
                write!(f, "Invalid filter method: expected 0, actual: {value}")
            }
            InvalidChunk::InvalidImageType {
                color_type,
                bit_depth,
            } => write!(
                f,
                "Invalid image type: color type: {color_type}, bit depth: {bit_depth}"
            ),
            InvalidChunk::InvalidInterlaceMethod { value } => {
                write!(
                    f,
                    "Invalid interlace method: expected 0 or 1, actual: {value}"
                )
            }
        }
    }
}
