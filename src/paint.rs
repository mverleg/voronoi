use color::empty_img;
use color::Img;
use colorset::PointColorAverages;
use colorset::PointColors;
use grouping::Grouping;

//TODO @mark: make a version that changes the image in-place (and one that makes a new one)
/// Set the color of each pixel to the average of the group.
pub fn pixel_to_group_colors(groups: Grouping, mut centers_average_color: PointColorAverages, img: Img) -> Img {
    let mut voronoi = img.clone();
    centers_average_color = group_colors_from_pixels(&groups, centers_average_color, &img);
    let mut centers_color = centers_average_color.compute();
    voronoi = paint_pixels_to_group_color(&groups, centers_color, img);
    voronoi
}

/// Like [pixel_to_group_colors], but updates the image in-place.
pub fn group_colors_from_pixels(groups: &Grouping, centers: PointColorAverages, img: &Img) -> PointColorAverages {
    unimplemented!() // TODO: mark
}

/// Apply the center's average color to each pixel that belongs to it.
pub fn paint_pixels_to_group_color(groups: &Grouping, centers: PointColors, img: Img) -> Img {
    unimplemented!() // TODO: mark
}
