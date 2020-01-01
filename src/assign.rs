use ::scoped_pool::Pool;

use crate::dims::{X, Y};
use crate::grouping::{Grouping, GroupingRow};
use crate::nearest_within_box::nearest_within_box;
use crate::norms::Norm;
use crate::parmap::par_map_on;
use crate::point::{Point, Point2D};
use crate::pointid::PointId;
use crate::pointset::UPoints;

/// This assigns the correct PointId to every single cell in `groups`.
#[cfg_attr(feature = "flame_it", flame)]
pub fn assign_to_centers(centers: &mut UPoints, workers: &Pool) -> Grouping {
    debug_assert!(centers.len() > 0);

    let width = centers.width();
    let height = centers.height();

    // Keep both parallel and serial codes for benchmarking
    let use_parallel = true;
    let results = if use_parallel {
        par_map_on(workers, width.indices_upto(), |x: X| {
            assign_to_centers_for_row(x, height, &centers)
        })
    } else {
        width
            .indices_upto()
            .map(|x: X| assign_to_centers_for_row(x, height, &centers))
            .collect()
    };

    Grouping::from(width, height, results)
}

fn assign_to_centers_for_row(x: X, y_range: Y, centers: &UPoints) -> GroupingRow {
    // Guess the point id based on them being homogeneous and ordered.
    let mut reference = PointId::new(((x.as_index() * centers.len()) as f64 / centers.width().as_index() as f64) as usize);
    let mut center_assignments = Vec::with_capacity(y_range.as_index());
    for y in y_range.indices_upto() {
        let current: Point2D = Point2D::new(x, y);
        let nearest: PointId = nearest_within_box(&centers, current, reference);
        center_assignments.push(nearest);
        reference = nearest;
    }
    GroupingRow::from(center_assignments, y_range)
}

#[allow(unused)]  // Note: probably unused, superseded by `nearest_within_box`.
fn find_nearest_to_reference(point: Point2D, candidates: &[PointId], centers: &UPoints) -> PointId {

    debug_assert!(
        centers.len() > 0,
        "There are no centers within the bounding box, which should never happen"
    );
    let mut nearest_center: PointId = candidates[0];
    let mut smallest_dist = (centers.get(nearest_center) - point).euclidean_pseudo();
    for center in candidates.iter().cloned() {
        let current_dist = (centers.get(center) - point).euclidean_pseudo();
        if current_dist < smallest_dist {
            smallest_dist = current_dist;
            nearest_center = center;
        }
    }
    nearest_center
}
