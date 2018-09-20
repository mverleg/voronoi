
use assign::assign_to_centers;
use clap::App;
use clap::Arg;
use distribute::generate_random_points;
use grouping::Grouping;
use img::Img;
use paint::pixel_to_group_colors;
use rand::{SeedableRng, StdRng};
use std::env;
use std::path::Path;
#[allow(unused_imports)]
use std::process::Command;
use std::process::exit;

pub fn parse_args() -> (&Path, &Path, usize, bool, u32) {
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
        .arg(Arg::with_name("count")
            .help("Number of color patches to divide the image into")
            .short("c")
            .long("center_count")
            .value_name("CENTERS")
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

    println!("{:?}", args);

    // Input
    let input = args.value_of("input");
    if !input.exists() {
        eprintln!("File {} does not exist", input.display());
        exit(1);
    }
    let input = Path::new(input.unwrap());

    // Output
    let output = args.value_of("output");
    if !output.exists() {
        eprintln!("File {} does not exist", output.display());
        exit(1);
    }
    let output = Path::new(output.unwrap());

    // Center count
    let count = 10;
    unimplemented!();  //TODO @mark: THIS CODE IS TEMPORARY!

    // Show
    let show = true;
    unimplemented!();  //TODO @mark: THIS CODE IS TEMPORARY!

    // Seed
    let seed = 123456789;
    unimplemented!();  //TODO @mark: THIS CODE IS TEMPORARY!

    (input, output, count, show, seed)
}
