#![feature(extern_prelude)]

extern crate image;
extern crate rand;

use dims::{Dim, X, Y};
use distribute::generate_points;
use image::DynamicImage;
use rand::{SeedableRng, StdRng};
use std::env;
use std::path::Path;
use std::process::Command;

pub mod dims;
pub mod norms;
pub mod distribute;
pub mod points;

fn make_grid(width: X, height: Y) -> Vec<Vec<usize>> {
    vec![vec![0; width._expose() as usize]; height._expose() as usize]
}

fn main() {
    // Load an image
    let pth = Path::new("resources").join("imgs").join("parrots.png");
    let dyn_img = image::open(pth).unwrap();
    if let DynamicImage::ImageRgb8(img) = dyn_img {
        // Get a random seed
        let mut rng: StdRng = SeedableRng::from_seed([154, 209, 215, 146, 162, 81, 13, 78, 243, 132, 107, 232, 61, 157, 71, 142, 202, 167, 65, 141, 113, 250, 202, 52, 46, 221, 141, 139, 22, 29, 183, 135]);
        let width = X::new(img.width() as i32);
        let height = Y::new(img.width() as i32);
        let node_count: usize = (img.width() * img.height()) as usize / 50;
        let mut points = generate_points(width, height, node_count, rng);
        // Write the output image
        let mut outpth = env::temp_dir().join("voronoi_gen.png");
        img.save(outpth.clone()).unwrap();
        //TODO @mark: turn on again:
        //Command::new("eog").arg(outpth).spawn().unwrap();
    } else {
        panic!("Wrong image type (maybe there is an alpha channel?)");
    }
}
