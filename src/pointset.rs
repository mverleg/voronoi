use crate::colorset::PointColorAverages;
use crate::dims::{X, Y};
use crate::find_index::find_index;
use crate::norms::Dist;
use crate::point::Point2D;
use crate::pointid::PointId;
use ::std::cmp::Ordering;
#[cfg(debug_assertions)]
use ::std::collections::HashSet;
#[cfg(debug_assertions)]
use ::std::iter::FromIterator;

/// Collection of *unique* points.
#[derive(Debug)]
pub struct UPoints {
    width: X,
    height: Y,
    points_by_x: Vec<Point2D>,
}

impl UPoints {

    pub fn new(width: X, height: Y, points: Vec<Point2D>) -> Self {
        let length = points.len();
        debug_assert!(length > 0);
        #[cfg(debug_assertions)]
        {
            // Hopefully this next line gets optimized away in production mode
            let unique_points = HashSet::<&Point2D>::from_iter(points.iter());
            debug_assert!(unique_points.len() == length);
            for point in points.clone() {
                debug_assert!(point.x() < width);
                debug_assert!(point.y() < height);
            }
        }
        let mut points_by_x = points;
        points_by_x.sort_by(|p1, p2| p1.x().cmp(&p2.x()));
        UPoints {
            width,
            height,
            points_by_x,
        }
    }

    pub fn width(&self) -> X {
        self.width
    }

    pub fn height(&self) -> Y {
        self.height
    }

    pub fn len(&self) -> usize {
        self.points_by_x.len()
    }

    pub fn new_color_averager(&self) -> PointColorAverages {
        PointColorAverages::new(self.len())
    }

    fn within_box_internal(&self, reference: Point2D, range: Dist, output_vec: &mut Vec<PointId>) {
        output_vec.clear();
        // Find any point within the range
        let urange = range.ufloor();
        let x_min = reference.x().saturating_sub(urange);
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
        if let Some(reference_index) = reference_index {
            let y_min = reference.y().saturating_sub(urange);
            let y_max = reference.y() + urange;
            let length = PointId::new(self.len());
            // Iterate backward from that point until range is exceeded (since points are ordered)
            let mut index = reference_index;
            let mut current = self.get(index);
            let x_min = reference.x().saturating_sub(urange);
            while current.x() >= x_min {
                if y_min <= current.y() && current.y() <= y_max {
                    debug_assert!(output_vec.len() <= self.points_by_x.len());
                    debug_assert!(output_vec.len() < output_vec.capacity());
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
    pub fn within_box_noalloc(
        &self,
        reference: Point2D,
        range: Dist,
        output_vec: &mut Vec<PointId>,
    ) {
        debug_assert!(output_vec.capacity() >= self.points_by_x.len());
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

    pub fn get(&self, id: PointId) -> Point2D {
        // This method was taking 19% of CPU when it used checked indexing.
        //rust: pub fn get(&self, id: PointId) -> Point2D {

        // Assembly when checked:
        //asm: push %rax
        //asm: mov 0x20(%rdi),%rdx
        //asm: cmp %rsi,%rdx
        //asm: jbe e328d <_ZN7vorolib8pointset7UPoints3get17h53098344e78bbbc3E+0x1d>
        //asm: mov 0x10(%rdi),%rcx
        //rust: self.points_by_x[id.as_index()]
        //asm: shl $0x4,%rsi
        //asm: mov (%rcx,%rsi,1),%rax
        //asm: mov 0x8(%rcx,%rsi,1),%rdx
        //rust: }
        //asm: pop %rcx
        //asm: retq
        //asm: lea 0x25361c(%rip),%rdi # 3368b0 <anon.3ad362cbc53c26fd2571867eaf5165d3.16.llvm.13471531638413625083>
        //asm: callq 50240 <_ZN4core9panicking18panic_bounds_check17h41b7398abc89de8fE>
        //asm: ud2
        //asm: nopl 0x0(%rax,%rax,1)

        //Assembly when unchecked
        //rust: unsafe { self.points_by_x.get_unchecked(id.as_index()) }.clone()
        //asm: shl $0x4,%rbx // shift bits left by 4
        //asm: mov (%rax,%rbx,1),%rbp
        //asm: mov 0x8(%rax,%rbx,1),%r13
        //asm: mov 0x50(%rsp),%rax
        //asm: mov %rax,%r14
        //asm: cmp 0x20(%rsp),%rax
        //asm: je 1e9d4 <_ZN7vorolib6assign25assign_to_centers_for_row17hed1ff5d1e9f58a4eE+0x1e4>
        //rust: step: (self.value as i32) - (other.value as i32),

        unsafe { self.points_by_x.get_unchecked(id.as_index()) }.clone()
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
    use crate::dims::{X, Y};
    use crate::distribute::generate_fixed_points;
    use ::std::collections::HashSet;
    use ::std::iter::FromIterator;

    #[test]
    fn test_within_one_eq() {
        let points: UPoints = generate_fixed_points(X::new(15), Y::new(15), 9);
        let matches = points.within_box(Point2D::from_raw(4, 4), Dist::fnew(3.0));
        assert_eq!(4, matches.len());
        let lookup: HashSet<Point2D> =
            HashSet::from_iter(matches.into_iter().map(|id| points.get(id)));
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
            HashSet::from_iter(matches.into_iter().map(|id| points.get(id)));
        assert!(lookup.contains(&Point2D::from_raw(2, 2)));
    }
}
