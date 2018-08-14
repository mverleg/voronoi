#![feature(extern_prelude)]

extern crate image;

use std::path::Path;
use std::env;
use std::process::Command;
use image::DynamicImage;

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

fn generate_points(width: usize, height: usize, count: usize) -> Points {
    Points { points: vec![] }
}

fn main() {
    let pth = Path::new("resources").join("imgs").join("parrots.png");
    let dyn_img = image::open(pth).unwrap();
    if let DynamicImage::ImageRgb8(img) = dyn_img {
        let mut outpth = env::temp_dir().join("voronoi_gen.png");
        img.save(outpth.clone()).unwrap();
        Command::new("eog").arg(outpth).spawn().unwrap();
    } else {
        panic!("Wrong image type (maybe there is an alpha channel?)");
    }
}
