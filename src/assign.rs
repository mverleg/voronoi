use norms::{Norm, Dist};
use point::Point;
use point::Point2D;
use pointid::PointId;
use pointset::UPoints;

//TODO @mark: is all this converting from usize to i32 expensive?
//TODO @mark: inline every function used in inner loop
//TODO @mark: paralellize this stuff

/// This assigns the correct PointId to every single cell in `groups`.
pub fn assign_to_centers(groups: Vec<Vec<PointId>>, mut centers: UPoints) -> Vec<Vec<PointId>> {
    assert!(centers.len() > 0);
    let mut x_i32: i32;
    //TODO @mark: output_vec line once per thread:
    let mut output_vec: Vec<PointId> = Vec::with_capacity(centers.len());
    for (x, row) in groups.iter().enumerate() {
        assign_to_centers_for_row(x, row, &centers, &mut output_vec);
    }
    unimplemented!();
    groups
}

//TODO @mark: paralellize here
#[inline]
fn assign_to_centers_for_row(x: usize, row: &Vec<PointId>, centers: &UPoints, output_vec: &mut Vec<PointId>) {
    let x_i32 = x as i32;
    let mut reference = centers.first_by_x();
    for (y, cell) in row.iter().enumerate() {
        println!("@xy: {:?}", row[y]);
        let current: Point2D = Point2D::from_raw(x_i32, y as i32);
        centers.within_box_noalloc(
            current,
            (current - reference).manhattan_norm() + Dist::fnew(1.),
            output_vec
        );
        // `output_vec` will contain the result of `within_box_noalloc`
        let nearest: PointId = find_nearest_to_reference(current, output_vec, &centers);
        let reference = nearest;
    }
}

#[inline]
fn find_nearest_to_reference(point: Point2D, candidates: &Vec<PointId>, centers: &UPoints) -> PointId {
    assert!(centers.len() > 0, "There are no centers within the bounding box, which should never happen");
    //TODO @mark: this mutability thing is going to break the paralellism, maybe the cache can be stored thread-local?
    let mut nearest_center: PointId = candidates[0];
    let mut smallest_dist = (centers.get(nearest_center) - point).euclidean_pseudo();
    // //TODO @mark: does this [1:] have a performance cost? is it slower than repeating an element?
    for center in candidates.iter().skip(1) {
        let current_dist = (centers.get(nearest_center) - point).euclidean_pseudo();
        if current_dist < smallest_dist {
            smallest_dist = current_dist;
            nearest_center = *center;
        }
    }
    nearest_center
}
