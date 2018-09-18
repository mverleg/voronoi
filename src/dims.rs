use find_index::Mid;
use std::ops::{Add, Sub};

/// These X and Y are indices (unsigned integers), not physical distances.

pub trait Dim {
    /// Expose the internal value. Careful with trying to use this to get around type safety.
    fn _expose(&self) -> usize;
}

macro_rules! make_dim {
    ( $T:ident, $dT:ident ) => {
        #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        pub struct $T {
            pub value: usize,
        }

        impl $T {
            pub fn new(value: usize) -> Self {
                $T { value }
            }

            //            /// Returns the highest $T that is within Dist below `self`, but still positive.
            //            pub fn margin_down(self, margin: Dist) -> Self {
            //                let margin = margin._expose().floor() as usize;
            //                // TODO: see issue #1
            //                if margin >= self.value {
            //                    return $T { value: 0 }
            //                }
            //                $T { value: self.value - margin }
            //            }

            pub fn as_index(&self) -> usize {
                self.value as usize
            }
        }

        impl Dim for $T {
            fn _expose(&self) -> usize {
                self.value
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
                    $T { value: ((self.value as i32) - other.step) as usize }
                }
            }
        }

        impl Add<$dT> for $T {
            type Output = $T;

            //TODO @mark: test e.g. 2 + -3
            fn add(self, other: $dT) -> Self::Output {
                if (self.value as i32) < -other.step {
                    $T { value: 0 }
                } else {
                    $T { value: self.value + other.step as usize }
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

        impl Sub<usize> for $T {
            type Output = $T;

            fn sub(self, other: usize) -> Self::Output {
                $T {
                    value: self.value - other,
                }
            }
        }
    };
}

make_dim!(X, dX);
make_dim!(Y, dY);
