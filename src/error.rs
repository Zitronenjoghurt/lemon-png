use crate::chunk::ihdr::IHDRChunkError;
use crate::chunk::ChunkError;

pub type PngResult<T> = Result<T, PngError>;

#[derive(Debug)]
pub enum PngError {
    InvalidChunk(ChunkError),
    InvalidChunkType(u32),
    MissingSignature,
    UnidentifiableChunk,
    Io(std::io::Error),
}

impl PngError {
    pub fn invalid_chunk_type(value: u32) -> Self {
        PngError::InvalidChunkType(value)
    }
}

impl std::fmt::Display for PngError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PngError::InvalidChunk(err) => write!(f, "Invalid chunk: {}", err),
            PngError::InvalidChunkType(value) => write!(f, "Invalid chunk type: {:#010X}", value),
            PngError::MissingSignature => write!(f, "Missing signature"),
            PngError::UnidentifiableChunk => write!(f, "Unidentifiable chunk"),
            PngError::Io(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for PngError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidChunk(err) => Some(err),
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<ChunkError> for PngError {
    fn from(value: ChunkError) -> Self {
        Self::InvalidChunk(value)
    }
}

impl From<std::io::Error> for PngError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<IHDRChunkError> for PngError {
    fn from(value: IHDRChunkError) -> Self {
        Self::InvalidChunk(value.into())
    }
}
