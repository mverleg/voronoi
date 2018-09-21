use dims::{Dim, X, Y};
use pointid::PointId;
use std::ops::Index;
use std::slice::IterMut;

#[derive(Debug)]
pub struct Grouping {
    center_links: Vec<Vec<PointId>>,
}

impl Grouping {
    pub fn new(width: X, height: Y) -> Self {
        Grouping {
            center_links: vec![
                vec![PointId::empty(); height._expose() as usize];
                width._expose() as usize
            ],
        }
    }

    #[inline]
    pub fn width(&self) -> X {
        //TODO @mark: consider just storing these fields
        X::new(self.center_links.len() as usize)
    }

    #[inline]
    pub fn height(&self) -> Y {
        Y::new(self.center_links[0].len() as usize)
    }

    //TODO @mark: still used?
    pub fn iter_mut(&mut self) -> IterMut<Vec<PointId>> {
        self.center_links.iter_mut()
    }

    pub fn iter_indexed(&mut self) -> GroupIndexIterator {
        GroupIndexIterator::new(self)
    }

    #[inline]
    pub fn set(&mut self, x: X, y: Y, point_id: PointId) {
        debug_assert!(
            x._expose() < self.width()._expose(),
            format!(
                "Expectation violated: X {} < X-dim {}\n",
                x._expose(),
                self.width()._expose()
            )
        );
        debug_assert!(
            y._expose() < self.height()._expose(),
            format!(
                "Expectation violated: Y {} < y-dim {}\n",
                y._expose(),
                self.height()._expose()
            )
        );
        self.center_links[x._expose() as usize][y._expose() as usize] = point_id;
    }

    #[inline]
    pub fn get(&self, x: X, y: Y) -> PointId {
        //TODO @mark: from over here, it looks like X and Y should be usize
        self.center_links[x._expose() as usize][y._expose() as usize]
    }
}

impl Index<(X, Y)> for Grouping {
    type Output = PointId;

    fn index(&self, index: (X, Y)) -> &Self::Output {
        &self.center_links[(index.0)._expose() as usize][(index.1)._expose() as usize]
    }
}

#[derive(Debug)]
pub struct GroupIndexIterator<'a> {
    grouping: &'a mut Grouping,
    x: X,
    y: Y,
}

impl<'a> GroupIndexIterator<'a> {
    pub fn new(grouping: &'a mut Grouping) -> Self {
        GroupIndexIterator {
            grouping,
            x: X::new(0),
            y: Y::new(0),
        }
    }
}

impl<'a> Iterator for GroupIndexIterator<'a> {
    type Item = (X, Y, PointId);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.grouping.width() {
            self.x = X::new(0);
            self.y = self.y + 1;
        }
        if self.y >= self.grouping.height() {
            return Option::None;
        }
        let res = Option::Some((self.x, self.y, self.grouping[(self.x, self.y)]));
        self.x = self.x + 1;
        res
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

    #[test]
    fn test_iter() {
        let mut groups = Grouping::new(X::new(2), Y::new(1));
        groups.set(X::new(1), Y::new(0), PointId::new(1));
        groups.set(X::new(0), Y::new(0), PointId::new(0));
        let mut iter = groups.iter_indexed();
        assert_eq!(
            Option::Some((X::new(0), Y::new(0), PointId::new(0))),
            iter.next()
        );
        assert_eq!(
            Option::Some((X::new(1), Y::new(0), PointId::new(1))),
            iter.next()
        );
        assert_eq!(Option::None, iter.next());
        assert_eq!(Option::None, iter.next());
    }
}
