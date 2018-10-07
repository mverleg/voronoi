#![feature(extern_prelude)]

extern crate byteorder;
extern crate clap;
extern crate rand;
extern crate scoped_pool;
extern crate vorolib;

use argparse::parse_args;
use rand::{SeedableRng, StdRng};
use scoped_pool::Pool;
#[allow(unused_imports)]
use std::process::Command;
use vorolib::distribute::generate_random_points;
use vorolib::img::Img;
use vorolib::voronoiify_image;

pub mod argparse;

fn main() {
    let (input, output, size, show, seed) = parse_args();
    println!("starting voronoi on image from {}", input.display());
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut img = Img::load(&input);
    let mut centers = generate_random_points(&img, size, &mut rng);
    let workers = Pool::new(num_cpus::get());
    let voronoi = voronoiify_image(&mut img, &mut centers, &workers);
    println!("saving generated image to {}", output.display());
    voronoi.save(output.as_path()).unwrap();
    if show {
        println!("showing image");
        Command::new("eog").arg(output).spawn().unwrap();
    }
}
