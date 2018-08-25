use dims::{X, Y};
use dims::Dim;
use find_index::find_index;
use norms::Dist;
use point::Point2D;
use std::cmp::max;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::cmp::Ordering;
use std::ops::{Add, Sub};
use find_index::Mid;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub struct PointId {
    value: usize,
}

impl PointId {
    pub fn new(value: usize) -> Self {
        PointId { value }
    }
}

impl Add<Self> for PointId {
    type Output = Self;

    fn add(self, rhs: PointId) -> <Self as Add<PointId>>::Output {
        PointId::new(self.value + rhs.value)
    }
}
//
//impl Sub<Self> for PointId {
//    type Output = Self;
//
//    fn sub(self, rhs: PointId) -> <Self as Add<PointId>>::Output {
//        if (rhs.value > self.value) {
//            panic!("PointId cannot be negative");
//        }
//        PointId::new(self.value - rhs.value)
//    }
//}
//
impl Add<usize> for PointId {
    type Output = Self;

    fn add(self, rhs: usize) -> <Self as Add<PointId>>::Output {
        PointId::new(self.value + rhs)
    }
}

impl Sub<usize> for PointId {
    type Output = Self;

    fn sub(self, rhs: usize) -> <Self as Add<PointId>>::Output {
        if (rhs > self.value) {
            panic!("PointId cannot be negative");
        }
        PointId::new(self.value - rhs)
    }
}

impl Mid for PointId {
    fn midpoint(first: Self, second: Self) -> Self {
        PointId::new((first.value + second.value) / 2)
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

    //TODO @mark: split within_box into separate methods
//    #[inline]
//    fn find_initial_point() {
//    }

    /// Return the index of app ponts within a square bounding box around `reference`, in arbitrary order.
    pub fn within_box(&mut self, reference: Point2D, range: Dist) -> &Vec<PointId> {
        // For efficiency, this returns current_result. This would be completely unsafe in most language,
        // but borrow rules will make sure it is not changed while in use in Rust.
        self.current_result.clear();
        // Find any point within the range
        let urange = range.ufloor();
        let reference_index: Option<PointId> = find_index(
            PointId::new(0),
            PointId::new(self.len() - 1),
            |index: PointId| {
                let x = self.get(index).x();
                if x < reference.x() - urange {
                    return Ordering::Greater
                }
                if x > reference.x() + urange {
                    return Ordering::Less
                }
                Ordering::Equal
            }
        );
        //TODO @mark: parallellize forward and backward searching?
        if let Some(reference_index) = reference_index {
            let y_min = reference.y() - urange;
            let y_max = reference.y() + urange;
            // Iterate backward from that point until range is exceeded (since points are ordered)
            let mut index = reference_index;
            // TODO: https://github.com/mverleg/typed_index_vec
            let mut current = self.get(index);
            println!("reference_index: {:?} = {:?}", reference_index, current);  //TODO: mark (temporary)
            let x_min = reference.x() - urange;
            while current.x() >= x_min {
                println!("back visit: {:?}, {:?}", index, current.x());  //TODO: mark (temporary)
                if (y_min <= current.y() && current.y() <= y_max) {
                    println!("pick x: {:?}", current);  //TODO: mark (temporary)
                    self.current_result.push(PointId::new(index));
                }
                if index == 0 {
                    break;
                }
                index -= 1;
                current = self.points_by_x[index];
            }
            // Iterate forward the same way
            index = (reference_index + 1).as_index();
            if (index < self.len()) {
                current = self.points_by_x[index];
                let x_max = reference.x() + urange;
                println!("x <= x_max: {:?} <= {:?}", current.x(), x_max);  //TODO: mark (temporary)
                while current.x() <= x_max {
                    println!("forw visit: {:?}, {:?}", index, current.x());  //TODO: mark (temporary)
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

//    #[test]
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
        println!("START");  //TODO: mark (temporary)
        let mut points: UPoints = generate_fixed_points(X::new(15), Y::new(15), 9);
        let matches: &Vec<PointId> = points.within_box(Point2D::from_raw(4, 4), Dist::fnew(2.0));
        println!("matches: {:?}", matches);  //TODO: mark (temporary)
        assert_eq!(1, matches.len());
        let lookup: HashSet<Point2D> = HashSet::from_iter(matches.clone().into_iter().map(|id| points.get(id)));
        assert!(lookup.contains(&Point2D::from_raw(2, 2)));
        println!("END");  //TODO: mark (temporary)
    }
}
