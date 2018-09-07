use image::Rgb;
use std::ops::Add;
use std::ops::AddAssign;

/// Add colors to compute average.
/// This works un to about 4000x4000 all white.
#[derive(Debug)]
pub struct RgbColorAverage {
    c0: u32,
    c1: u32,
    c2: u32,
    count: u32,
}

impl RgbColorAverage {
    pub fn new() -> Self {
        RgbColorAverage { c0: 0, c1: 0, c2: 0, count: 0 }
    }

    pub fn calc_avg(&self) -> Rgb<u8> {
        assert!(self.count > 0);
        Rgb([
            (self.c0 / self.count) as u8,
            (self.c1 / self.count) as u8,
            (self.c2 / self.count) as u8,
        ])
    }
}

impl AddAssign<Rgb<u8>> for RgbColorAverage {
    fn add_assign(&mut self, color: Rgb<u8>) {
        self.c0 += color.data[0] as u32;
        self.c1 += color.data[1] as u32;
        self.c2 += color.data[2] as u32;
        self.count += 1;
    }
}

impl Add<Rgb<u8>> for RgbColorAverage {
    type Output = Self;

    fn add(mut self, color: Rgb<u8>) -> Self::Output {
        self += color;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_avg_same() {
        let mut avg = RgbColorAverage::new();
        avg += Rgb([127u8, 127u8, 127u8]);
        avg += Rgb([127u8, 127u8, 127u8]);
        avg += Rgb([127u8, 127u8, 127u8]);
        assert_eq!(Rgb([127u8, 127u8, 127u8]), avg.calc_avg());
    }

    #[test]
    fn test_color_avg_minmax() {
        let mut avg = RgbColorAverage::new();
        avg += Rgb([0u8, 0u8, 0u8]);
        avg += Rgb([255u8, 255u8, 255u8]);
        avg += Rgb([255u8, 255u8, 255u8]);
        assert_eq!(Rgb([170u8, 170u8, 170u8]), avg.calc_avg());
    }
}