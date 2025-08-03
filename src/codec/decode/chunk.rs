use crate::codec::chunk::context::ChunkContext;
use crate::codec::chunk::{idat, ihdr, plte};
use crate::codec::parsed_chunk::ParsedChunk;
use crate::codec::raw_chunk::RawChunk;
use crate::codec::validate::chunk::ChunkValidator;
use crate::error::{PngError, PngResult};
use crate::png::chunk::{Chunk, ChunkType};
use crate::png::invalid_chunk::InvalidChunk;

#[derive(Debug, Default)]
pub struct ChunkDecoder {
    validator: ChunkValidator,
}

impl ChunkDecoder {
    pub fn new(validator: ChunkValidator) -> Self {
        Self { validator }
    }

    pub fn decode(&self, context: &mut ChunkContext, raw: RawChunk) -> PngResult<Chunk> {
        let chunk_type = ChunkType::try_from(raw.chunk_type)?;
        let offset = raw.offset;
        if !raw.validate_crc() {
            return Err(PngError::invalid_chunk(
                chunk_type,
                offset,
                InvalidChunk::CRC,
            ));
        }

        let parsed_chunk = self.parse(context, raw)?;
        let chunk = self.validator.validate(context, parsed_chunk)?;
        Ok(chunk)
    }

    pub fn decode_all(
        &mut self,
        context: &mut ChunkContext,
        raw_chunks: impl Iterator<Item = RawChunk>,
    ) -> PngResult<Vec<Chunk>> {
        let mut result = Vec::new();
        for raw_chunk in raw_chunks {
            let chunk = self.decode(context, raw_chunk)?;
            Self::update_context_from_chunk(context, &chunk);
            result.push(chunk);
        }
        Ok(result)
    }

    pub fn decode_all_skip_errors(
        &mut self,
        context: &mut ChunkContext,
        raw_chunks: impl Iterator<Item = RawChunk>,
    ) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        for raw_chunk in raw_chunks {
            if let Ok(chunk) = self.decode(context, raw_chunk) {
                Self::update_context_from_chunk(context, &chunk);
                chunks.push(chunk);
            }
        }
        chunks
    }

    pub fn parse(&self, context: &mut ChunkContext, raw: RawChunk) -> PngResult<ParsedChunk> {
        Self::update_context_from_raw_chunk(context, &raw);

        let chunk_type = ChunkType::try_from(raw.chunk_type)?;
        let parsed_chunk = match chunk_type {
            ChunkType::ImageData => ParsedChunk::ImageData(idat::parse(raw)?),
            ChunkType::ImageEnd => ParsedChunk::ImageEnd,
            ChunkType::ImageHeader => ParsedChunk::ImageHeader(ihdr::parse(raw)?),
            ChunkType::Palette => ParsedChunk::Palette(plte::parse(raw)?),
        };

        Ok(parsed_chunk)
    }

    fn update_context_from_raw_chunk(context: &mut ChunkContext, raw_chunk: &RawChunk) {
        context.set_offset(raw_chunk.offset);
    }

    fn update_context_from_chunk(context: &mut ChunkContext, chunk: &Chunk) {
        if let Chunk::ImageHeader(header) = chunk {
            context.set_width(header.width);
            context.set_height(header.height);
            context.set_color_type(header.color_type());
            context.set_bit_depth(header.bit_depth());
        }
    }
}
