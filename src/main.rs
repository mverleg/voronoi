#![feature(extern_prelude)]

extern crate image;
extern crate rand;

pub mod dims;

use image::DynamicImage;
use rand::{Rng, SeedableRng, StdRng};
use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::process::Command;
use dims::{X, Y};
use dims::Count;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: X,
    y: Y,
}

struct Points {
    // The points must be unique.
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
    vec![vec![0; width._expose()]; height._expose()]
}

fn generate_points(width: X, height: Y, count: Count, mut rng: StdRng) -> Points {
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

fn main() {
    // Load an image
    let pth = Path::new("resources").join("imgs").join("parrots.png");
    let dyn_img = image::open(pth).unwrap();
    if let DynamicImage::ImageRgb8(img) = dyn_img {
        // Get a random seed
        let mut rng: StdRng = SeedableRng::from_seed([154, 209, 215, 146, 162, 81, 13, 78, 243, 132, 107, 232, 61, 157, 71, 142, 202, 167, 65, 141, 113, 250, 202, 52, 46, 221, 141, 139, 22, 29, 183, 135]);
        let width = X::new(img.width() as usize);
        let height = Y::new(img.width() as usize);
        let mut points = generate_points(width, height, (width * height) / 50, rng);
        // Write the output image
        let mut outpth = env::temp_dir().join("voronoi_gen.png");
        img.save(outpth.clone()).unwrap();
        //TODO @mark: turn on again:
        //Command::new("eog").arg(outpth).spawn().unwrap();
    } else {
        panic!("Wrong image type (maybe there is an alpha channel?)");
    }
}
