
use dims::{X, Y, dX, dY};
use std::ops::{Add, Sub};

pub trait Point {
    fn from(x: X, y: Y) -> Self;
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Point2D {
    x: X,
    y: Y,
}

impl Point for Point2D {
    fn from(x: X, y: Y) -> Self {
        Point2D { x, y }
    }
}

/// A Step is a vector that can e.g. point from one point to another.
pub trait Step {
    fn from(dx: dX, dy: dY) -> Self;
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Step2D {
    dx: dX,
    dy: dY,
}

impl Step for Step2D {
    fn from(dx: dX, dy: dY) -> Self {
        Step2D { dx, dy }
    }
}

/// This must be done for each Point type, because S must be specified
impl Sub<Point2D> for Point2D {
    type Output = Step2D;

    fn sub(self, other: Point2D) -> Self::Output {
        Step2D::from(self.x - other.x, self.y - other.y)
    }
}

impl<S> Sub<S> for Point2D where S: Step {
    type Output = Point2D;

    fn sub(self, other: S) -> Self::Output {
        Point2D::from(self.x - other.dx, self.y - other.dy)
    }
}

impl<S> Add<S> for Point2D where S: Step {
    type Output = Point2D;

    fn add(self, other: S) -> Self::Output {
        Point2D::from(self.x + other.dx, self.y + other.dy)
    }
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
