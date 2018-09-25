extern crate byteorder;
extern crate clap;
extern crate image;
extern crate rand;
extern crate test;
extern crate threadpool;

use argparse::default_seed;
use distribute::generate_random_points;
use img::Img;
use rand::{SeedableRng, StdRng};
use std::path::Path;
#[allow(unused_imports)]
use std::process::Command;
use std::time::Instant;
use voronoiify_image;

/// Benchmark function for --benchmark argument (because tests aren't customizable enough)
pub fn run_bench(reps: usize) {
    assert!(reps >= 2);
    // Create inputs
    let pth = Path::new("resources").join("imgs").join("parrots.png");
    let mut rng: StdRng = SeedableRng::from_seed(default_seed());
    let original_img = Img::load(pth.as_path());
    // Benchmark
    let mut times_ns: Vec<u128> = Vec::with_capacity(reps + 1);
    for _ in 0 .. reps + 1 {
        let mut img = original_img.clone();
        let mut centers = generate_random_points(&img, 100, &mut rng);
        let start = Instant::now();
        test::black_box(voronoiify_image(&mut img, &mut centers));
        let end = Instant::now();
        times_ns.push(end.duration_since(start).as_nanos());
    }
    // First iteration is for warmup
    let avg: u128 = times_ns.iter().skip(1)
        .fold(0u128, |s, t| s + t)
        / reps as u128;
    let std: u128 = times_ns.iter().skip(1)
        .map(|t| (t - avg).pow(2))
        .fold(0u128, |s, t| s + t)
        / (reps - 1) as u128;
    println!("{} reps took {} ± {} ns each", reps, avg, std);
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
