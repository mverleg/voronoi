use dims::{dX, dY, X, Y};
use std::hash::Hash;
use std::ops::{Add, Sub};

pub trait Point: Sized + Eq + Hash {
    fn new(x: X, y: Y) -> Self;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point2D {
    x: X,
    y: Y,
}

impl Point2D {
    pub fn x(&self) -> X {
        self.x
    }

    pub fn y(&self) -> Y {
        self.y
    }
}

impl Point for Point2D {
    fn new(x: X, y: Y) -> Self {
        Point2D { x, y }
    }
}

/// A Step is a vector that can e.g. point from one point to another.
pub trait Step {
    fn new(dx: dX, dy: dY) -> Self;
    fn dx(&self) -> dX;
    fn dy(&self) -> dY;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Step2D {
    pub dx: dX,
    pub dy: dY,
}

impl Step for Step2D {
    fn new(dx: dX, dy: dY) -> Self {
        Step2D { dx, dy }
    }
    fn dx(&self) -> dX {
        self.dx
    }
    fn dy(&self) -> dY {
        self.dy
    }
}

/// This must be done for each Point type, because S must be specified
impl Sub<Point2D> for Point2D {
    type Output = Step2D;

    fn sub(self, other: Point2D) -> Self::Output {
        Step2D::new(self.x - other.x, self.y - other.y)
    }
}

impl<S> Sub<S> for Point2D where S: Step {
    type Output = Point2D;

    fn sub(self, other: S) -> Self::Output {
        Point2D::new(self.x - other.dx(), self.y - other.dy())
    }
}

impl<S> Add<S> for Point2D where S: Step {
    type Output = Point2D;

    fn add(self, other: S) -> Self::Output {
        Point2D::new(self.x + other.dx(), self.y + other.dy())
    }
}
