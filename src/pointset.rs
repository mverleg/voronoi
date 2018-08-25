use dims::{X, Y};
use dims::Dim;
use find_index::find_index;
use norms::Dist;
use point::Point2D;
use std::cmp::max;
use std::collections::HashSet;
use std::iter::FromIterator;

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
    current_result: Vec<PointId>,
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
        let mut points_by_x = points;
        points_by_x.sort_by(|p1, p2| p1.x().cmp(&p2.x()));
        UPoints { points_by_x, current_result: Vec::with_capacity(length) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.points_by_x.len()
    }

    pub fn within_box(&mut self, reference: Point2D, range: Dist) -> &Vec<PointId> {
        // For efficiency, this returns current_result. This would be completely unsafe in most language,
        // but borrow rules will make sure it is not changed while in use in Rust.
        self.current_result.clear();
        // Find any point within the range
        let urange = range.ufloor();
        let reference_index: Option<X> = find_index(
            reference.x() - urange,
            reference.x() + urange,
            |x: X| x.cmp(&reference.x()))
        ;
        if let Some(reference_index) = reference_index {
            let y_min = reference.y() - urange;
            let y_max = reference.y() + urange;
            // Iterate backward from that point until range is exceeded (since points are ordered)
            let mut index = reference_index.as_index();
            // TODO: https://github.com/mverleg/typed_index_vec
            let mut current = self.points_by_x[index];
            let x_min = reference.x() - urange;
            while current.x() >= x_min {
                if (y_min <= current.y() && current.y() <= y_max) {
                    self.current_result.push(PointId::new(index));
                }
                if (index == 0) {
                    break;
                }
                index -= 1;
                current = self.points_by_x[index];
            }
            // Iterate forward the same way
            index = (reference_index + 1).as_index();
            current = self.points_by_x[index];
            let x_max = reference.x() + urange;
            while current.x() <= x_max {
                if (y_min <= current.y() && current.y() <= y_max) {
                    self.current_result.push(PointId::new(index));
                }
                index += 1;
                if (index == self.len()) {
                    break;
                }
                current = self.points_by_x[index];
            }
        }
        &self.current_result
    }

    /// Get the first Point by X coordinate, or one of them if tied (somewhat arbitrary, which is acceptable)
    pub fn first_by_x(&self) -> Point2D {
        self.points_by_x[0]
    }

    pub fn get(&self, id: PointId) -> Point2D {
        self.points_by_x[id.value]
    }
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
    use distribute::generate_fixed_points;
    use super::*;

    #[test]
    fn test_within_one_eq() {
        let mut points: UPoints = generate_fixed_points(X::new(15), Y::new(15), 9);
        let matches: &Vec<PointId> = points.within_box(Point2D::from_raw(4, 4), Dist::fnew(3.0));
        assert_eq!(4, matches.len());
        let lookup: HashSet<Point2D> = HashSet::from_iter(matches.clone().into_iter().map(|id| points.get(id)));
        assert!(lookup.contains(&Point2D::from_raw(2, 2)));
        assert!(lookup.contains(&Point2D::from_raw(2, 7)));
        assert!(lookup.contains(&Point2D::from_raw(7, 2)));
        assert!(lookup.contains(&Point2D::from_raw(7, 7)));
    }

    #[test]
    fn test_within_one_lt() {
        let mut points: UPoints = generate_fixed_points(X::new(15), Y::new(15), 9);
        let matches: &Vec<PointId> = points.within_box(Point2D::from_raw(4, 4), Dist::fnew(2.0));
        println!("matches: {:?}", matches);  //TODO: mark (temporary)
        assert_eq!(1, matches.len());
        let lookup: HashSet<Point2D> = HashSet::from_iter(matches.clone().into_iter().map(|id| points.get(id)));
        assert!(lookup.contains(&Point2D::from_raw(2, 2)));
    }
}
