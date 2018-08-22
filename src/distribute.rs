use dims::{X, Y};
use dims::Dim;
use norms::Dist;
use norms::Norm;
use point::Point;
use point::Point2D;
use pointset::UPoints;
use rand::{Rng, StdRng};
use std::collections::HashSet;

pub fn generate_points(width: X, height: Y, count: usize, mut rng: StdRng) -> UPoints<Point2D> {
    assert!(width.euclidean_norm() * height.euclidean_norm() > Dist::fnew(2.0 * (count as f64)));
    let mut pointset = HashSet::<Point2D>::with_capacity(count);
    while pointset.len() < count {
        let point = Point2D::new(
            X::new(rng.gen_range(0, width._expose())),
            Y::new(rng.gen_range(0, height._expose())),
        );
        if !pointset.contains(&point) {
            pointset.insert(point);
        } else {}
    }
    UPoints::new(pointset.into_iter().collect())
}
