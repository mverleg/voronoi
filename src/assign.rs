use norms::{Norm, Dist};
use point::Point2D;
use pointid::PointId;
use pointset::UPoints;
use grouping::Grouping;

//TODO @mark: is all this converting from usize to i32 expensive?
//TODO @mark: inline every function used in inner loop

/// This assigns the correct PointId to every single cell in `groups`.
pub fn assign_to_centers(mut groups: Grouping, centers: UPoints) -> Grouping {
    assert!(centers.len() > 0);
    //TODO @mark: output_vec line once per thread:
    let mut output_vec: Vec<PointId> = Vec::with_capacity(centers.len());
    for (x, row) in groups.iter_mut().enumerate() {
        assign_to_centers_for_row(x, row, &centers, &mut output_vec);
    }
    groups
}

//TODO @mark: paralellize here
#[inline]
fn assign_to_centers_for_row(x: usize, row: &mut Vec<PointId>, centers: &UPoints, output_vec: &mut Vec<PointId>) {
    let mut reference = centers.first_by_x();
    let x_i32 = x as i32;
    for y in 0 .. row.len() {
        let current: Point2D = Point2D::from_raw(x_i32, y as i32);
//        println!("distance: {:?}", (current - reference).manhattan_norm() + Dist::fnew(1.));  //TODO: mark (temporary)
        centers.within_box_noalloc(
            current,
            (current - reference).manhattan_norm() + Dist::fnew(1.),
            output_vec
        );
        // `output_vec` will contain the result of `within_box_noalloc`
        let nearest: PointId = find_nearest_to_reference(current, output_vec, &centers);
        row[y] = nearest;
        reference = centers.get(nearest);
    }
}

#[inline]
fn find_nearest_to_reference(point: Point2D, candidates: &Vec<PointId>, centers: &UPoints) -> PointId {
    assert!(centers.len() > 0, "There are no centers within the bounding box, which should never happen");
    let mut nearest_center: PointId = candidates[0];
    let mut smallest_dist = (centers.get(nearest_center) - point).euclidean_pseudo();
    //TODO @mark: is this 'skip' faster than just repeating an element?
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
