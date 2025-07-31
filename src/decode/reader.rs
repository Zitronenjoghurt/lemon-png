use crate::error::{PngError, PngResult};
use crate::utils::bytestream::{read_u32_be, read_u64_be};

pub trait Reader {
    fn get_data(&self) -> &[u8];
    fn get_offset(&self) -> usize;
    fn set_offset(&mut self, offset: usize);

    fn read_bytes(&mut self, len: usize) -> PngResult<&[u8]> {
        let offset = self.get_offset();
        if offset + len <= self.get_data().len() {
            self.set_offset(offset + len);
            let slice = &self.get_data()[offset..offset + len];
            Ok(slice)
        } else {
            Err(PngError::reader_overflow(offset))
        }
    }

    fn read_u32(&mut self) -> PngResult<u32> {
        let result = read_u32_be(self.get_data(), self.get_offset())
            .ok_or(PngError::reader_overflow(self.get_offset()))?;
        self.set_offset(self.get_offset() + 4);
        Ok(result)
    }

    fn read_u64(&mut self) -> PngResult<u64> {
        let result = read_u64_be(self.get_data(), self.get_offset())
            .ok_or(PngError::reader_overflow(self.get_offset()))?;
        self.set_offset(self.get_offset() + 8);
        Ok(result)
    }

    fn has_data(&self) -> bool {
        self.get_offset() < self.get_data().len()
    }
}
