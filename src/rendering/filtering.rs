use crate::rendering::filtering::filter_type::FilterType;
use crate::rendering::filtering::prediction::PredictionGrid;

pub mod filter_type;
pub mod prediction;

#[derive(Debug, Default)]
pub struct Filterer;

impl Filterer {
    pub fn new() -> Self {
        Self
    }

    pub fn filter(&self, stream: &mut [u8], image_width: u32) {
        let scanline_width = image_width as usize + 1;
        let Some(mut grid) = PredictionGrid::new(stream, scanline_width) else {
            return;
        };

        grid.apply_filter(|predictors, filter_type| {
            Self::filter_byte(
                predictors.a,
                predictors.b,
                predictors.c,
                predictors.x,
                filter_type,
            )
        });
    }

    pub fn unfilter(&self, stream: &mut [u8], image_width: u32) {
        let scanline_width = image_width as usize + 1;
        let Some(mut grid) = PredictionGrid::new(stream, scanline_width) else {
            return;
        };

        grid.apply_unfilter(|predictors, filter_type| {
            Self::unfilter_byte(
                predictors.a,
                predictors.b,
                predictors.c,
                predictors.x,
                filter_type,
            )
        });
    }

    pub fn filter_byte(a: u8, b: u8, c: u8, x: u8, filter_type: FilterType) -> u8 {
        match filter_type {
            FilterType::None => x,
            FilterType::Sub => x.wrapping_sub(a),
            FilterType::Up => x.wrapping_sub(b),
            FilterType::Average => x.wrapping_sub(a.wrapping_add(b) / 2),
            FilterType::Paeth => Self::paeth(a, b, c),
        }
    }

    pub fn unfilter_byte(a: u8, b: u8, c: u8, x: u8, filter_type: FilterType) -> u8 {
        match filter_type {
            FilterType::None => x,
            FilterType::Sub => x.wrapping_add(a),
            FilterType::Up => x.wrapping_add(b),
            FilterType::Average => x.wrapping_add(a.wrapping_add(b) / 2),
            FilterType::Paeth => Self::paeth(a, b, c),
        }
    }

    pub fn paeth(a: u8, b: u8, c: u8) -> u8 {
        let p = a.wrapping_add(b).wrapping_sub(c);
        let pa = p.abs_diff(a);
        let pb = p.abs_diff(b);
        let pc = p.abs_diff(c);
        if pa <= pb && pa <= pc {
            a
        } else if pb <= pc {
            b
        } else {
            c
        }
    }
}
