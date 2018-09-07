use image::Rgb;

/// Add colors to compute average.
/// This works un to about 4000x4000 all white.
#[derive(Debug)]
pub struct RgbColorAverage {
    c1: u32,
    c2: u32,
    c3: u32,
    count: u32,
}

impl RgbColorAverage {
    pub fn new() -> Self {
        RgbColorAverage { c1: 0, c2: 0, c3: 0, count: 0 }
    }

    pub fn calc_avg(&self) -> Rgb<u8> {
        Rgb([
            (self.c1 / self.count) as u8,
            (self.c2 / self.count) as u8,
            (self.c3 / self.count) as u8,
        ])
    }
}



