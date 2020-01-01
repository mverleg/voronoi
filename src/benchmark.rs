#![feature(test)]
#![feature(plugin)]
#![cfg_attr(feature = "flame_it", feature(proc_macro_hygiene))]

#[cfg(feature = "flame_it")]
extern crate flame;
#[cfg(feature = "flame_it")]
#[macro_use] extern crate flamer;

use ::std::path::Path;
use ::std::path::PathBuf;
#[allow(unused_imports)]
use ::std::process::Command;
use ::std::process::exit;
use ::std::time::Instant;

use ::clap::{App, Arg};
use ::rand::rngs::StdRng;
use ::rand::SeedableRng;
use ::scoped_pool::Pool;
use ::separator::Separatable;

use ::vorolib::distribute::default_seed;
use ::vorolib::distribute::generate_random_points;
use ::vorolib::img::Img;
use ::vorolib::voronoiify_image;
#[cfg(feature = "flame_it")]
use std::fs::File;

pub mod argparse;

#[cfg_attr(feature = "flame_it", flame)]
pub fn main() {
    let args = App::new("Voronoi benchmark")
        .arg(
            Arg::with_name("input")
                .help("Input png file to run voronoiify benchmark on")
                .short("i")
                .long("input")
                .value_name("IN_PTH"),
        )
        .arg(
            Arg::with_name("reps")
                .help("How many repetitions")
                .short("r")
                .long("reps")
                .value_name("REPS")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Log every second approximately")
                .short("v")
                .long("verbose"),
        )
        .get_matches();

    // Input
    let input = Path::new(
        args.value_of("input")
            .unwrap_or("resources/imgs/parrots.png"),
    )
    .to_path_buf();
    if !input.exists() {
        eprintln!("Benchmark input file {} does not exist", input.display());
        exit(2);
    }

    // Repetition count
    let resp = if let Some(sizetxt) = args.value_of("reps") {
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
    run_bench(input, resp, do_log);
}

#[cfg_attr(feature = "flame_it", flame)]
pub fn load_img(pth: &Path) -> Img {
    Img::load(pth)
}

#[cfg_attr(feature = "flame_it", flame)]
pub fn clone_img(img: &Img) -> Img {
    img.clone()
}

#[cfg_attr(feature = "flame_it", flame)]
pub fn report_total_time(times_ns: &[u64], reps: usize) {
    // First iteration is for warmup
    let avg: u64 = times_ns.iter().skip(1).sum::<u64>() / (reps as u64);
    let std: f64 = ((times_ns
        .iter()
        .skip(1)
        .map(|t| (if t > &avg { t - avg } else { avg - t }).pow(2))
        .sum::<u64>()
        / (reps - 1) as u64) as f64)
        .sqrt();
    let devperc = 100f64 * std / (avg as f64);
    println!(
        "{} reps took {} ns ± {:.2} % each ({} per second)",
        reps,
        (avg as u64).separated_string(),
        devperc,
        1_000_000_000 / avg
    );
}

/// Benchmark function for --benchmark argument (because tests aren't customizable enough)
#[cfg_attr(feature = "flame_it", flame)]
pub fn run_bench(input: PathBuf, reps: usize, do_log_flag: bool) {
    assert!(reps >= 2);

    let do_log: bool = if cfg!(feature = "log_it") {
        do_log_flag
    } else {
        if do_log_flag {
            eprintln!("cannot do verbose logging, because feature 'log_it' was disabled");
        }
        false
    };

    let init = Instant::now();
    let mut last_log = 0;
    // Create inputs
    let pth = ::core::hint::black_box(input);
    let mut rng: StdRng = SeedableRng::from_seed(default_seed());
    let original_img = load_img(pth.as_path());
    // Benchmark
    let mut times_ns: Vec<u64> = Vec::with_capacity(reps + 1);
    if do_log {
        println!(" {:4} / {:4}", 0, reps);
    }
    let workers = Pool::new(num_cpus::get());
    for rep in 0..=reps {
        if do_log {
            let total_time = Instant::now().duration_since(init).as_secs();
            if total_time > last_log {
                last_log = total_time;
                println!(" {:4} / {:4}", rep, reps);
            }
        }
        let mut img = clone_img(&original_img);
        let mut centers = generate_random_points(&img, 100, &mut rng);
        let start = Instant::now();
        ::core::hint::black_box(voronoiify_image(&mut img, &mut centers, &workers));
        let end = Instant::now();
        times_ns.push(end.duration_since(start).as_nanos() as u64);
        //        times_ns.push(1u64);
    }
    if do_log {
        println!(" {:4} / {:4}", reps, reps);
    }
    #[cfg(feature = "flame_it")]
    {
        //flame::dump_stdout();
        let f = File::create(&format!("target/flame-{}.json", reps));
        flame::dump_json(&mut f.unwrap()).unwrap();
    }
    report_total_time(&times_ns, reps);
}

#[cfg(bench)]
mod tests {
    extern crate test;

    use test::Bencher;

    use super::*;

    #[bench]
    fn test_full_flow_performance(bench: &mut Bencher) {
        let input = Path::new("resources").join("imgs").join("parrots.png");
        bench.iter(|| run_bench(input.clone(), 10, false));
    }
}
