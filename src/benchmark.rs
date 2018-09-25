//extern crate rand;
//extern crate test;
extern crate byteorder;
//extern crate rand;
extern crate voronoi;
extern crate clap;
//extern crate byteorder;

pub mod argparse;

use argparse::default_seed;
use distribute::generate_random_points;
use img::Img;
use rand::{SeedableRng, StdRng};
use separator::Separatable;
use std::path::Path;
#[allow(unused_imports)]
use std::process::Command;
use std::time::Instant;
use voronoiify_image;

pub fn main() {
    println!("running benchmark, may take a while");
    run_bench(100);
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, StdRng};
    use super::*;
    use test::Bencher;
    //TODO @mark: THIS CODE IS TEMPORARY!

    #[bench]
    fn test_full_flow_performance(bench: &mut Bencher) {
        // Create inputs
        let pth = Path::new("resources").join("imgs").join("parrots.png");
        let mut rng: StdRng = SeedableRng::from_seed(default_seed());
        let original_img = Img::load(pth.as_path());
        // Warmup
        let mut img = original_img.clone();
        let mut centers = generate_random_points(&img, 100, &mut rng);
        test::black_box(voronoiify_image(&mut img, &mut centers));
        // Benchmark
        for _ in 0 .. 10 {
            let mut img = original_img.clone();
            let mut centers = generate_random_points(&img, 100, &mut rng);
            bench.iter(|| voronoiify_image(&mut img, &mut centers));
        }
    }
}
