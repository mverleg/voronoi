
use clap::{App, Arg};
use std::path::{Path, PathBuf};
#[allow(unused_imports)]
use std::process::Command;
use std::process::exit;

pub fn parse_args() -> (PathBuf, PathBuf, usize, bool, u32) {
    let args = App::new("Voronoiify")
        .version("1.0")
        .about("Group image into voronoi-based patches and assign the average color to each patch")
        .arg(Arg::with_name("input")
            .help("Input png file to voronoiify")
            .required(true)
            .value_name("IN_PTH")
            .index(1))
        .arg(Arg::with_name("output")
            .help("Path to store the generated file")
            .short("o")
            .long("output")
            .value_name("OUT_PTH")
            .takes_value(true))
        .arg(Arg::with_name("size")
            .help("Average number of pixels per group")
            .short("c")
            .long("patch_size")
            .value_name("PATCH_SIZE")
            .takes_value(true))
        .arg(Arg::with_name("show")
            .help("Show the generated image using EOG")
            .short("s")
            .long("show"))
        .arg(Arg::with_name("seed")
            .help("Use the given random seed")
            .short("r")
            .long("seed")
            .value_name("SEED")
            .takes_value(true))
        .get_matches();

    // Input
    let input = Path::new(args.value_of("input")
        .unwrap()).to_path_buf();
    if !input.exists() {
        eprintln!("Input file {} does not exist", input.display());
        exit(1);
    }

    // Output
    //TODO @mark: better default?
    let output = Path::new(args.value_of("output")
        .unwrap_or("/tmp/generated.png")).to_path_buf();

    // Center count
    let size = if let Some(sizetxt) = args.value_of("size") {
        if let Ok(sizeint) = sizetxt.parse::<i32>() {
            if sizeint < 2 {
                eprintln!("Argument to -c/--patch_size be at least 1 (got integer {})", sizeint);
                exit(3);
            }
            sizeint as usize
        } else {
            eprintln!("Argument to -c/--patch_size should be positive integer (got non-integer '{}')", sizetxt);
            exit(4);
        }
    } else {
        50
    };

    // Show
    let show = true;
    unimplemented!();  //TODO @mark: THIS CODE IS TEMPORARY!

    // Seed
    let seed = 123456789;
    unimplemented!();  //TODO @mark: THIS CODE IS TEMPORARY!

    (input, output, size, show, seed)
}
