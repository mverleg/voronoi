#![feature(extern_prelude)]
#![feature(nll)]
#![feature(test)]

/// alloc_system avoids using the default bundled allocator, to save space
/// todo: could not get this to work, no space saved at all
// #![feature(alloc_system)]
// extern crate alloc_system;

extern crate byteorder;
extern crate clap;
extern crate image;
extern crate rand;
extern crate test;
extern crate threadpool;

use argparse::parse_args;
use assign::assign_to_centers;
use distribute::generate_random_points;
use grouping::Grouping;
use img::Img;
use paint::pixel_to_group_colors;
use rand::{SeedableRng, StdRng};
#[allow(unused_imports)]
use std::process::Command;
use pointset::UPoints;

#[macro_use]
pub mod test_util;

pub mod argparse;
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

//TODO @mark: update readme
//TODO @mark: find a way to turn of all asserts in optimized mode? => or just convert the hot-loop-ones to debug_assert and keep the rest
//TODO @mark: get rid of expose()

fn main() {
    let (input, output, size, show, seed) = parse_args();
    println!("starting voronoi on image from {}", input.display());
    let rng: StdRng = SeedableRng::from_seed(seed);
    let img = Img::load(&input);
    let centers = generate_random_points(&img, size, rng);
    let voronoi = voronoiify_image(img, centers);
    println!("saving generated image to {}", output.display());
    voronoi.save(output.as_path()).unwrap();
    if show {
        println!("showing image");
        Command::new("eog").arg(output).spawn().unwrap();
    }
}

pub fn voronoiify_image(img: Img, center_points: UPoints) -> Img {
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
    use argparse::default_seed;
    use std::path::Path;
    use test::Bencher;

    #[bench]
    fn test_full_flow_performance(bench: &mut Bencher) {
        //TODO @mark: compile error
        // Create inputs
        let pth = Path::new("resources").join("imgs").join("parrots.png");
        let rng = SeedableRng::from_seed(default_seed());
        let original_img = Img::load(pth.as_path());
        // Warmup
        let centers = generate_random_points(&original_img, 100, rng);
        let voronoi = voronoiify_image(original_img, centers);
        // Benchmark
        for _ in 0 .. 100 {
            let centers = generate_random_points(&original_img, 100, rng);
            let img = original_img.clone();
            bench.iter(|| voronoiify_image(original_img, centers));
        }
    }
}
