use crate::rendering::filtering::filter_type::FilterType;

pub struct BytePredictors {
    pub x: u8,
    pub a: u8,
    pub b: u8,
    pub c: u8,
}

pub struct PredictionGrid<'a> {
    data: &'a mut [u8],
    offset: usize,
    width: usize,
}

impl<'a> PredictionGrid<'a> {
    pub fn new(data: &'a mut [u8], width: usize) -> Option<Self> {
        if data.len() % width != 0 {
            return None;
        }

        Some(Self {
            data,
            offset: 0,
            width,
        })
    }

    pub fn apply_filter<F>(&mut self, filter_fn: F)
    where
        F: Fn(BytePredictors, FilterType) -> u8,
    {
        let line_count = self.data.len() / self.width;
        for scanline in (0..line_count).rev() {
            let scanline_start = scanline * self.width;
            let filter_type = FilterType::try_from(self.data[scanline_start]).unwrap_or_default();

            for i in (1..self.width).rev() {
                let offset = scanline_start + i;
                let predictors = self.get_predictors(offset);
                self.data[offset] = filter_fn(predictors, filter_type);
            }
        }
    }

    pub fn apply_unfilter<F>(&mut self, filter_fn: F)
    where
        F: Fn(BytePredictors, FilterType) -> u8,
    {
        let line_count = self.data.len() / self.width;
        for scanline in (0..line_count) {
            let scanline_start = scanline * self.width;
            let filter_type = FilterType::try_from(self.data[scanline_start]).unwrap_or_default();

            for i in (1..self.width) {
                let offset = scanline_start + i;
                let predictors = self.get_predictors(offset);
                self.data[offset] = filter_fn(predictors, filter_type);
            }
        }
    }

    pub fn get_predictors(&self, offset: usize) -> BytePredictors {
        let column = offset % self.width;
        let row = offset / self.width;

        let x = self.data[offset];
        let a = if column > 1 { self.data[offset - 1] } else { 0 };
        let b = if row > 0 {
            self.data[offset - self.width]
        } else {
            0
        };
        let c = if column > 1 && row > 0 {
            self.data[offset - self.width - 1]
        } else {
            0
        };

        BytePredictors { x, a, b, c }
    }
}
