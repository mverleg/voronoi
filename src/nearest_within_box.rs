use crate::colorset::PointColorAverages;
use crate::dims::{dX, dY, X, Y};
use crate::find_index::find_index;
use crate::norms::Dist;
use crate::norms::Norm;
use crate::point::{Point2D, Step, Step2D};
use crate::pointid::PointId;
use crate::pointset::UPoints;
use std::cmp::Ordering;

struct CurrentMinimum {

}

/// Return the nearest center, searching only within a box.
// (Because returning a list if a waste of performance)
pub fn nearest_within_box(
    centers: &UPoints,
    reference: Point2D,
    max_range: Dist
) -> PointId {
    let urange = max_range.ufloor();
    let x_min = reference.x().saturating_sub(urange);
    let x_max = reference.x() + urange;
    let starting_index: PointId = find_index(
        PointId::new(0),
        PointId::new(centers.len() - 1),
        |index: PointId| {
            let x = centers.get(index).x();
            if x < x_min {
                return Ordering::Less;
            }
            if x > x_max {
                return Ordering::Greater;
            }
            Ordering::Equal
        },
    ).unwrap();

    let max_pseudo_dist = (reference - centers.get(starting_index)).euclidean_pseudo();
    let mut closest_center = (starting_index, max_pseudo_dist);
    let length = PointId::new(centers.len());

    // Iterate backward from that point until range is exceeded (since points are ordered)
    let mut index = starting_index;
    let mut current = centers.get(index);
    //TODO @mark: x_min could be narrowed faster
    while current.x() >= x_min {
        let pseudo_dist = (centers.get(index) - current).euclidean_pseudo();
        if pseudo_dist < closest_center.1 {
            closest_center = (index, pseudo_dist);
        }
        if index == PointId::new(0) {
            break;
        }
        index.decrement();
        current = centers.get(index);
    }

    // Iterate forward the same way
    index = starting_index + 1;
    if index >= length {
        return closest_center.0;
    }
    current = centers.get(index);
    //TODO @mark: x_max could be narrowed faster
    while current.x() <= x_max {
        let pseudo_dist = (centers.get(index) - current).euclidean_pseudo();
        if pseudo_dist < closest_center.1 {
            closest_center = (index, pseudo_dist);
        }
        index.increment();
        if index == length {
            break;
        }
        current = centers.get(index);
    }

    closest_center.0
}
