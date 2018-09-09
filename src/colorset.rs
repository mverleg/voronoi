use color::RgbColorAverage;
use pointid::PointId;
use color::Color;

/// A color-averaging object per point.
#[derive(Debug)]
pub struct PointColorAverages {
    averages: Vec<RgbColorAverage>,
}

impl PointColorAverages {
    pub fn new(count: usize) -> Self {
        let mut colors = Vec::with_capacity(count);
        for _ in 0 .. count {
            colors.push(RgbColorAverage::new());
        }
        PointColorAverages { averages: colors }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.averages.len()
    }

    #[inline]
    pub fn get(&mut self, id: PointId) -> &mut RgbColorAverage {
        &mut self.averages[id._expose()]
    }
}

/// A c computed average color per point.
#[derive(Debug)]
pub struct PointColors {
    colors: Vec<Color>,
}

impl PointColors {
    pub fn new(colors: Vec<Color>) -> Self {
        PointColors { colors }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.colors.len()
    }

    #[inline]
    pub fn get(&self, id: PointId) -> Color {
        //TODO @mark: is this indeed a copy type?
        self.colors[id._expose()]
    }
}
