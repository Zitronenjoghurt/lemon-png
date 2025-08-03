use crate::png::types::scanline::FilteredScanline;

#[derive(Debug)]
pub struct IDATChunk {
    pub scanlines: Vec<FilteredScanline>,
}
