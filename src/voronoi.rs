#![feature(extern_prelude)]
#![feature(nll)]
#![feature(test)]
#![feature(duration_as_u128)]

/// alloc_system avoids using the default bundled allocator, to save space
/// todo: could not get this to work, no space saved at all
// #![feature(alloc_system)]
// extern crate alloc_system;

extern crate byteorder;
extern crate clap;
extern crate image;
extern crate num_cpus;
extern crate rand;
extern crate scoped_pool;
extern crate separator;
extern crate test;

use assign::assign_to_centers;
use img::Img;
use paint::pixel_to_group_colors;
use pointset::UPoints;
use scoped_pool::Pool;
#[allow(unused_imports)]
use std::process::Command;

#[macro_use]
#[cfg(test)]
mod test_util;
mod assign;
mod color;
mod colorset;
mod dims;
mod find_index;
mod grouping;
mod norms;
mod paint;
mod point;
mod pointid;
mod pointset;
mod parmap;
pub mod distribute;
pub mod img;

/// Voronoi transform function
pub fn voronoiify_image(img: &mut Img, center_points: &mut UPoints, workers: &Pool) -> Img {
    let center_colors = center_points.new_color_averager();
    // Assign all pixels to the nearest center.
    let groups = assign_to_centers(center_points, &workers);
    let voronoi = pixel_to_group_colors(groups, center_colors, img);
    voronoi
}

