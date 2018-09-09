use pointid::PointId;
use dims::{Dim, X, Y};
use std::slice::IterMut;

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

    pub fn iter_mut(&mut self) -> IterMut<Vec<PointId>> {
        self.center_links.iter_mut()
    }
}
