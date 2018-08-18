use dims::{X, Y};
use points::UPoints;
use rand::{Rng, StdRng};
use std::collections::HashSet;
use points::Point;
use points::Point2D;

pub fn generate_points(width: X, height: Y, count: usize, mut rng: StdRng) -> UPoints<Point2D> {
    assert!(width.euclidean_norm() * height.euclidean_norm() > count * 2);
    let mut pointset = HashSet::<Point>::with_capacity(count._expose());
    while pointset.len() < count._expose() {
        let point = Point2D {
            x: X::new(rng.gen_range(0, width._expose())),
            y: Y::new(rng.gen_range(0, height._expose())),
        };
        if !pointset.contains(&point) {
            pointset.insert(point);
        } else {}
    }
    UPoints::new(pointset.into_iter().collect())
}
