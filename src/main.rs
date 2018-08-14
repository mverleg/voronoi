#![feature(extern_prelude)]

use std::path::Path;
use std::env;
use std::process::Command;

fn main() {
    let pth = Path::new("resources").join("imgs").join("parrots.png");
    let img = image::open(pth).unwrap();
//    println!("{:?}", img);
    let mut outpth = env::temp_dir().join("voronoi_gen.png");
    img.save(outpth.clone()).unwrap();
    Command::new("eog").arg(outpth).spawn().unwrap();
}
