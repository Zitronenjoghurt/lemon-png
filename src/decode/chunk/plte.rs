use crate::error::PngResult;
use crate::png::chunk::plte::RawPLTEChunk;
use crate::png::raw_chunk::RawChunk;

pub fn decode(raw: RawChunk) -> PngResult<RawPLTEChunk> {
    Ok(RawPLTEChunk { colors: raw.data })
}
