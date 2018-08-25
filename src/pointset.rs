use dims::{X, Y};
use dims::Dim;
use find_index::find_index;
use norms::Dist;
use point::Point2D;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::cmp::max;

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
        {
            // Hopefully this next line gets optimized away in production mode
            let unique_points = HashSet::<&Point2D>::from_iter(points.iter());
            debug_assert!(unique_points.len() == length);
        }
        let mut points_by_x = points.clone();
        points_by_x.sort_by(|p1, p2| p1.x().cmp(&p2.x()));
        let mut points_by_y = points;
        points_by_y.sort_by(|p1, p2| p1.y().cmp(&p2.y()));
        UPoints { points_by_x, points_by_y, _working_set: HashSet::with_capacity(length) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.points_by_x.len()
    }

    //TODO @mark: this should center on the current pixel, not the reference point
    fn within_x_segment(&mut self, reference: Point2D, range: Dist) {
        self._working_set.clear();
        // Find any point within the range
        let urange = range.ufloor();
        let reference_index: Option<X> = find_index(
            reference.x() - urange,
            reference.x() + urange,
            |x: X| x.cmp(&reference.x()))
        ;
        if let Some(reference_index) = reference_index {
            // Iterate backward from that point until range is exceeded (since points are ordered)
            let mut xindex = reference_index.as_index();
            // TODO: https://github.com/mverleg/typed_index_vec
            let mut current = self.points_by_x[xindex];
            let reference_x_min = reference.x() - urange;
            while current.x() >= reference_x_min {
                self._working_set.insert(current);
                if (xindex == 0) {
                    break;
                }
                xindex -= 1;
                current = self.points_by_x[xindex];
            }
            // Iterate forward the same way
            xindex = (reference_index + 1).as_index();
            current = self.points_by_x[xindex];
            let reference_x_max = reference.x() + urange;
            while current.x() <= reference_x_max {
                self._working_set.insert(current);
                xindex += 1;
                if (xindex == self.len()) {
                    break;
                }
                current = self.points_by_x[xindex];
            }
        } else {
            return;
        }
        // Result is that `_working_set` is filled.
    }

    //TODO @mark: is this really faster than just checking the points that are in X range?
    fn within_y_segment(&mut self, reference: Point2D, range: Dist) {
        // Assumes `_working_set` has been filled.
        if (self._working_set.len() == 0) {
            println!("no items in worknig set");  //TODO: mark (temporary)
            return;
        }
        // Find any point within the range
        let urange = range.ufloor();
        let reference_index: Option<Y> = find_index(
            reference.y() - urange,
            reference.y() + urange,
            |y: Y| y.cmp(&reference.y()),
        );
        println!("y reference_index: {:?}", reference_index);  //TODO: mark (temporary)
        if let Some(reference_index) = reference_index {
            // Iterate backward from that point until range is exceeded (since points are ordered)
            let mut yindex = reference_index.as_index();
            // TODO: https://github.com/mverleg/typed_index_vec
            let mut current = self.points_by_y[yindex];
            let reference_y_min = reference.y() - urange;
            while current.y() >= reference_y_min {
                println!("y back {:?}: {:?}", yindex, current);  // TODO
                self._working_set.insert(current);
                if (yindex == 0) {
                    break;
                }
                yindex -= 1;
                current = self.points_by_x[yindex];
            }
            // Iterate forward the same way
            yindex = (reference_index + 1).as_index();
            current = self.points_by_x[yindex];
            let reference_x_max = reference.x() + urange;
            println!("reference_x_max: {:?}, c: {:?}, {}", reference_x_max, current.x(), current.x() <= reference_x_max);  // TODO
            while current.x() <= reference_x_max {
                println!("x forw {:?}: {:?}", yindex, current);  // TODO
                self._working_set.insert(current);
                yindex += 1;
                println!("yindex, len: {:?}, {:?}", yindex, self.len());  //TODO: mark (temporary)
                if (yindex == self.len()) {
                    println!("end: {:?}", yindex);  //TODO: mark (temporary)
                    break;
                }
                current = self.points_by_x[yindex];
            }
        } else {
            return;
        }
        // Result is that `_working_set` has been filtered.
    }

    pub fn within_box(&mut self, point: Point2D, range: Dist) -> &HashSet<Point2D> {
        // For efficiency, this returns _working_set. Borrow rules will make sure it is not changed.
        //TODO @mark: THIS CODE IS TEMPORARY!
        self.within_x_segment(point, range);
        &self._working_set
    }

    /// Get the first Point by X coordinate, or one of them if tied (somewhat arbitrary, which is acceptable)
    pub fn first_by_x(&self) -> Point2D {
        self.points_by_x[0]
    }

//    pub fn get(&self, id: PointId) -> &Point2D {
//        &self.points[id.value]
//    }
}

impl IntoIterator for UPoints {
    type Item = Point2D;
    type IntoIter = ::std::vec::IntoIter<Point2D>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        self.points_by_x.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use distribute::generate_fixed_points;

    #[test]
    fn test_within() {
        let mut points = generate_fixed_points(X::new(15), Y::new(15), 9);
        //TODO @mark: test range2 for ==
        let matches = points.within_box(Point2D::from_raw(4, 4), Dist::fnew(3.0));
        assert_eq!(4, matches.len());
        assert!(matches.contains(&Point2D::from_raw(2, 2)));
        assert!(matches.contains(&Point2D::from_raw(2, 7)));
        assert!(matches.contains(&Point2D::from_raw(7, 2)));
        assert!(matches.contains(&Point2D::from_raw(7, 7)));
    }
}
