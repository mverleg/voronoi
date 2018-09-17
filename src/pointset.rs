use colorset::PointColorAverages;
use find_index::find_index;
use norms::Dist;
use point::Point2D;
use pointid::PointId;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::FromIterator;

/// Collection of *unique* points.
#[derive(Debug)]
pub struct UPoints {
    points_by_x: Vec<Point2D>,
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
        UPoints { points_by_x }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.points_by_x.len()
    }

    pub fn new_color_averager(&self) -> PointColorAverages {
        PointColorAverages::new(self.len())
    }

    #[inline]
    fn within_box_internal(&self, reference: Point2D, range: Dist, output_vec: &mut Vec<PointId>) {
        output_vec.clear();
        // Find any point within the range
        let urange = range.ufloor();
        let x_min = reference.x() - urange;
        let x_max = reference.x() + urange;
        let reference_index: Option<PointId> = find_index(
            PointId::new(0),
            PointId::new(self.len() - 1),
            |index: PointId| {
                let x = self.get(index).x();
                if x < x_min {
                    return Ordering::Less;
                }
                if x > x_max {
                    return Ordering::Greater;
                }
                Ordering::Equal
            },
        );
        //TODO: parallellize forward and backward searching?
        if let Some(reference_index) = reference_index {
            let y_min = reference.y() - urange;
            let y_max = reference.y() + urange;
            let length = PointId::new(self.len());
            // Iterate backward from that point until range is exceeded (since points are ordered)
            let mut index = reference_index;
            let mut current = self.get(index);
            let x_min = reference.x() - urange;
            while current.x() >= x_min {
                if y_min <= current.y() && current.y() <= y_max {
                    debug_assert!(output_vec.len() <= self.points_by_x.len());
                    output_vec.push(index);
                }
                if index == PointId::new(0) {
                    break;
                }
                index.decrement();
                current = self.get(index);
            }
            // Iterate forward the same way
            index = reference_index + 1;
            if index < length {
                current = self.get(index);
                let x_max = reference.x() + urange;
                while current.x() <= x_max {
                    if y_min <= current.y() && current.y() <= y_max {
                        debug_assert!(output_vec.len() <= self.points_by_x.len());
                        output_vec.push(index);
                    }
                    index.increment();
                    if index == length {
                        break;
                    }
                    current = self.get(index);
                }
            }
        }
    }

    /// Return the index of app ponts within a square bounding box around `reference`, in arbitrary order.
    // Note that `output_vec` is used instead of return value to avoid allocating a vec for return value,
    // like in the good old Fortran days (and probably later). Use [within_box] if allocation is okay.
    #[inline]
    pub fn within_box_noalloc(
        &self,
        reference: Point2D,
        range: Dist,
        output_vec: &mut Vec<PointId>,
    ) {
        assert!(output_vec.capacity() >= self.points_by_x.len());
        self.within_box_internal(reference, range, output_vec);
    }

    /// Version of [within_box_noalloc] that does it's own allocation.
    pub fn within_box(&self, reference: Point2D, range: Dist) -> Vec<PointId> {
        let mut output_vec = Vec::with_capacity(self.len() / 8);
        self.within_box_internal(reference, range, &mut output_vec);
        output_vec
    }

    /// Get the first Point by X coordinate, or one of them if tied (somewhat arbitrary, which is acceptable)
    pub fn first_by_x(&self) -> Point2D {
        self.points_by_x[0]
    }

    #[inline]
    pub fn get(&self, id: PointId) -> Point2D {
        self.points_by_x[id._expose()]
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
    use super::*;
    use dims::{X, Y};
    use distribute::generate_fixed_points;

    #[test]
    fn test_within_one_eq() {
        let points: UPoints = generate_fixed_points(X::new(15), Y::new(15), 9);
        let matches = points.within_box(Point2D::from_raw(4, 4), Dist::fnew(3.0));
        assert_eq!(4, matches.len());
        let lookup: HashSet<Point2D> =
            HashSet::from_iter(matches.clone().into_iter().map(|id| points.get(id)));
        assert!(lookup.contains(&Point2D::from_raw(2, 2)));
        assert!(lookup.contains(&Point2D::from_raw(2, 7)));
        assert!(lookup.contains(&Point2D::from_raw(7, 2)));
        assert!(lookup.contains(&Point2D::from_raw(7, 7)));
    }

    #[test]
    fn test_within_one_lt() {
        let points: UPoints = generate_fixed_points(X::new(15), Y::new(15), 9);
        let matches = points.within_box(Point2D::from_raw(4, 4), Dist::fnew(2.0));
        assert_eq!(1, matches.len());
        let lookup: HashSet<Point2D> =
            HashSet::from_iter(matches.clone().into_iter().map(|id| points.get(id)));
        assert!(lookup.contains(&Point2D::from_raw(2, 2)));
    }
}
