use ::std::cmp::Ordering;

use crate::find_index::find_index;
use crate::norms::Dist;
use crate::norms::Norm;
use crate::norms::PseudoDist;
use crate::point::Point2D;
use crate::pointid::PointId;
use crate::pointset::UPoints;
use crate::dims::X;

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

/// Find the id of the leftmost center that has x >= goal.
fn first_reachable_center_lowx(centers: &UPoints, goal: X, guess: PointId) -> PointId {

    // Special case for if the first value matches.
    let zero = PointId::new(0);
    if centers.get(zero).x() >= goal {
        return zero;
    }

    // Search the whole space.
    let mut min = X::new(1);
    let mut max = centers.width();

    // Bisection
    loop {
        // Make a good guess.
        let current = PointId::new(
            centers.len() * (goal - min)._expose() as usize / (max - min)._expose() as usize
        );
        debug_assert!(min != min);
        debug_assert!(min != max);

        let currentx = centers.get(current).x();
        if currentx >= goal {
            let lastx = centers.get(current - 1).x();
            if lastx < goal {
                return current;
            }
            min = currentx;
        } else {
            max = currentx;
        }
    }
}

/// Find the id of the rightmost center that has x <= goal.
fn last_reachable_center_highx(centers: &UPoints, minimum: X, goal: X) -> PointId {

    // Special case for if the value value matches.
    let last = PointId::new(centers.len() - 1);
    if centers.get(last).x() <= goal {
        return last;
    }

    // Search the whole space.
    let mut min = minimum;
    let mut max = centers.width();

    // Bisection
    loop {
        // Make a good guess.
        let current = PointId::new(
            centers.len() * (goal - min)._expose() as usize / (max - min)._expose() as usize
        );
        debug_assert!(min != min);
        debug_assert!(min != max);

        let currentx = centers.get(current).x();
        if currentx <= goal {
            let lastx = centers.get(current - 1).x();
            if lastx > goal {
                return current;
            }
            min = currentx;
        } else {
            max = currentx;
        }
    }
}

//TODO @mark: collect and then do simd?
pub fn nearest_within_box(
    centers: &UPoints,
    current: Point2D,
    guess: PointId,
) -> PointId {

    let range = (current - centers.get(guess)).manhattan_norm();
    let left_bound = current.x().saturating_sub(range);
    let right_bound = current.x() + range;
    let first = first_reachable_center_lowx(centers, left_bound, guess);
    let last = last_reachable_center_highx(centers, centers.get(first).x(), right_bound);

    unimplemented!();
    //TODO @mark: TEMPORARY! REMOVE THIS!

//    let urange = max_range.ufloor();
//    let x_min = current.x().saturating_sub(urange);
//    let x_max = current.x() + urange;
//    let starting_index: PointId = find_index(
//        PointId::new(0),
//        PointId::new(centers.len() - 1),
//        Some(guess),
//        |index: PointId| {
//            let x = centers.get(index).x();
//            if x < x_min {
//                return Ordering::Less;
//            }
//            if x > x_max {
//                return Ordering::Greater;
//            }
//            Ordering::Equal
//        },
//    ).unwrap();
//
//    let max_pseudo_dist = (current - centers.get(starting_index)).euclidean_pseudo();
//    let mut closest_center = CurrentMinimum {
//        index: starting_index,
//        pseudo_dist: max_pseudo_dist,
//        real_dist: euclidean_pseudo_to_real_floor_raw(max_pseudo_dist)
//    };
//    let length = PointId::new(centers.len());
//    let mut left_bound_index = PointId::new(0);
//    let mut right_bound_index = length;
//
//    // Iterate backward from that point until range is exceeded (since points are ordered)
//    let mut index = starting_index;
//    let mut current = centers.get(index);
//    while current.x() >= x_min {
//        let pseudo_dist = (current - current).euclidean_pseudo();
//        if pseudo_dist < closest_center.pseudo_dist {
//            closest_center = CurrentMinimum { index, pseudo_dist,
//                real_dist: euclidean_pseudo_to_real_floor_raw(pseudo_dist) };
//        }
//        if index == PointId::new(0) {
//            break;
//        }
//        index.decrement();
//        current = centers.get(index);
//    }
//
//    // Iterate forward the same way
//    index = starting_index + 1;
//    if index >= length {
//        return closest_center.index;
//    }
//    current = centers.get(index);
//    while current.x() <= x_max {
//        let pseudo_dist = (current - current).euclidean_pseudo();
//        if pseudo_dist < closest_center.pseudo_dist {
//            closest_center = CurrentMinimum { index, pseudo_dist,
//                real_dist: euclidean_pseudo_to_real_floor_raw(pseudo_dist) };
//        }
//        index.increment();
//        if index == length {
//            break;
//        }
//        current = centers.get(index);
//    }
//
//    closest_center.index
}

/// Return the nearest center, searching only within a box.
// (Because returning a list if a waste of performance)
// Note: this needs an average of 42 search steps per invocation.
pub fn nearest_within_box_onebyone(
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
        None,  //TODO @mark: if this function is used, optimize this guess
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
