use crate::decode::chunk::ChunkDecoder;
use crate::decode::raw_chunks::RawChunkExtractor;
use crate::decode::reader::Reader;
use crate::error::{PngError, PngResult};
use crate::png::chunk::Chunk;
use crate::png::{Png, PNG_SIGNATURE};

pub struct PngDecoder<'a> {
    data: &'a [u8],
    offset: usize,
    config: &'a PngDecoderConfig,
    chunk_decoder: ChunkDecoder,
}

impl<'a> PngDecoder<'a> {
    pub fn new(data: &'a [u8], config: &'a PngDecoderConfig) -> Self {
        Self {
            data,
            offset: 0,
            config,
            chunk_decoder: ChunkDecoder,
        }
    }

    pub fn decode(mut self) -> PngResult<Png> {
        self.verify_signature()?;

        let chunks = if self.config.skip_erroneous_chunks {
            self.decode_chunks_skip_erroneous()
        } else {
            self.decode_chunks()?
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

    fn decode_chunks(&mut self) -> PngResult<Vec<Chunk>> {
        RawChunkExtractor::new(&self.data[8..]).try_fold(Vec::new(), |mut chunks, raw_chunk| {
            let chunk = self.chunk_decoder.decode(raw_chunk)?;
            chunks.push(chunk);
            Ok(chunks)
        })
    }

    fn decode_chunks_skip_erroneous(&mut self) -> Vec<Chunk> {
        RawChunkExtractor::new(&self.data[8..])
            .filter_map(|raw_chunk| self.chunk_decoder.decode(raw_chunk).ok())
            .collect()
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
