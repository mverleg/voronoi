
use std::ops::{Add, Sub};

/// These X and Y are indices (unsigned integers), not physical distances.

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
        }

        //noinspection RsStructNaming
        #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        pub struct $dT {
            step: i32,
        }

        impl $dT {
            pub fn new(value: usize) -> Self {
                $dT { value }
            }
        }

        impl Sub<$T> for $T {
            type Output = $dT;

            fn sub(self, other: $T) -> Self::Output {
                $ dT { step: self.value - other.value }
            }
        }

        impl Sub<$dT> for $T {
            type Output = $T;

            fn sub(self, other: $dT) -> Self::Output {
                if self.value < other.step {
                    $ T { value: 0 }
                } else {
                    $ T { value: self.value - other.value }
                }
            }
        }

        impl Add<$dT> for $T {
            type Output = $T;

            fn add(self, other: $dT) -> Self::Output {
                if self.value < -other.step {
                    $ T { value: 0 }
                } else {
                    $ T { value: self.value - other.value }
                }
            }
        }
    }
}

make_dim!(X, dX);
make_dim!(Y, dY);
