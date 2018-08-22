use point::Point2D;

#[derive(Debug, Clone, Copy)]
pub struct PointId {
    value: usize,
}

impl PointId {
    pub fn new(value: usize) -> Self {
        PointId { value }
    }
}

/// Collection of *unique* points.
#[derive(Debug)]
pub struct UPoints {
    points_by_x: Vec<Point2D>,
    points_by_y: Vec<Point2D>,
}

impl UPoints {
    pub fn new(points: Vec<Point2D>) -> Self {
        let mut points_by_x = points.clone();
        points_by_x.sort_by(|p1, p2| p1.x().cmp(&p2.x()));
        let mut points_by_y = points;
        points_by_y.sort_by(|p1, p2| p1.y().cmp(&p2.y()));
        UPoints { points_by_x, points_by_y }
    }

//    pub fn get(&self, id: PointId) -> &Point2D {
//        &self.points[id.value]
//    }
}
