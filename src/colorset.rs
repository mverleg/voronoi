use crate::color::Color;
use crate::color::RgbColorAverage;
use crate::pointid::PointId;
use ::std::ops::Index;
use ::std::ops::IndexMut;

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

    pub fn compute(self) -> PointColors {
        //TODO @mark: parallel
        let mut colors: Vec<Color> = Vec::with_capacity(self.len());
        let mut d = 0;  //TODO @mark: TEMPORARY! REMOVE THIS!
        let mut e = 0;  //TODO @mark: TEMPORARY! REMOVE THIS!
        for c in &self.averages {  //TODO @mark: TEMPORARY! REMOVE THIS!
            if c.count <= 0 {
                d += 1;
            }
            e += c.count;
            eprintln!("{:?}", c);
        }
        eprintln!("{} of {} zero, total {}", d, self.averages.len(), e);
        for avg in self.averages.into_iter() {
            colors.push(avg.calc_avg())
        }
        PointColors::new(colors)
    }

    pub fn len(&self) -> usize {
        self.averages.len()
    }

    pub fn get(&mut self, id: PointId) -> &mut RgbColorAverage {
        &mut self.averages[id.as_index()]
    }
}

impl Index<PointId> for PointColorAverages {
    type Output = RgbColorAverage;
    fn index(&self, index: PointId) -> &Self::Output {
        &self.averages[index.as_index() as usize]
    }
}

impl IndexMut<PointId> for PointColorAverages {
    fn index_mut(&mut self, index: PointId) -> &mut Self::Output {
        self.get(index)
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
    pub fn len(&self) -> usize {
        self.colors.len()
    }
    pub fn get(&self, id: PointId) -> Color {
        self.colors[id.as_index()]
    }
}

impl Index<PointId> for PointColors {
    type Output = Color;
    fn index(&self, index: PointId) -> &Self::Output {
        &self.colors[index.as_index()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::new_color;

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
