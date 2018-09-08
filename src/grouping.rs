use pointid::PointId;
use dims::{X, Y};

#[derive(Debug)]
pub struct Grouping {
    center_links: Vec<Vec<PointId>>,
}

impl Grouping {
    pub fn new(width: X, height: Y) -> Self {
        Grouping {
            center_links: vec![
                vec![
                    PointId::empty(); width._expose() as usize
                ]; height._expose() as usize
            ]
        }
    }
}
