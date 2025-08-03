use crate::color::Color;

#[derive(Debug)]
pub struct FilteredScanline {
    pub filter: u8,
    pub colors: Vec<Color>,
}

#[derive(Debug)]
pub struct UnfilteredScanline(Vec<Color>);
