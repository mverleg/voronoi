#![feature(nll)]
#![feature(test)]

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
mod grouping;
pub mod img;
mod norms;
mod paint;
mod parmap;
mod point;
mod pointid;
mod pointset;

/// Voronoi transform function
pub fn voronoiify_image(img: &mut Img, center_points: &mut UPoints, workers: &Pool) -> Img {
    let center_colors = center_points.new_color_averager();
    // Assign all pixels to the nearest center.
    let groups = assign_to_centers(center_points, &workers);
    pixel_to_group_colors(groups, center_colors, img)
}
