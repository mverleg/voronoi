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
extern crate rand;
extern crate scoped_pool;
extern crate separator;
extern crate test;
extern crate num_cpus;

//TODO @mark: make some of these mods private?

use assign::assign_to_centers;
use grouping::Grouping;
use img::Img;
use paint::pixel_to_group_colors;
use pointset::UPoints;
use scoped_pool::Pool;
#[allow(unused_imports)]
use std::process::Command;

#[macro_use]
#[cfg(test)]
pub mod test_util;
pub mod assign;
pub mod color;
pub mod colorset;
pub mod dims;
pub mod distribute;
pub mod find_index;
pub mod grouping;
pub mod img;
pub mod norms;
pub mod paint;
pub mod point;
pub mod pointid;
pub mod pointset;

//TODO @mark: update readme
//TODO @mark: find a way to turn of all asserts in optimized mode? => or just convert the hot-loop-ones to debug_assert and keep the rest

/// Voronoi transform function
pub fn voronoiify_image(img: &mut Img, center_points: &mut UPoints) -> Img {
    let center_colors = center_points.new_color_averager();
    // Assign all pixels to the nearest center.
    //TODO @mark: if movies are added, make sure to recycle threadpool
    let workers = Pool::new(num_cpus::get());
    let groups = assign_to_centers(center_points, &workers);
    let voronoi = pixel_to_group_colors(groups, center_colors, img);
    voronoi
}

