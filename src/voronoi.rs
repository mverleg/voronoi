#![feature(extern_prelude)]
#![feature(nll)]
#![feature(test)]
#![feature(duration_as_u128)]

/// alloc_system avoids using the default bundled allocator, to save space
/// todo: could not get this to work, no space saved at all
// #![feature(alloc_system)]
// extern crate alloc_system;

extern crate byteorder;
extern crate clap;
extern crate image;
extern crate rand;
extern crate separator;
extern crate test;
extern crate threadpool;

//TODO @mark: make some of these mods private?

use assign::assign_to_centers;
use distribute::default_seed;
use distribute::generate_random_points;
use grouping::Grouping;
use img::Img;
use paint::pixel_to_group_colors;
use pointset::UPoints;
use rand::{SeedableRng, StdRng};
use separator::Separatable;
use std::path::Path;
#[allow(unused_imports)]
use std::process::Command;
use std::time::Instant;

#[macro_use]
#[cfg(test)]
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

//TODO @mark: update readme
//TODO @mark: find a way to turn of all asserts in optimized mode? => or just convert the hot-loop-ones to debug_assert and keep the rest

/// Voronoi transform function
pub fn voronoiify_image(img: &mut Img, center_points: &mut UPoints) -> Img {
    let center_colors = center_points.new_color_averager();
    // Assign all pixels to the nearest center.
    let pixel_group = Grouping::new(img.width(), img.height());
    let groups = assign_to_centers(pixel_group, center_points);
    let voronoi = pixel_to_group_colors(groups, center_colors, img);
    voronoi
}

/// Benchmark function for --benchmark argument (because tests aren't customizable enough)
pub fn run_bench(reps: usize) {
    assert!(reps >= 2);
    // Create inputs
    let pth = Path::new("resources").join("imgs").join("parrots.png");
    let mut rng: StdRng = SeedableRng::from_seed(default_seed());
    let original_img = Img::load(pth.as_path());
    // Benchmark
    let mut times_ns: Vec<u64> = Vec::with_capacity(reps + 1);
    for _ in 0 .. reps + 1 {
        let mut img = original_img.clone();
        let mut centers = generate_random_points(&img, 100, &mut rng);
        let start = Instant::now();
        test::black_box(voronoiify_image(&mut img, &mut centers));
        let end = Instant::now();
        times_ns.push(end.duration_since(start).as_nanos() as u64);
    }
    // First iteration is for warmup
    let avg: u64 = times_ns.iter().skip(1)
        .fold(0, |s, t| s + t)
        / (reps as u64);
    let std: f64 = ((times_ns.iter().skip(1)
        .map(|t| (if t > &avg { t - avg } else { avg - t }).pow(2))
        .fold(0, |s, t| s + t)
        / (reps - 1) as u64)
        as f64).sqrt();
    let devperc = 100f64 * std / (avg as f64);
    println!("{} reps took {} ns ± {:.2} % each", reps, (avg as u64).separated_string(), devperc);
}
