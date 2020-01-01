use ::std::cmp::Ordering;

use crate::find_index::find_index;
use crate::norms::Dist;
use crate::norms::Norm;
use crate::norms::PseudoDist;
use crate::point::Point2D;
use crate::pointid::PointId;
use crate::pointset::UPoints;

struct CurrentMinimum {
    index: PointId,
    pseudo_dist: PseudoDist,
    real_dist: usize,
}

/// This only works for Euclidean, and removes distance type safety!
/// But sometimes hacks make things faster, so here we are.
fn euclidean_pseudo_to_real_floor_raw(pseudo: PseudoDist) -> usize {
    pseudo._expose().sqrt().floor().abs() as usize
}

/// Return the nearest center, searching only within a box.
// (Because returning a list if a waste of performance)
pub fn nearest_within_box(
    centers: &UPoints,
    reference: Point2D,
    max_range: Dist
) -> PointId {

    let urange = max_range.ufloor();
    let mut x_min = reference.x().saturating_sub(urange);
    let mut x_max = reference.x() + urange;
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
    let mut closest_center = CurrentMinimum {
        index: starting_index,
        pseudo_dist: max_pseudo_dist,
        real_dist: euclidean_pseudo_to_real_floor_raw(max_pseudo_dist)
    };
    let length = PointId::new(centers.len());

    // Iterate backward from that point until range is exceeded (since points are ordered)
    let mut index = starting_index;
    let mut current = centers.get(index);
    //TODO @mark: x_min could be narrowed faster
    while current.x() >= x_min {
        let pseudo_dist = (reference - current).euclidean_pseudo();
        if pseudo_dist < closest_center.pseudo_dist {
            closest_center = CurrentMinimum { index, pseudo_dist,
                real_dist: euclidean_pseudo_to_real_floor_raw(pseudo_dist) };
            x_min = reference.x().saturating_sub(closest_center.real_dist);
            x_max = reference.x() + closest_center.real_dist;
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
        return closest_center.index;
    }
    current = centers.get(index);
    //TODO @mark: x_max could be narrowed faster
    while current.x() <= x_max {
        let pseudo_dist = (reference - current).euclidean_pseudo();
        if pseudo_dist < closest_center.pseudo_dist {
            closest_center = CurrentMinimum { index, pseudo_dist,
                real_dist: euclidean_pseudo_to_real_floor_raw(pseudo_dist) };
            x_max = reference.x() + closest_center.real_dist;
        }
        index.increment();
        if index == length {
            break;
        }
        current = centers.get(index);
    }

    closest_center.index
}
