use color::Color;
use color::RgbColorAverage;
use pointid::PointId;

/// A color-averaging object per point.
#[derive(Debug)]
pub struct PointColorAverages {
    averages: Vec<RgbColorAverage>,
}

impl PointColorAverages {
    pub fn new(count: usize) -> Self {
        let mut colors = Vec::with_capacity(count);
        for _ in 0..count {
            colors.push(RgbColorAverage::new());
        }
        PointColorAverages { averages: colors }
    }

    //TODO @mark: is there some way to recycle PointColorAverages memory for PointColors
    pub fn compute(self) -> PointColors {
        let mut colors: Vec<Color> = Vec::with_capacity(self.len());
        for avg in self.averages.into_iter() {
            colors.push(avg.calc_avg())
        }
        PointColors::new(colors)
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

#[cfg(test)]
mod tests {
    use super::*;
    use color::new_color;

    #[test]
    fn test_averages_to_colors() {
        let mut avgs = PointColorAverages::new(2);
        let mut avg0 = avgs.get(PointId::new(0));
        avg0 += new_color(255u8, 255u8, 255u8);
        avg0 += new_color(0u8, 0u8, 0u8);
        let mut avg1 = avgs.get(PointId::new(1));
        avg1 += new_color(255u8, 255u8, 255u8);
        let colors = avgs.compute();
        assert_eq!(new_color(127u8, 127u8, 127u8), colors.get(PointId::new(0)));
        assert_eq!(new_color(255u8, 255u8, 255u8), colors.get(PointId::new(1)));
    }
}
