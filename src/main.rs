#![feature(extern_prelude)]

//TODO @mark: update readme
//TODO @mark: try with another distance function

extern crate byteorder;
extern crate clap;
extern crate rand;
extern crate vorolib;
extern crate scoped_pool;

use argparse::parse_args;
use rand::{SeedableRng, StdRng};
#[allow(unused_imports)]
use std::process::Command;
use vorolib::distribute::generate_random_points;
use vorolib::img::Img;
use vorolib::voronoiify_image;
use scoped_pool::Pool;

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
