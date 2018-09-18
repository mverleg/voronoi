use dims::Dim;
use dims::{X, Y};
use norms::Dist;
use norms::Norm;
use point::Point2D;
use pointset::UPoints;
use rand::{Rng, StdRng};
use std::collections::HashSet;

/// Distribute points randomly.
pub fn generate_random_points(width: X, height: Y, count: usize, mut rng: StdRng) -> UPoints {
    assert!(width.euclidean_norm() * height.euclidean_norm() > Dist::fnew(2.0 * (count as f64)));
    let mut points = HashSet::<Point2D>::with_capacity(count);
    while points.len() < count {
        let point = Point2D::from_raw(
            rng.gen_range(0, width._expose()),
            rng.gen_range(0, height._expose()),
        );
        if !points.contains(&point) {
            points.insert(point);
        } else {
        }
    }
    UPoints::new(points.into_iter().collect())
}

/// Distribute points predictably.
pub fn generate_fixed_points(width: X, height: Y, count: usize) -> UPoints {
    assert!(width.euclidean_norm() * height.euclidean_norm() > Dist::fnew(9.0 * (count as f64)));
    let mut points = HashSet::<Point2D>::with_capacity(count);
    let count_per_dim = (count as f64).sqrt().ceil() as usize;
    for xi in 0..count_per_dim {
        let x = width.as_index() * (2 * xi + 1) / (2 * count_per_dim);
        for yi in 0..count_per_dim {
            let y = height.as_index() * (2 * yi + 1) / (2 * count_per_dim);
            points.insert(Point2D::from_raw(x as usize, y as usize));
        }
    }
    UPoints::new(points.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_equidistant() {
        let points = generate_fixed_points(X::new(15), Y::new(15), 9);
        assert_eq!(X::new(2), points.first_by_x().x());
        let lookup: HashSet<Point2D> = HashSet::from_iter(points.into_iter());
        assert!(lookup.contains(&Point2D::from_raw(2, 2)));
        assert!(lookup.contains(&Point2D::from_raw(2, 7)));
        assert!(lookup.contains(&Point2D::from_raw(2, 12)));
        assert!(lookup.contains(&Point2D::from_raw(7, 2)));
        assert!(lookup.contains(&Point2D::from_raw(7, 7)));
        assert!(lookup.contains(&Point2D::from_raw(7, 12)));
        assert!(lookup.contains(&Point2D::from_raw(12, 2)));
        assert!(lookup.contains(&Point2D::from_raw(12, 7)));
        assert!(lookup.contains(&Point2D::from_raw(12, 12)));
    }
}
