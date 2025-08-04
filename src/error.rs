use crate::png::chunk::ChunkType;
use crate::png::invalid_chunk::InvalidChunk;

pub type PngResult<T> = Result<T, PngError>;

#[derive(Debug)]
pub enum PngError {
    DuplicateHeader,
    InvalidChunk {
        chunk_type: ChunkType,
        offset: usize,
        kind: InvalidChunk,
    },
    InvalidImageData,
    InvalidSignature,
    MisplacedHeader,
    MissingEnd,
    MissingHeader,
    MissingPalette,
    MissingSignature,
    ReaderOverflow {
        offset: usize,
    },
    UnknownChunkType(u32),
    Io(std::io::Error),
}

impl PngError {
    pub fn invalid_chunk(chunk_type: ChunkType, offset: usize, kind: InvalidChunk) -> Self {
        Self::InvalidChunk {
            chunk_type,
            offset,
            kind,
        }
    }

    pub fn reader_overflow(offset: usize) -> Self {
        Self::ReaderOverflow { offset }
    }
}

impl std::fmt::Display for PngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PngError::DuplicateHeader => write!(f, "Duplicate header chunks"),
            PngError::InvalidChunk {
                chunk_type,
                offset,
                kind,
            } => {
                write!(f, "Invalid chunk ({chunk_type}) at offset {offset}: {kind}")
            }
            PngError::InvalidImageData => write!(f, "Invalid image data"),
            PngError::InvalidSignature => write!(f, "Invalid signature"),
            PngError::MisplacedHeader => write!(f, "Misplaced header chunk"),
            PngError::MissingEnd => write!(f, "Missing end chunk"),
            PngError::MissingHeader => write!(f, "Missing header"),
            PngError::MissingPalette => write!(f, "Missing palette"),
            PngError::MissingSignature => write!(f, "Missing signature"),
            PngError::ReaderOverflow { offset } => {
                write!(f, "Reader overflow at offset {offset}")
            }
            PngError::UnknownChunkType(chunk_type) => {
                write!(f, "Unknown chunk type: 0x{chunk_type:02X?}")
            }
            PngError::Io(err) => write!(f, "IO error: {err}"),
        }
    }
}

impl std::error::Error for PngError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for PngError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
