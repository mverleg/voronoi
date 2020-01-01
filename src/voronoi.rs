#![feature(nll)]
#![feature(test)]
#![cfg_attr(feature = "flame_it", feature(proc_macro_hygiene))]

#[cfg(feature = "flame_it")]
extern crate flame;
#[cfg(feature = "flame_it")]
#[macro_use] extern crate flamer;

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
pub mod distribute;
mod find_index;
mod nearest_within_box;
mod grouping;
pub mod img;
mod norms;
mod paint;
mod parmap;
mod point;
mod pointid;
mod pointset;

/// Voronoi transform function
#[cfg_attr(feature = "flame_it", flame)]
pub fn voronoiify_image(img: &mut Img, center_points: &mut UPoints, workers: &Pool) -> Img {
    let center_colors = center_points.new_color_averager();
    // Assign all pixels to the nearest center.
    let groups = assign_to_centers(center_points, &workers);
    pixel_to_group_colors(groups, center_colors, img)
}
