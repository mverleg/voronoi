use grouping::Grouping;
use norms::{Dist, Norm};
use point::{Point, Point2D};
use pointid::PointId;
use pointset::UPoints;
use threadpool::ThreadPool;
use grouping::GroupingRow;
use dims::X;

//TODO @mark: inline every function used in inner loop

/// This assigns the correct PointId to every single cell in `groups`.
pub fn assign_to_centers(groups: Grouping, centers: &mut UPoints, workers: &ThreadPool) -> Grouping {
    //TODO @mark: @opt=1 this is 93%
    debug_assert!(centers.len() > 0);
    //TODO @mark: I must either limit the borrow scope (no idea how), or I need to split groups and reassemble it after processing
    for (x, row) in groups.into_iter().enumerate() {
//        assign_to_centers_for_row(x, row, &centers);
        workers.execute(|| assign_to_centers_for_row(X::new(x), row, &centers));
    }
//    groups //TODO @mark:
    unimplemented!(); //TODO @mark: THIS CODE IS TEMPORARY!
}

//TODO @mark: paralellize here?
#[inline]
fn assign_to_centers_for_row(
    x: X,
    mut row: GroupingRow,
    centers: &UPoints,
) {
    // Performance: I thought it would be faster to recycle this output vector,
    // but (at least without parallelization), it is slightly faster to just recreate it.
    // It also makes parallelization easier, since this would otherwise be per-thread.
    let mut output_vec: Vec<PointId> = Vec::with_capacity(centers.len());
    let mut reference = centers.first_by_x();
    for y in row.indices() {
        let current: Point2D = Point2D::new(x, y);
        centers.within_box_noalloc(
            current,
            (current - reference).manhattan_norm() + Dist::fnew(1.),
            &mut output_vec,
        );
        // `output_vec` will contain the result of `within_box_noalloc`
        let nearest: PointId = find_nearest_to_reference(current, &mut output_vec, &centers);
        row[y] = nearest;
        //TODO @mark: index
        reference = centers.get(nearest);
    }
    row; //TODO @mark: send this to a channel
}

#[inline]
fn find_nearest_to_reference(
    point: Point2D,
    candidates: &Vec<PointId>,
    centers: &UPoints,
) -> PointId {
    debug_assert!(
        centers.len() > 0,
        "There are no centers within the bounding box, which should never happen"
    );
    let mut nearest_center: PointId = candidates[0];
    let mut smallest_dist = (centers.get(nearest_center) - point).euclidean_pseudo();
    //TODO @mark: is this 'skip(1)' faster than just repeating an element?
    //TODO @mark: want to not use *, iterate over copies...
    for center in candidates.iter().skip(1) {
        let current_dist = (centers.get(*center) - point).euclidean_pseudo();
        if current_dist < smallest_dist {
            smallest_dist = current_dist;
            nearest_center = *center;
        }
    }
    nearest_center
}
