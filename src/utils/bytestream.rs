pub fn get_u8(stream: &[u8], offset: usize) -> Option<u8> {
    stream.get(offset).copied()
}

pub fn get_u16_be(stream: &[u8], offset: usize) -> Option<u16> {
    stream
        .get(offset..offset + 2)
        .and_then(|slice| slice.try_into().ok())
        .map(u16::from_be_bytes)
}

pub fn get_u32_be(stream: &[u8], offset: usize) -> Option<u32> {
    stream
        .get(offset..offset + 4)
        .and_then(|slice| slice.try_into().ok())
        .map(u32::from_be_bytes)
}

pub fn get_u64_be(stream: &[u8], offset: usize) -> Option<u64> {
    stream
        .get(offset..offset + 8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_be_bytes)
}
