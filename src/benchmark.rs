#![feature(test)]

extern crate byteorder;
extern crate clap;
extern crate rand;
extern crate test;
extern crate vorolib;

#[allow(unused_imports)]
use std::process::Command;
use vorolib::run_bench;

pub mod argparse;

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
        bench.iter(|| run_bench(10));
    }
}
