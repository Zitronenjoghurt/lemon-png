use crate::codec::chunk::context::ChunkContext;
use crate::codec::decode::chunk::ChunkDecoder;
use crate::codec::decode::raw_chunks::RawChunkExtractor;
use crate::codec::decode::reader::Reader;
use crate::codec::validate::chunk::ChunkValidator;
use crate::error::{PngError, PngResult};
use crate::png::{Png, PNG_SIGNATURE};

pub struct PngDecoder<'a> {
    data: &'a [u8],
    offset: usize,
    config: &'a PngDecoderConfig,
}

impl<'a> PngDecoder<'a> {
    pub fn new(data: &'a [u8], config: &'a PngDecoderConfig) -> Self {
        Self {
            data,
            offset: 0,
            config,
        }
    }

    pub fn decode(mut self) -> PngResult<Png> {
        self.verify_signature()?;

        let extractor = RawChunkExtractor::new(&self.data[8..]);
        let mut chunk_context = ChunkContext::default();
        let mut chunk_decoder = ChunkDecoder::new(ChunkValidator::default());

        let chunks = if self.config.skip_erroneous_chunks {
            chunk_decoder.decode_all_skip_errors(&mut chunk_context, extractor)
        } else {
            chunk_decoder.decode_all(&mut chunk_context, extractor)?
        };

        Ok(Png::new(chunks))
    }

    fn verify_signature(&mut self) -> PngResult<()> {
        let signature = self.read_u64().map_err(|_| PngError::MissingSignature)?;
        if signature != PNG_SIGNATURE {
            Err(PngError::InvalidSignature)
        } else {
            Ok(())
        }
    }
}

impl Reader for PngDecoder<'_> {
    fn get_data(&self) -> &[u8] {
        self.data
    }

    fn get_offset(&self) -> usize {
        self.offset
    }

    fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PngDecoderConfig {
    pub skip_erroneous_chunks: bool,
}

impl PngDecoderConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn skip_erroneous_chunks(mut self) -> Self {
        self.skip_erroneous_chunks = true;
        self
    }
}
