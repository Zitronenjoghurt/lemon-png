use crate::utils::crc::update_crc32;

#[derive(Debug)]
pub struct RawChunk {
    pub offset: usize,
    pub length: u32,
    pub chunk_type: u32,
    pub data: Vec<u8>,
    pub crc: u32,
}

impl RawChunk {
    pub fn validate_crc(&self) -> bool {
        let mut crc = u32::MAX;
        crc = update_crc32(crc, &self.chunk_type.to_be_bytes());
        crc = update_crc32(crc, &self.data);
        (crc ^ u32::MAX) == self.crc
    }
}
