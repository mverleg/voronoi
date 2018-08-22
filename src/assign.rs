use dims::{X, Y};
use point::{Point, Point2D};
use pointset::{PointId, UPoints};

//TODO @mark: is all this converting from usize to i32 expensive?

/// This assigns the correct PointId to every single cell in `groups`.
pub fn assign_to_centers(groups: Vec<Vec<PointId>>, centers: UPoints) -> Vec<Vec<PointId>> {
//    let mut x_i32: i32;
//    for (x, row) in groups.iter().enumerate() {
//        x_i32 = x as i32;
//        let mut reference = centers.first_by_x();
//        for (y, cell) in row.iter().enumerate() {
//            println!("{:?}", groups[x][y]);
//            let mut current: Point2D = Point2D::new(X::new(x_i32), Y::new(y as i32));
//            find_center(current, row_reference, &centers);
//            //TODO @mark: I am not clear on when to use points and ids
//            // reference = ...
//
//            pointset.within_box(...)
//
//        }
//    }
    unimplemented!();
    groups
}

fn find_center(position: Point2D, start_at: Point2D, centers: &UPoints) -> PointId {
    // There is a reference point `start_at`, so we need not search further than that distance.
    unimplemented!();
}
