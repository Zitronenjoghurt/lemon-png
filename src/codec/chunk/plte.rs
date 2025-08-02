use crate::codec::raw_chunk::RawChunk;
use crate::error::PngResult;

#[derive(Debug)]
pub struct ParsedPLTEChunk {
    pub colors: Vec<u8>,
}

pub fn decode(raw: RawChunk) -> PngResult<ParsedPLTEChunk> {
    Ok(ParsedPLTEChunk { colors: raw.data })
}
