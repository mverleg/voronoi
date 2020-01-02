use ::std::cmp::Ordering;

use crate::find_index::find_index;
use crate::norms::Dist;
use crate::norms::Norm;
use crate::norms::PseudoDist;
use crate::point::Point2D;
use crate::pointid::PointId;
use crate::pointset::UPoints;
use crate::dims::X;
use crate::util::crop;

struct CurrentMinimum {
    index: PointId,
    pseudo_dist: PseudoDist,
    real_dist: usize,
}

//TODO @mark: rename?
struct CurrentMinimumNR {
    index: PointId,
    pseudo_dist: PseudoDist,
}

struct Border {
    index: PointId,
    x: X,
}

/// This only works for Euclidean, and removes distance type safety!
/// But sometimes hacks make things faster, so here we are.
fn euclidean_pseudo_to_real_floor_raw(pseudo: PseudoDist) -> usize {
    pseudo._expose().sqrt().floor().abs() as usize
}

/// Find the id of the leftmost center that has x >= goal.
fn first_reachable_center_lowx(centers: &UPoints, goal: X, guess: PointId) -> PointId {

    // Special case for if the first value matches.
    let (first_index, last_index) = (PointId::new(0), PointId::new(centers.len()));
    if centers.get(first_index).x() >= goal {
        return first_index;
    }

    // Search the whole space.
    let mut min = X::new(0);
    let mut max = centers.width();
    debug_assert!(goal > min);
    debug_assert!(goal < max);

    // Bisection.
    let mut i = 0;  //TODO @mark: TEMPORARY! REMOVE THIS!
    loop {
        i += 1;
        // Make a good guess.
//        let current = crop(
//            PointId::new(
//                (centers.len() as f64 *
//                    (goal - min)._expose() as f64 /
//                    (max - min)._expose() as f64) as usize,
//            ),
//            first_index, last_index  //TODO @mark: but the problem is with `x` not `id`
//        );
        let current = PointId::new(
            (centers.len() as f64 *
                (goal - min)._expose() as f64 /
                (max - min)._expose() as f64) as usize
        );
        println!(">>>> {}: {:?} / {:?} = {:?} [{:?}, {:?}, {:?}]", i, (goal - min)._expose(), (max - min)._expose(),
                 (goal - min)._expose() as f64 / (max - min)._expose() as f64,
                 goal, min, max);  //TODO @mark: TEMPORARY! REMOVE THIS!
        debug_assert!(current >= first_index);
        debug_assert!(current <= last_index);
        println!(">>> {:?} < {:?} < {:?}", min, centers.get(current).x(), max);  //TODO @mark: TEMPORARY! REMOVE THIS!
        //TODO @mark: I think == is possible, there may be multiple centers with the same x coordinate
        debug_assert!(centers.get(current).x() >= min);
        debug_assert!(centers.get(current).x() <= max);

        // Return if a match, adjust bounds otherwise.
        let current_x = centers.get(current).x();
        if current_x >= goal {
            let prev_x = centers.get(current - 1).x();
            if prev_x < goal {
                return current;
            }
            min = current_x;
        } else {
            max = current_x;
        }
    }
}

/// Find the id of the rightmost center that has x <= goal.
fn last_reachable_center_highx(centers: &UPoints, minimum: X, goal: X) -> PointId {
    dbg!(goal);  //TODO @mark:
    dbg!(minimum);  //TODO @mark:
    debug_assert!(goal >= minimum);

    // Special case for if the value value matches.
    let (first_index, last_index) = (PointId::new(0), PointId::new(centers.len() - 1));
    if centers.get(last_index).x() <= goal {
        return last_index;
    }

    // Search the whole space.
    let mut min = Border { index: first_index, x: minimum };
    let mut max = Border { index: last_index, x: centers.width() };
    debug_assert!(goal > min.x);
    debug_assert!(goal < max.x);

    // Bisection.
    let mut i = 0;  //TODO @mark: TEMPORARY! REMOVE THIS!
    let mut prev_iter = None;
    loop {
        i += 1;
        // Make a good guess.
        let current = PointId::new(
            (centers.len() as f64 *
                (goal - min.x)._expose() as f64 /
                (max.x - min.x)._expose() as f64) as usize
        );
        println!(">> i = {:?}", i);
        println!("  goal = {:?}", goal);
        println!("  min = {:?}", min.x);
        println!("  max = {:?}", max.x);
        println!("  current = {:?}", current);
        println!("  centers->x = {:?}", centers.get(current).x());
        let current = crop(current, min.index + 1, max.index - 1);
//        while centers.get(current).x() < min {
//            println!("PERFORMANCE HIT! +1");
//            current.increment();
//        }
//        while centers.get(current).x() > max {
//            println!("PERFORMANCE HIT! -1");
//            current.decrement();
//        }
        println!("  current = {:?}", current);
        println!("  centers->x = {:?}", centers.get(current).x());
        println!(">>>>+ {}: {:?} / {:?} = {:?} -> {:?} [{:?}, {:?}, {:?}]", i, (goal - min.x)._expose(), (max.x - min.x)._expose(),
                 (goal - min.x)._expose() as f64 / (max.x - min.x)._expose() as f64,
                 current.as_index(), goal, min.x, max.x);  //TODO @mark: TEMPORARY! REMOVE THIS!
        debug_assert!(current >= first_index);
        debug_assert!(current <= last_index);
        println!(">>> {:?} < {:?} < {:?}", min.x, centers.get(current).x(), max.x);  //TODO @mark: TEMPORARY! REMOVE THIS!
        //TODO @mark: I think == is possible, there may be multiple centers with the same x coordinate
//        debug_assert!(centers.get(current).x() >= min);
//        debug_assert!(centers.get(current).x() <= max);
        if let Some(v) = prev_iter {
            debug_assert!(v != current);
        }

        // Return if a match, adjust bounds otherwise.
        let current_x = centers.get(current).x();
        if current_x <= goal {
            let next_x = centers.get(current + 1).x();
            if next_x > goal {
                return current;
            }
            min = Border { index: current, x: current_x };
        } else {
            max = Border { index: current, x: current_x };
        }
        prev_iter = Some(current);
    }
}

//TODO @mark: collect and then do simd?
pub fn nearest_within_box(
    centers: &UPoints,
    reference: Point2D,
    guess: PointId,
) -> PointId {

    //TODO @mark: is this +1 needed?
    let range = (reference - centers.get(guess)).manhattan_norm().ufloor();
    let left_bound = reference.x().saturating_sub(range);
    let right_bound = reference.x() + range;
    let first = first_reachable_center_lowx(centers, left_bound, guess);
    let last = last_reachable_center_highx(centers, centers.get(first).x(), right_bound);

    let mut current = first;
    let mut closest_center = CurrentMinimumNR {
        index: first,
        pseudo_dist: (reference - centers.get(first)).euclidean_pseudo()
    };

    current.increment();
    while current != last && current.as_index() < centers.len() {

        //TODO @mark: SIMD this?

        let dist = (reference - centers.get(current)).euclidean_pseudo();
        if dist < closest_center.pseudo_dist {
            closest_center = CurrentMinimumNR {
                index: current,
                pseudo_dist: dist
            };
        }

        current.increment();
    }

    closest_center.index

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
