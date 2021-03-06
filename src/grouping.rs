use crate::dims::{X, Y};
use crate::pointid::PointId;
use ::std::ops::Index;
use ::std::ops::IndexMut;
use ::std::vec::IntoIter;

#[derive(Debug)]
pub struct Grouping {
    center_links: Vec<GroupingRow>,
    width: X,
    height: Y,
}

impl Grouping {
    pub fn from(width: X, height: Y, centers: Vec<GroupingRow>) -> Self {
        #[cfg(debug_assertions)]
        {
            debug_assert!(width.value == centers.len());
            for row in centers.iter() {
                debug_assert!(height == row.height());
            }
        }
        Grouping {
            center_links: centers,
            width,
            height,
        }
    }
    pub fn width(&self) -> X {
        self.width
    }
    pub fn height(&self) -> Y {
        self.height
    }
    pub fn iter_indexed(&mut self) -> GroupIndexIterator {
        GroupIndexIterator::new(self)
    }
}

// Test only
pub trait TestTrait {
    fn empty(width: X, height: Y) -> Grouping;

    fn set(&mut self, x: X, y: Y, point_id: PointId);

    fn get(&self, x: X, y: Y) -> PointId;
}

// Test only
impl TestTrait for Grouping {
    fn empty(width: X, height: Y) -> Grouping {
        Grouping {
            center_links: vec![
                GroupingRow {
                    center_links_row: vec![PointId::empty(); height.value],
                    height
                };
                width.value as usize
            ],
            width,
            height,
        }
    }

    fn set(&mut self, x: X, y: Y, point_id: PointId) {
        debug_assert!(
            x.value < self.width().value,
            format!(
                "Expectation violated: X {} < X-dim {}\n",
                x.value,
                self.width().value
            )
        );
        debug_assert!(
            y.value < self.height().value,
            format!(
                "Expectation violated: Y {} < y-dim {}\n",
                y.value,
                self.height().value
            )
        );
        self.center_links[x.value][y] = point_id;
    }

    fn get(&self, x: X, y: Y) -> PointId {
        //TODO @mark: from over here, it looks like X and Y should be usize
        self.center_links[x.value][y]
    }
}

#[derive(Debug, Clone)]
pub struct GroupingRow {
    center_links_row: Vec<PointId>,
    height: Y,
}

impl GroupingRow {
    pub fn from(center_links_row: Vec<PointId>, height: Y) -> Self {
        debug_assert!(center_links_row.len() == height.as_index());
        GroupingRow {
            center_links_row,
            height,
        }
    }

    #[allow(dead_code)] // Not really unused
    pub fn height(&self) -> Y {
        self.height
    }
}

#[derive(Debug)]
pub struct GroupingRowIterator {
    grouping: IntoIter<GroupingRow>,
    index: usize,
}

impl Iterator for GroupingRowIterator {
    type Item = (X, GroupingRow);
    fn next(&mut self) -> Option<Self::Item> {
        let val = match self.grouping.next() {
            Some(row) => Some((X::new(self.index), row)),
            None => None,
        };
        self.index += 1;
        val
    }
}

impl IntoIterator for Grouping {
    type Item = <GroupingRowIterator as Iterator>::Item;
    type IntoIter = GroupingRowIterator;
    fn into_iter(self) -> Self::IntoIter {
        GroupingRowIterator {
            grouping: self.center_links.into_iter(),
            index: 0,
        }
    }
}

impl Index<(X, Y)> for Grouping {
    type Output = PointId;
    fn index(&self, index: (X, Y)) -> &Self::Output {
        &self.center_links[(index.0).value][(index.1)]
    }
}

impl Index<Y> for GroupingRow {
    type Output = PointId;
    //TODO @mark: update all Index that return copy types to just .get() to prevent & ?
    fn index(&self, index: Y) -> &Self::Output {
        &self.center_links_row[index.value]
    }
}

impl IndexMut<Y> for GroupingRow {
    fn index_mut(&mut self, index: Y) -> &mut Self::Output {
        &mut self.center_links_row[index.value]
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
        let mut groups = Grouping::empty(X::new(2), Y::new(1));
        groups.set(X::new(1), Y::new(0), PointId::new(1));
        groups.set(X::new(0), Y::new(0), PointId::new(0));
        assert_eq!(PointId::new(0), groups.get(X::new(0), Y::new(0)));
        assert_eq!(PointId::new(1), groups.get(X::new(1), Y::new(0)));
    }

    #[test]
    fn test_iter() {
        let mut groups = Grouping::empty(X::new(2), Y::new(1));
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
