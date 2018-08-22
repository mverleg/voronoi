use find_index::find_index;
use norms::Dist;
use point::Point2D;
use std::collections::HashSet;

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
    _working_set: HashSet<Point2D>,
}

impl UPoints {
    pub fn new(points: Vec<Point2D>) -> Self {
        let length = points.len();
        assert!(length > 0);
        let mut points_by_x = points.clone();
        points_by_x.sort_by(|p1, p2| p1.x().cmp(&p2.x()));
        let mut points_by_y = points;
        points_by_y.sort_by(|p1, p2| p1.y().cmp(&p2.y()));
        UPoints { points_by_x, points_by_y, _working_set: HashSet::with_capacity(length) }
    }

    //TODO @mark: this should center on the current pixel, not the reference point
    fn within_x_segment(&mut self, reference: Point2D, centers: &UPoints, range: Dist) {
        self._working_set.clear();
        // Find any point within the range
        let reference_index = find_index(reference.x() - range.ufloor(), reference.x() + range.ufloor(), |p: Point2D| p.x().cmp(&reference.x()));
        if let None = reference_index {
            return
        }
        // Iterate backward from that point until range is exceeded (since points are ordered)
        let mut xindex = reference_index;
        let mut current = self.points_by_x[xindex];
        let reference_x_min = reference.x() - range;
        while current.x() >= reference_x_min {
            self._working_set.insert(current);
            xindex -= 1;
            current = self.points_by_x[xindex];
            println!("x forw {:?}: {:?}", xindex, current);
        }
        // Iterate forward the same way6
        xindex = reference_index;
        current = self.points_by_x[xindex + 1];
        let reference_x_max = reference.x() + range;
        while current.x() <= reference_x_max {
            self._working_set.insert(current);
            xindex += 1;
            current = self.points_by_x[xindex];
            println!("x back {:?}: {:?}", xindex, current);
        }
        // Result is that `_working_set` is filled.
    }

    fn within_y_segment() {
        // Assumes `_working_set` has been filled.



        // Result is that `_working_set` has been filtered.
    }

    pub fn within_box(point: Point2D, range: Dist) {

    }

    /// Get the first Point by X coordinate, or one of them if tied (somewhat arbitrary, which is acceptable)
    pub fn first_by_x(&self) -> Point2D {
        self.points_by_x[0]
    }

//    pub fn get(&self, id: PointId) -> &Point2D {
//        &self.points[id.value]
//    }
}
