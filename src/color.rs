use image::Rgb;
use std::ops::Add;
use std::ops::AddAssign;

pub type Color = Rgb<u8>;

pub fn new_color(c0: u8, c1: u8, c2: u8) -> Color {
    Rgb([c0, c1, c2])
}

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
        RgbColorAverage {
            c0: 0,
            c1: 0,
            c2: 0,
            count: 0,
        }
    }

    pub fn calc_avg(&self) -> Color {
        debug_assert!(self.count > 0, "No colors have been added for this average; this should not happen if all points are unique");
        new_color(
            (self.c0 / self.count) as u8,
            (self.c1 / self.count) as u8,
            (self.c2 / self.count) as u8,
        )
    }

    fn add(&mut self, color: Color) {
        self.c0 += color.data[0] as u32;
        self.c1 += color.data[1] as u32;
        self.c2 += color.data[2] as u32;
        self.count += 1;
    }
}

impl AddAssign<Color> for RgbColorAverage {
    fn add_assign(&mut self, color: Color) {
        self.add(color);
    }
}

impl<'a> AddAssign<Color> for &'a mut RgbColorAverage {
    fn add_assign(&mut self, color: Color) {
        self.add(color);
    }
}

impl Add<Color> for RgbColorAverage {
    type Output = Self;

    fn add(mut self, color: Color) -> Self::Output {
        self += color;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_avg_same() {
        // Also tests u8 overflow
        let mut avg = RgbColorAverage::new();
        avg += new_color(127u8, 127u8, 127u8);
        avg += new_color(127u8, 127u8, 127u8);
        avg += new_color(127u8, 127u8, 127u8);
        assert_eq!(new_color(127u8, 127u8, 127u8), avg.calc_avg());
    }

    #[test]
    fn test_color_avg_minmax() {
        let mut avg = RgbColorAverage::new();
        avg += new_color(0u8, 0u8, 0u8);
        avg += new_color(255u8, 255u8, 255u8);
        avg += new_color(255u8, 255u8, 255u8);
        assert_eq!(new_color(170u8, 170u8, 170u8), avg.calc_avg());
    }
}
