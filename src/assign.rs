use dims::{X, Y};
use grouping::{Grouping, GroupingRow};
use norms::{Dist, Norm};
use parmap::par_map_on;
use point::{Point, Point2D};
use pointid::PointId;
use pointset::UPoints;
use scoped_pool::Pool;

/// This assigns the correct PointId to every single cell in `groups`.
pub fn assign_to_centers(centers: &mut UPoints, workers: &Pool) -> Grouping {
    debug_assert!(centers.len() > 0);

    let width = centers.width();
    let height = centers.height();

    // Keep both parallel and serial codes for benchmarking
    let use_parallel = false;
    let results = if use_parallel {
        par_map_on(
            workers,
            width.indices_upto(),
            |x: X| assign_to_centers_for_row(
                x, height, &centers
            ))
    } else {
        width.indices_upto().map(
            |x: X| assign_to_centers_for_row(
                x, height, &centers
        )).collect()
    };

    Grouping::from(width, height, results)
}
fn assign_to_centers_for_row(
    x: X,
    y_range: Y,
    centers: &UPoints,
) -> GroupingRow {
    // Performance: I thought it would be faster to recycle this output vector,
    // but (at least without parallelization), it is slightly faster to just recreate it.
    // It also makes parallelization easier, since this would otherwise be per-thread.
    let mut output_vec: Vec<PointId> = Vec::with_capacity(centers.len());
    let mut reference = centers.first_by_x();
    let mut links = Vec::with_capacity(y_range.as_index());
    for y in y_range.indices_upto() {
        let current: Point2D = Point2D::new(x, y);
        centers.within_box_noalloc(
            current,
            (current - reference).manhattan_norm() + Dist::fnew(1.),
            &mut output_vec,
        );
        // `output_vec` will contain the result of `within_box_noalloc`
        let nearest: PointId = find_nearest_to_reference(current, &mut output_vec, &centers);
        links.push(nearest);
        reference = centers.get(nearest);
    }
    GroupingRow::from(links, y_range)
}
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
    for center in candidates.iter() {
        let current_dist = (centers.get(*center) - point).euclidean_pseudo();
        if current_dist < smallest_dist {
            smallest_dist = current_dist;
            nearest_center = *center;
        }
    }
    nearest_center
}
