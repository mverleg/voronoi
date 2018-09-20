#![feature(extern_prelude)]
#![feature(nll)]

extern crate clap;
extern crate image;
extern crate rand;
extern crate threadpool;

use assign::assign_to_centers;
use clap::App;
use clap::Arg;
use distribute::generate_random_points;
use grouping::Grouping;
use img::Img;
use paint::pixel_to_group_colors;
use rand::{SeedableRng, StdRng};
use std::env;
use std::path::Path;
#[allow(unused_imports)]
use std::process::Command;
use std::process::exit;
use argparse::parse_args;

#[macro_use]
pub mod test_util;

pub mod assign;
pub mod color;
pub mod colorset;
pub mod dims;
pub mod distribute;
pub mod find_index;
pub mod grouping;
pub mod img;
pub mod norms;
pub mod paint;
pub mod point;
pub mod pointid;
pub mod pointset;
pub mod argparse;

//TODO @mark: find a way to turn of all asserts in optimized mode? => or just convert the hot-loop-ones to debug_assert and keep the rest

fn main() {
    let (input, output, count, show, seed) = parse_args();

    let voronoi = voronoiify_image(Img::load(input));
    let outpth = env::temp_dir().join("voronoi_gen.png");
    voronoi.save(outpth.clone()).unwrap();
    Command::new("eog").arg(outpth).spawn().unwrap();
}

pub fn voronoiify_image(img: Img) -> Img {
    let rng: StdRng = SeedableRng::from_seed([
        154, 209, 215, 146, 162, 81, 13, 78, 243, 132, 107, 232, 61, 157, 71, 142, 202, 167,
        65, 141, 113, 250, 202, 52, 46, 221, 141, 139, 22, 29, 183, 135,
    ]);
    let node_count: usize = img.pixel_cnt() / 50;
    let center_points = generate_random_points(img.width(), img.height(), node_count, rng);
    let center_colors = center_points.new_color_averager();
    // Assign all pixels to the nearest center.
    let pixel_group = Grouping::new(img.width(), img.height());
    let groups = assign_to_centers(pixel_group, center_points);
    let voronoi = pixel_to_group_colors(groups, center_colors, img);
    voronoi
}

#[cfg(test)]
mod tests {
    use super::*;

//    #[bench]
//    fn test_full_flow_performance() {
//        let pth = Path::new("resources").join("imgs").join("parrots.png");
//        let voronoi = voronoiify_image(Img::load(pth.as_path()));
//    }
}
