#![feature(extern_prelude)]

extern crate image;
extern crate rand;

use image::DynamicImage;
use rand::{Rng, SeedableRng, StdRng};
use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

struct Points {
    points: Vec<Point>,
}

fn make_grid(width: usize, height: usize) -> Vec<Vec<usize>> {
    vec![vec![0; width]; height]
}

fn generate_points(width: usize, height: usize, count: usize, mut rng: StdRng) -> Points {
    assert!(width * height > 2 * count);
    let mut pointset = HashSet::<Point>::with_capacity(count);
    while pointset.len() < count {
        let point = Point {
            x: rng.gen_range(0, width),
            y: rng.gen_range(0, height),
        };
        if !pointset.contains(&point) {
            pointset.insert(point);
        } else {
        }
    }
    Points { points: pointset.into_iter().collect() }
}

fn main() {
    // Load an image
    let pth = Path::new("resources").join("imgs").join("parrots.png");
    let dyn_img = image::open(pth).unwrap();
    if let DynamicImage::ImageRgb8(img) = dyn_img {
        // Get a random seed
        let mut rng: StdRng = SeedableRng::from_seed([154, 209, 215, 146, 162, 81, 13, 78, 243, 132, 107, 232, 61, 157, 71, 142, 202, 167, 65, 141, 113, 250, 202, 52, 46, 221, 141, 139, 22, 29, 183, 135]);
        let (width, height) = (img.width() as usize, img.height() as usize);
        let mut points = generate_points(width, height, (width * height) / 50, rng);
        // Write the output image
        let mut outpth = env::temp_dir().join("voronoi_gen.png");
        img.save(outpth.clone()).unwrap();
        Command::new("eog").arg(outpth).spawn().unwrap();
    } else {
        panic!("Wrong image type (maybe there is an alpha channel?)");
    }
}
