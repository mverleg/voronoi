use std::ops::{Add, Sub};
use norms::Dist;

/// These X and Y are indices (unsigned integers), not physical distances.

pub trait Dim {
    /// Expose the internal value. Careful with trying to use this to get around type safety.
    fn _expose(&self) -> i32;
}

macro_rules! make_dim {
    ( $T:ident, $dT:ident ) => {
        #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        pub struct $T {
            pub value: i32,
        }

        impl $T {
            pub fn new(value: i32) -> Self {
                $T { value }
            }

//            /// Returns the highest $T that is within Dist below `self`, but still positive.
//            pub fn margin_down(self, margin: Dist) -> Self {
//                let margin = margin._expose().floor() as i32;
//                // TODO: see issue #1
//                if margin >= self.value {
//                    return $T { value: 0 }
//                }
//                $T { value: self.value - margin }
//            }
        }

        impl Dim for $T {
            fn _expose(&self) -> i32 {
                self.value
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
                    return $dT { step: self.step }
                }
                self.clone()
            }
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
                    $T { value: self.value - other.step }
                }
            }
        }

        impl Add<$dT> for $T {
            type Output = $T;

            fn add(self, other: $dT) -> Self::Output {
                if self.value < -other.step {
                    $T { value: 0 }
                } else {
                    $T { value: self.value + other.step }
                }
            }
        }

        impl Add<usize> for $T {
            type Output = $T;

            fn add(self, other: usize) -> Self::Output {
                $T { value: self.value + other as i32 }
            }
        }

        impl Sub<usize> for $T {
            type Output = $T;

            fn sub(self, other: usize) -> Self::Output {
                // TODO
                $T { value: self.value - other as i32 }
            }
        }

//        impl Mul<$T> for $T {
//            type Output = Dist;
//
//            fn mul(self, other: $T) -> Self::Output {
//                Dist { value: self.value * other.value }
//            }
//        }
    }
}

make_dim!(X, dX);
make_dim!(Y, dY);

//TODO @mark: re-enable
//impl Mul<Y> for X {
//    type Output = Dist;
//
//    fn mul(self, other: Y) -> Self::Output {
//        Dist { value: (self.value * other.value) as f64 }
//    }
//}

//TODO @mark: re-enable
//impl Mul<dY> for dX {
//    type Output = Dist;
//
//    fn mul(self, other: dY) -> Self::Output {
//        Dist { value: (self.step + other.step) as f64 }
//    }
//}

//impl Add<dY> for dX {
//    type Output = Dist;
//
//    fn add(self, other: dY) -> Self::Output {
//        Dist { value: (self.step + other.step) as f64 }
//    }
//}
