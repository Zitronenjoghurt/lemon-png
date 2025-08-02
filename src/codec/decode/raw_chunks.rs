use crate::codec::decode::reader::Reader;
use crate::codec::raw_chunk::RawChunk;

pub struct RawChunkExtractor<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> RawChunkExtractor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    pub fn read_chunk(&mut self) -> Option<RawChunk> {
        let offset = self.offset;
        let length = self.read_u32().ok()?;
        let chunk_type = self.read_u32().ok()?;
        let data = self.read_bytes(length as usize).ok()?.to_vec();
        let crc = self.read_u32().ok()?;

        Some(RawChunk {
            offset,
            length,
            chunk_type,
            data,
            crc,
        })
    }
}

impl Reader for RawChunkExtractor<'_> {
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

impl Iterator for RawChunkExtractor<'_> {
    type Item = RawChunk;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_data() {
            self.read_chunk()
        } else {
            None
        }
    }
}
