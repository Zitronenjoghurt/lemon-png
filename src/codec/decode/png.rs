use crate::codec::decode::chunk::ChunkDecoder;
use crate::codec::decode::raw_chunks::RawChunkExtractor;
use crate::codec::decode::reader::Reader;
use crate::codec::validate::chunk::DefaultChunkValidator;
use crate::error::{PngError, PngResult};
use crate::png::chunk::{Chunk, ChunkType, Chunks};
use crate::png::types::color_type::ColorType;
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
        let validator = DefaultChunkValidator;
        let mut decoder = ChunkDecoder::new();

        let chunks = if self.config.skip_erroneous_chunks {
            decoder.decode_all_skip_errors(extractor, &validator)
        } else {
            decoder.decode_all(extractor, &validator)?
        };

        self.assemble_png(chunks)
    }

    fn verify_signature(&mut self) -> PngResult<()> {
        let signature = self.read_u64().map_err(|_| PngError::MissingSignature)?;
        if signature != PNG_SIGNATURE {
            Err(PngError::InvalidSignature)
        } else {
            Ok(())
        }
    }

    fn assemble_png(&self, chunks: Chunks) -> PngResult<Png> {
        let Some((header_index, Chunk::ImageHeader(header))) =
            chunks.get_one_by_type_with_index(ChunkType::ImageHeader)
        else {
            return Err(PngError::MissingHeader);
        };
        if header_index != 0 {
            return Err(PngError::MisplacedHeader);
        };
        if !chunks.is_type_unique(ChunkType::ImageData) {
            return Err(PngError::DuplicateHeader);
        };

        let chunk_count = chunks.len();
        if !chunks.is_type_at_index_and_unique(ChunkType::ImageEnd, chunk_count - 1) {
            return Err(PngError::MissingEnd);
        };

        let palette = match chunks.get_one_by_type(ChunkType::Palette) {
            Some(Chunk::Palette(palette)) => Some(palette),
            _ => None,
        };
        if header.color_type() == ColorType::Indexed && !chunks.is_type_unique(ChunkType::Palette) {
            return Err(PngError::MissingPalette);
        };

        let data =
            chunks
                .iter_by_type(ChunkType::ImageData)
                .fold(Vec::new(), |mut acc, (_, chunk)| {
                    if let Chunk::ImageData(data_chunk) = chunk {
                        acc.extend(data_chunk.compressed.clone());
                    }
                    acc
                });

        let mut builder = Png::builder().header_chunk(header).compressed_data(data);
        if let Some(palette) = palette {
            builder = builder.palette_chunk(palette);
        }

        Ok(builder.build())
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
