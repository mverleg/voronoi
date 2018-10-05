use find_index::Mid;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
// TODO: usize.max_value() should be interpreted as empty and should not be requested
pub struct PointId {
    value: usize,
}

impl PointId {
    pub fn new(value: usize) -> Self {
        PointId { value }
    }
    pub fn empty() -> Self {
        PointId {
            value: usize::max_value(),
        }
    }
    pub fn increment(&mut self) {
        self.value += 1
    }
    pub fn decrement(&mut self) {
        if self.value == 0 {
            panic!("PointId cannot be decremented because it is 0")
        }
        self.value -= 1
    }

    /// Expose the internal value. Careful with trying to use this to get around type safety.
    //TODO @mark: remove
    pub fn _expose(&self) -> usize {
        self.value
    }
}

impl Add<Self> for PointId {
    type Output = Self;
    fn add(self, rhs: PointId) -> <Self as Add<PointId>>::Output {
        PointId::new(self.value + rhs.value)
    }
}

impl Add<usize> for PointId {
    type Output = Self;
    fn add(self, rhs: usize) -> <Self as Add<PointId>>::Output {
        PointId::new(self.value + rhs)
    }
}

impl Sub<usize> for PointId {
    type Output = Self;
    fn sub(self, rhs: usize) -> <Self as Add<PointId>>::Output {
        if rhs > self.value {
            panic!("PointId cannot be negative");
        }
        PointId::new(self.value - rhs)
    }
}

impl Mid for PointId {
    fn midpoint(first: Self, second: Self) -> Self {
        PointId::new((first.value + second.value) / 2)
    }
}
