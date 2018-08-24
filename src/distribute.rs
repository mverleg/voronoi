use dims::{X, Y};
use dims::Dim;
use norms::Dist;
use norms::Norm;
use point::Point;
use point::Point2D;
use pointset::UPoints;
use rand::{Rng, StdRng};
use std::collections::HashSet;

/// Distribute points randomly.
pub fn generate_random_points(width: X, height: Y, count: usize, mut rng: StdRng) -> UPoints {
    assert!(width.euclidean_norm() * height.euclidean_norm() > Dist::fnew(2.0 * (count as f64)));
    let mut points = HashSet::<Point2D>::with_capacity(count);
    while points.len() < count {
        let point = Point2D::new(
            X::new(rng.gen_range(0, width._expose())),
            Y::new(rng.gen_range(0, height._expose())),
        );
        if !points.contains(&point) {
            points.insert(point);
        } else {}
    }
    UPoints::new(points.into_iter().collect())
}


/// Distribute points predictably.
pub fn generate_fixed_points(width: X, height: Y, count: usize) -> UPoints {
    assert!(width.euclidean_norm() * height.euclidean_norm() > Dist::fnew(9.0 * (count as f64)));
    let mut points = HashSet::<Point2D>::with_capacity(count);
    let count_per_dim = (count as f64).sqrt().ceil() as usize;
    for xi in 0 .. count_per_dim {
        let x = width * (2 * xi + 1) / (2 * count_per_dim);
        for yi in 0 .. count_per_dim {
            let y = height * (2 * yi + 1) / (2 * count_per_dim);
            points.push(Point2D::new(
                X::new(x),
                Y::new(y),
            ));
        }
    }
    UPoints::new(points.into_iter().collect())
}
