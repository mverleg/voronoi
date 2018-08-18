
use dims::{X, Y};

pub trait Point {}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Point2D {
    x: X,
    y: Y,
}

/// Collection of *unique* points.
pub struct UPoints<P> where P: Point {
    points: Vec<P>,
}

pub struct PointId {
    val: usize,
}

impl<P> UPoints<P> where P: Point {
    pub fn new(points: Vec<P>) -> Self {
        UPoints { points }
    }

    pub fn get(&self, id: PointId) -> &P {
        &self.points[id.val]
    }

    pub fn within(&self, left: X, right: X, top: Y, bottom: Y) -> &Vec<P> {
        //TODO @mark:
        unimplemented!()
    }
}


