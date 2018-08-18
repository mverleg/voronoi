
use std::ops::{Add, Sub, Mul};

/// These X and Y are indices (unsigned integers), not physical distances.

macro_rules! make_dim {
    ( $T:ident, $dT:ident ) => {
        #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
        pub struct $T {
            value: i32,
        }

        impl $T {
            pub fn new(value: i32) -> Self {
                $T { value }
            }

            /// Expose the internal usize. Should only be used when required for external code.
            pub fn _expose(&self) -> i32 {
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
//        //TODO @mark: this is unsafe, could be negative
//        Dist { value: (self.step + other.step) as f64 }
//    }
//}
