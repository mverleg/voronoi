use dims::{X, Y};
use dims::Count;
use points::Points;
use rand::{Rng, StdRng};
use std::collections::HashSet;
use points::Point;

pub fn generate_points(width: X, height: Y, count: Count, mut rng: StdRng) -> Points {
    assert!(width * height > count * 2);
    let mut pointset = HashSet::<Point>::with_capacity(count._expose());
    while pointset.len() < count._expose() {
        let point = Point {
            x: X::new(rng.gen_range(0, width._expose())),
            y: Y::new(rng.gen_range(0, height._expose())),
        };
        if !pointset.contains(&point) {
            pointset.insert(point);
        } else {
        }
    }
    Points::new(pointset.into_iter().collect())
}
