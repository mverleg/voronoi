
use std::ops::{Add, Sub, Mul};
use std::ops::Div;
use norms::Dist;

/// These X and Y are indices (unsigned integers), not physical distances.

/// The number of units in X or Y dimension, used as an area.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Count {
    value: usize,
}

impl Count {
    pub fn new(value: usize) -> Self {
        Count { value }
    }

    /// Expose the internal usize. Should only be used when required for external code.
    pub fn _expose(&self) -> usize {
        self.value
    }
}

impl Mul<usize> for Count {
    type Output = Count;

    fn mul(self, other: usize) -> Self::Output {
        Count { value: self.value * other }
    }
}

impl Div<usize> for Count {
    type Output = Count;

    fn div(self, other: usize) -> Self::Output {
        Count { value: self.value / other }
    }
}

macro_rules! make_dim {
    ( $T:ident, $dT:ident ) => {
        #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        pub struct $T {
            value: usize,
        }

        impl $T {
            pub fn new(value: usize) -> Self {
                $T { value }
            }

            /// Expose the internal usize. Should only be used when required for external code.
            pub fn _expose(&self) -> usize {
                self.value
            }
        }

        //noinspection RsStructNaming
        #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        pub struct $dT {
            step: i32,
        }

        impl $dT {
            pub fn new(step: i32) -> Self {
                $dT { step }
            }

            pub fn abs(&self) -> Self {
                if self.step < 0 {
                    return $dT { step: self.step }
                }
                self.clone()
            }

//            pub fn dist(self) -> Dist {
//                Dist::fnew(self.step.abs() as f64)
//            }
        }

        impl Sub<$T> for $T {
            type Output = $dT;

            fn sub(self, other: $T) -> Self::Output {
                $dT { step: (self.value as i32) - (other.value as i32) }
            }
        }

        impl Sub<$dT> for $T {
            type Output = $T;

            fn sub(self, other: $dT) -> Self::Output {
                if (self.value as i32) < other.step {
                    $T { value: 0 }
                } else {
                    $T { value: self.value - (other.step as usize) }
                }
            }
        }

        impl Add<$dT> for $T {
            type Output = $T;

            fn add(self, other: $dT) -> Self::Output {
                if self.value < (-other.step as usize) {
                    $T { value: 0 }
                } else {
                    $T { value: self.value + (other.step as usize) }
                }
            }
        }

        impl Mul<$T> for $T {
            type Output = Count;

            fn mul(self, other: $T) -> Self::Output {
                Count { value: self.value * other.value }
            }
        }
    }
}

make_dim!(X, dX);
make_dim!(Y, dY);

impl Mul<Y> for X {
    type Output = Count;

    fn mul(self, other: Y) -> Self::Output {
        Count { value: self.value * other.value }
    }
}

impl Mul<dY> for dX {
    type Output = Dist;

    fn mul(self, other: dY) -> Self::Output {
        Dist { value: (self.step + other.step) as f64 }
    }
}

impl Add<dY> for dX {
    type Output = Count;

    fn add(self, other: dY) -> Self::Output {
        //TODO @mark: this is unsafe, could be negative
        Count { value: (self.step + other.step) as usize }
    }
}

impl From<Count> for Dist {
    fn from(count: Count) -> Self {
        Dist { value: count.value as f64 }
    }
}
