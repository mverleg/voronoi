use find_index::Mid;
use std::ops::{Add, Sub};

/// These X and Y are indices (unsigned integers), not physical distances.

macro_rules! make_dim {
    ( $T:ident, $dT:ident ) => {

        // Performance: hand-coding PartialEq is not faster than deriving,
        // in fact it's slower unless lt/gt/le/ge are also hand-coded.
        #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        pub struct $T {
            pub value: usize,
        }

        impl $T {
            pub fn new(value: usize) -> Self {
                $T { value }
            }
            pub fn as_index(&self) -> usize {
                self.value
            }
            pub fn indices_upto(&self) -> impl Iterator<Item=$T> {
                (0 .. self.value).map(|val| $T::new(val))
            }
            pub fn saturating_sub(self, other: usize) -> Self {
                $T { value: self.value.saturating_sub(other) }
            }
        }

        impl Mid for $T {
            fn midpoint(first: $T, second: $T) -> $T {
                $T {
                    value: (first.value + second.value) / 2,
                }
            }
        }

        #[allow(non_camel_case_types)]
        #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        pub struct $dT {
            pub step: i32,
        }

        impl $dT {
            pub fn new(step: i32) -> Self {
                $dT { step }
            }
            pub fn abs(&self) -> Self {
                if self.step < 0 {
                    return $dT { step: self.step };
                }
                self.clone()
            }
        }

        impl Sub<$T> for $T {
            type Output = $dT;
            fn sub(self, other: $T) -> Self::Output {
                $dT {
                    step: (self.value as i32) - (other.value as i32),
                }
            }
        }

        impl Sub<$dT> for $T {
            type Output = $T;
            fn sub(self, other: $dT) -> Self::Output {
                if (self.value as i32) < other.step {
                    $T { value: 0 }
                } else {
                    $T {
                        value: ((self.value as i32) - other.step) as usize,
                    }
                }
            }
        }

        impl Add<usize> for $T {
            type Output = $T;
            fn add(self, other: usize) -> Self::Output {
                $T {
                    value: self.value + other,
                }
            }
        }
    };
}

make_dim!(X, dX);
make_dim!(Y, dY);
