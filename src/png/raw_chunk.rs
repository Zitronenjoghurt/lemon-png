#[derive(Debug)]
pub struct RawChunk {
    pub offset: usize,
    pub length: u32,
    pub chunk_type: u32,
    pub data: Vec<u8>,
    pub crc: u32,
}
