use dims::{Dim, X, Y};
use pointid::PointId;
use std::slice::IterMut;

#[derive(Debug)]
pub struct Grouping {
    center_links: Vec<Vec<PointId>>,
}

impl Grouping {
    pub fn new(width: X, height: Y) -> Self {
        Grouping {
            center_links: vec![
                vec![PointId::empty(); width._expose() as usize];
                height._expose() as usize
            ],
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<Vec<PointId>> {
        self.center_links.iter_mut()
    }

    #[inline]
    pub fn set(&mut self, x: X, y: Y, point_id: PointId) {
        self.center_links[x._expose() as usize][y._expose() as usize] = point_id;
    }

    #[inline]
    pub fn get(&self, x: X, y: Y) -> PointId {
        //TODO @mark: from over here, it looks like X and Y should be usize
        self.center_links[x._expose() as usize][y._expose() as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get() {
        let mut groups = Grouping::new(X::new(2), Y::new(1));
        groups.set(X::new(1), Y::new(0), PointId::new(1));
        groups.set(X::new(0), Y::new(0), PointId::new(0));
        assert_eq!(PointId::new(0), groups.get(X::new(0), Y::new(0)));
        assert_eq!(PointId::new(1), groups.get(X::new(1), Y::new(0)));
    }
}
