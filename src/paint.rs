use colorset::PointColorAverages;
use colorset::PointColors;
use grouping::Grouping;
use img::Img;

//TODO @mark: make a version that changes the image in-place (and one that makes a new one)
/// Set the color of each pixel to the average of the group.
pub fn pixel_to_group_colors(
    mut groups: Grouping,
    mut centers_average_color: PointColorAverages,
    img: &mut Img,
) -> Img {
    let voronoi = img.clone();
    centers_average_color = group_colors_from_pixels(&mut groups, centers_average_color, &voronoi);
    let centers_color = centers_average_color.compute();
    paint_pixels_to_group_color(&mut groups, centers_color, voronoi)
}

/// Like [pixel_to_group_colors], but updates the image in-place.
pub fn group_colors_from_pixels(
    groups: &mut Grouping,
    mut centers: PointColorAverages,
    img: &Img,
) -> PointColorAverages {
    for (x, y, p) in groups.iter_indexed() {
        centers[p] += img[(x, y)];
    }
    centers
}

/// Apply the center's average color to each pixel that belongs to it.
pub fn paint_pixels_to_group_color(
    groups: &mut Grouping,
    centers: PointColors,
    mut img: Img,
) -> Img {
    for (x, y, p) in groups.iter_indexed() {
        img[(x, y)] = centers[p];
    }
    img
}

#[cfg(test)]
mod tests {
    use super::*;
    use color::new_color;
    use pointid::PointId;
    use dims::{X, Y};

    fn make_groups() -> Grouping {
        let p0 = PointId::new(0);
        let p1 = PointId::new(1);
        let mut groups = Grouping::new(X::new(3), Y::new(2));
        let (x0, x1, x2, y0, y1) = (X::new(0), X::new(1), X::new(2), Y::new(0), Y::new(1));
        groups.set(x0, y0, p0);
        groups.set(x1, y0, p0);
        groups.set(x2, y0, p1);
        groups.set(x0, y1, p0);
        groups.set(x1, y1, p1);
        groups.set(x2, y1, p1);
        groups
    }

    #[test]
    fn test_group_colors_from_pixels() {
        let p0 = PointId::new(0);
        let p1 = PointId::new(1);
        let mut groups = make_groups();
        let centers = PointColorAverages::new(2);
        let mut img = Img::empty(X::new(3), Y::new(2));
        let (x0, x1, x2, y0, y1) = (X::new(0), X::new(1), X::new(2), Y::new(0), Y::new(1));
        img[(x0, y0)] = new_color(0, 0, 0);
        img[(x1, y0)] = new_color(0, 0, 0);
        img[(x2, y0)] = new_color(255, 255, 255);
        img[(x0, y1)] = new_color(255, 255, 255);
        img[(x1, y1)] = new_color(255, 255, 0);
        img[(x2, y1)] = new_color(255, 0, 0);
        let avgs = group_colors_from_pixels(&mut groups, centers, &img);
        let colors = avgs.compute();
        assert_eq!(colors[p0], new_color(85, 85, 85));
        assert_eq!(colors[p1], new_color(255, 170, 85));
    }

    #[test]
    fn test_paint_pixels_to_group_color() {
        let mut groups = make_groups();
        let img = Img::empty(X::new(3), Y::new(2));
        let colors = PointColors::new(vec![new_color(255, 170, 85), new_color(170, 170, 170)]);
        let voronoi = paint_pixels_to_group_color(&mut groups, colors, img);
        let (x0, x1, x2, y0, y1) = (X::new(0), X::new(1), X::new(2), Y::new(0), Y::new(1));
        assert_eq!(new_color(255, 170, 85), voronoi[(x0, y0)]);
        assert_eq!(new_color(255, 170, 85), voronoi[(x1, y0)]);
        assert_eq!(new_color(170, 170, 170), voronoi[(x2, y0)]);
        assert_eq!(new_color(255, 170, 85), voronoi[(x0, y1)]);
        assert_eq!(new_color(170, 170, 170), voronoi[(x1, y1)]);
        assert_eq!(new_color(170, 170, 170), voronoi[(x2, y1)]);
    }
}
