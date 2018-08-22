use point::Point;

/// Collection of *unique* points.
pub struct UPoints<P> where P: Point {
    points: Vec<P>,
}

#[derive(Debug, Clone, Copy)]
pub struct PointId {
    value: usize,
}

impl PointId {
    pub fn new(value: usize) -> Self {
        PointId { value }
    }
}

impl<P> UPoints<P> where P: Point {
    pub fn new(points: Vec<P>) -> Self {
        UPoints { points }
    }

    pub fn get(&self, id: PointId) -> &P {
        &self.points[id.value]
    }
}
