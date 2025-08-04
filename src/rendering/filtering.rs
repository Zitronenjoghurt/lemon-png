use crate::rendering::filtering::filter_type::FilterType;

pub mod filter_type;

#[derive(Debug, Default)]
pub struct Filterer;

impl Filterer {
    pub fn new() -> Self {
        Self
    }

    pub fn filter(&self, stream: &[u8], scanline_width: usize) -> Vec<u8> {
        todo!()
    }

    pub fn unfilter(
        &self,
        stream: &[u8],
        image_height: u32,
        scanline_width: usize,
        bytes_per_pixel: usize,
    ) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::with_capacity(image_height as usize * (scanline_width - 1));

        for row in 0..image_height as usize {
            let scanline_start = row * scanline_width;
            let scanline_end = scanline_start + scanline_width;
            let filter_type = FilterType::try_from(stream[scanline_start]).unwrap_or_default();

            for (column, filtered_offset) in (scanline_start + 1..scanline_end).enumerate() {
                let x = stream[filtered_offset];
                let unfiltered_offset = row * (scanline_width - 1) + column;

                let a = if column >= bytes_per_pixel {
                    result[unfiltered_offset - bytes_per_pixel]
                } else {
                    0
                };

                let b = if row > 0 {
                    result[unfiltered_offset - (scanline_width - 1)]
                } else {
                    0
                };

                let c = if column >= bytes_per_pixel && row > 0 {
                    result[unfiltered_offset - (scanline_width - 1) - bytes_per_pixel]
                } else {
                    0
                };

                let unfiltered_x = Self::unfilter_byte(a, b, c, x, filter_type);
                result.push(unfiltered_x);
            }
        }

        result
    }

    pub fn filter_byte(a: u8, b: u8, c: u8, x: u8, filter_type: FilterType) -> u8 {
        match filter_type {
            FilterType::None => x,
            FilterType::Sub => x.wrapping_sub(a),
            FilterType::Up => x.wrapping_sub(b),
            FilterType::Average => (x as u16 - ((a as u16 + b as u16) / 2)) as u8,
            FilterType::Paeth => x.wrapping_sub(Self::paeth(a, b, c)),
        }
    }

    pub fn unfilter_byte(a: u8, b: u8, c: u8, x: u8, filter_type: FilterType) -> u8 {
        match filter_type {
            FilterType::None => x,
            FilterType::Sub => x.wrapping_add(a),
            FilterType::Up => x.wrapping_add(b),
            FilterType::Average => (x as u16 + ((a as u16 + b as u16) / 2)) as u8,
            FilterType::Paeth => x.wrapping_add(Self::paeth(a, b, c)),
        }
    }

    pub fn paeth(a: u8, b: u8, c: u8) -> u8 {
        let a = a as i16;
        let b = b as i16;
        let c = c as i16;

        let p = a + b - c;

        let pa = (p - a).abs();
        let pb = (p - b).abs();
        let pc = (p - c).abs();

        if pa <= pb && pa <= pc {
            a as u8
        } else if pb <= pc {
            b as u8
        } else {
            c as u8
        }
    }
}
