#![feature(test)]
#![feature(plugin)]
#![feature(custom_attribute)]

extern crate byteorder;
extern crate clap;
extern crate rand;
extern crate test;
extern crate vorolib;

#[allow(unused_imports)]
use std::process::Command;
use vorolib::run_bench;
use clap::{App, Arg};
use std::process::exit;

pub mod argparse;

pub fn main() {
    let args = App::new("Voronoi benchmark")
        .arg(
            Arg::with_name("reps")
                .help("How many repetitions")
                .short("r")
                .long("reps")
                .value_name("REPS")
                .takes_value(true),
        ).arg(
            Arg::with_name("verbose")
                .help("Log every second approximately")
                .short("v")
                .long("verbose")
    ).get_matches();

    // Repetition count
    let resp = if let Some(sizetxt) = args.value_of("resp") {
        if let Ok(sizeint) = sizetxt.parse::<i32>() {
            sizeint as usize
        } else {
            eprintln!("Invalid value for argv 'count'");
            exit(1);
        }
    } else {
        100
    };

    // Verbose
    let do_log = args.is_present("verbose");

    // Run!
    run_bench(resp, do_log);
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
