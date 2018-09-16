use color::Img;
use colorset::PointColorAverages;
use colorset::PointColors;
use dims::{X, Y};
use grouping::Grouping;

//TODO @mark: make a version that changes the image in-place (and one that makes a new one)
/// Set the color of each pixel to the average of the group.
pub fn pixel_to_group_colors(
    groups: Grouping,
    mut centers_average_color: PointColorAverages,
    img: Img,
) -> Img {
    let mut voronoi = img.clone();
    centers_average_color = group_colors_from_pixels(&groups, centers_average_color, &img);
    let mut centers_color = centers_average_color.compute();
    voronoi = paint_pixels_to_group_color(&groups, centers_color, img);
    voronoi
}

/// Like [pixel_to_group_colors], but updates the image in-place.
pub fn group_colors_from_pixels(
    groups: &Grouping,
    centers: PointColorAverages,
    img: &Img,
) -> PointColorAverages {

    unimplemented!()  //TODO @mark: THIS CODE IS TEMPORARY!
}

/// Apply the center's average color to each pixel that belongs to it.
pub fn paint_pixels_to_group_color(groups: &Grouping, centers: PointColors, img: Img) -> Img {
    unimplemented!() // TODO: mark
}

#[cfg(test)]
mod tests {
    use color::empty_img;
    use color::new_color;
    use pointid::PointId;
    use super::*;
    use color::RgbColorAverage;

    #[test]
    fn test_group_colors_from_pixels() {
        let p0 = PointId::new(0);
        let p1 = PointId::new(1);
        let mut groups = Grouping::new(X::new(3), Y::new(2));
        groups.set(X::new(0), Y::new(0), p0);
        groups.set(X::new(1), Y::new(0), p0);
        groups.set(X::new(2), Y::new(0), p1);
        groups.set(X::new(0), Y::new(1), p0);
        groups.set(X::new(2), Y::new(1), p1);
        groups.set(X::new(3), Y::new(1), p1);
        let centers = PointColorAverages::new(2);
        let mut img = empty_img(3, 2);
        img[(0, 0)] = new_color(0, 0, 0);
        img[(1, 0)] = new_color(0, 0, 0);
        img[(2, 0)] = new_color(255, 255, 255);
        img[(0, 1)] = new_color(255, 255, 255);
        img[(2, 1)] = new_color(255, 255, 0);
        img[(3, 1)] = new_color(255, 0, 0);
        let avgs = group_colors_from_pixels(&groups, centers, &img);
        let colors = avgs.compute();
        assert_eq!(colors[p0], new_color(85, 85, 85));
        assert_eq!(colors[p1], new_color(170, 170, 170));

        panic!(); //TODO @mark:
    }

    #[test]
    fn test_paint_pixels_to_group_color() {
        panic!(); //TODO @mark:
    }
}
