#![feature(extern_prelude)]
#![feature(nll)]

#[macro_use]
extern crate assert_approx_eq;
extern crate image;
extern crate rand;
extern crate threadpool;

use assign::assign_to_centers;
use dims::{Dim, X, Y};
use distribute::generate_random_points;
use image::DynamicImage;
use pointid::PointId;
use rand::{SeedableRng, StdRng};
use std::env;
use std::path::Path;
#[allow(unused_imports)]
use std::process::Command;

pub mod assign;
pub mod dims;
pub mod distribute;
pub mod find_index;
pub mod norms;
pub mod point;
pub mod pointid;
pub mod pointset;
pub mod regions;

//TODO @mark: should this be a dedicated matrix structure rather than just Vec<Vec<.>> ?
fn make_grid(width: X, height: Y) -> Vec<Vec<PointId>> {
    vec![vec![PointId::new(0); width._expose() as usize]; height._expose() as usize]
}

fn main() {
    // Load an image
    let pth = Path::new("resources").join("imgs").join("parrots.png");
    let dyn_img = image::open(pth).unwrap();
    if let DynamicImage::ImageRgb8(img) = dyn_img {
        // Get a random seed and generate points.
        let mut rng: StdRng = SeedableRng::from_seed([
            154, 209, 215, 146, 162, 81, 13, 78, 243, 132, 107, 232, 61, 157, 71, 142, 202, 167,
            65, 141, 113, 250, 202, 52, 46, 221, 141, 139, 22, 29, 183, 135,
        ]);
        let width = X::new(img.width() as i32);
        let height = Y::new(img.height() as i32);
        let node_count: usize = (img.width() * img.height()) as usize / 50;
        let points = generate_random_points(width, height, node_count, rng);
        // Assign all pixels to the nearest center.
        let pixel_group = make_grid(width, height);
        assign_to_centers(pixel_group, points);
        // Write the output image
        let outpth = env::temp_dir().join("voronoi_gen.png");
        img.save(outpth.clone()).unwrap();
    //TODO @mark: turn on again:
    //        Command::new("eog").arg(outpth).spawn().unwrap();
    } else {
        panic!("Wrong image type (maybe there is an alpha channel?)");
    }
}
