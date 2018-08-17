
use dims::{X, Y};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Point {
    x: X,
    y: Y,
}

pub struct Points {
    // The points must be unique.
    points: Vec<Point>,
}

pub struct PointId {
    val: usize,
}

impl Points {
    pub fn new(points: Vec<Point>) -> Self {
        Points { points }
    }

    pub fn get(&self, id: PointId) -> &Point {
        &self.points[id.val]
    }

    pub fn within(&self, left: X, right: X, top: Y, bottom: Y) -> &Vec<Point> {
        //TODO @mark:
        unimplemented!()
    }
}
