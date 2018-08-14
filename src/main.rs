#![feature(extern_prelude)]

extern crate image;
extern crate rand;

use image::DynamicImage;
use rand::{Rng, SeedableRng, StdRng};
use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::process::Command;
use std::ops::Mul;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct X(usize);

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Y(usize);

impl Mul<Y> for X {
    type Output = usize;

    fn mul(self, rhs: Y) -> <Self as Mul<Y>>::Output {
        self.0 * rhs.0
    }
}

impl Mul<X> for Y {
    type Output = usize;

    fn mul(self, rhs: X) -> <Self as Mul<X>>::Output {
        self.0 * rhs.0
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: X,
    y: Y,
}

struct Points {
    points: Vec<Point>,
}

struct PointId {
    val: usize,
}

impl Points {
    pub fn new(points: Vec<Point>) -> Self {
        Points { points }
    }

    pub fn get(&self, id: PointId) -> &Point {
        &self.points[id.val]
    }

    pub fn within(&self, left: X, right: X, top: Y, bottom: Y) -> &Vec<Point> {
        //TODO @mark:
        unimplemented!()
    }
}

fn make_grid(width: X, height: Y) -> Vec<Vec<usize>> {
    vec![vec![0; width.0]; height.0]
}

fn generate_points(width: X, height: Y, count: usize, mut rng: StdRng) -> Points {
    assert!(width * height > 2 * count);
    let mut pointset = HashSet::<Point>::with_capacity(count);
    while pointset.len() < count {
        let point = Point {
            x: X(rng.gen_range(0, width.0)),
            y: Y(rng.gen_range(0, height.0)),
        };
        if !pointset.contains(&point) {
            pointset.insert(point);
        } else {
        }
    }
    Points::new(pointset.into_iter().collect())
}

fn main() {
    // Load an image
    let pth = Path::new("resources").join("imgs").join("parrots.png");
    let dyn_img = image::open(pth).unwrap();
    if let DynamicImage::ImageRgb8(img) = dyn_img {
        // Get a random seed
        let mut rng: StdRng = SeedableRng::from_seed([154, 209, 215, 146, 162, 81, 13, 78, 243, 132, 107, 232, 61, 157, 71, 142, 202, 167, 65, 141, 113, 250, 202, 52, 46, 221, 141, 139, 22, 29, 183, 135]);
        let width = X(img.width() as usize);
        let height = Y(img.width() as usize);
        let mut points = generate_points(width, height, (width * height) / 50, rng);
        // Write the output image
        let mut outpth = env::temp_dir().join("voronoi_gen.png");
        img.save(outpth.clone()).unwrap();
        Command::new("eog").arg(outpth).spawn().unwrap();
    } else {
        panic!("Wrong image type (maybe there is an alpha channel?)");
    }
}
