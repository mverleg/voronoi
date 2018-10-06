use dims::{X, Y};
use point::Step2D;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

/// Trait for objects that have a length (norm). Specifically L1, L2 and L3.
#[allow(non_snake_case)]
pub trait Norm {
    // Default implementations assume that pseudo-norm is just |x|^n + |y|^n without any roots taken.
    fn manhattan_norm(&self) -> Dist {
        Dist {
            value: self.manhattan_pseudo().value,
        }
    }
    fn euclidean_norm(&self) -> Dist {
        Dist {
            value: self.euclidean_pseudo().value.sqrt(),
        }
    }
    fn L3_norm(&self) -> Dist {
        Dist {
            value: self.L3_pseudo().value.cbrt(),
        }
    }
    fn manhattan_pseudo(&self) -> PseudoDist;
    fn euclidean_pseudo(&self) -> PseudoDist;
    fn L3_pseudo(&self) -> PseudoDist;
}

macro_rules! make_dist {
    ( $T:ident ) => {
        #[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
        pub struct $T {
            value: f64,
        }

        impl $T {
            #[allow(dead_code)]  // not really dead
            pub fn new(value: f64) -> Option<Self> {
                if value < 0.0 {
                    return None;
                }
                Some($T { value })
            }

            #[allow(dead_code)]  // not really dead
            pub fn fnew(value: f64) -> Self {
                Self::new(value).unwrap()
            }

            #[allow(dead_code)]  // not really dead
            pub fn ufloor(&self) -> usize {
                self.value.floor().abs() as usize
            }
        }

        impl Add<$T> for $T {
            type Output = $T;
            fn add(self, other: $T) -> Self::Output {
                $T {
                    value: self.value + other.value,
                }
            }
        }

        impl Sub<$T> for $T {
            type Output = $T;
            fn sub(self, other: $T) -> Self::Output {
                $T {
                    value: self.value - other.value,
                }
            }
        }

        impl Mul<$T> for $T {
            type Output = $T;
            fn mul(self, other: $T) -> Self::Output {
                $T {
                    value: self.value * other.value,
                }
            }
        }
    };
}

make_dist!(Dist);
/// Pseudo-distance is not a valid norm, but rather a value with the only guarantees
/// that is has the exact same partial order as the real norm ([Dist]) and is non-negative.
make_dist!(PseudoDist);

/// Manhattan (L1) distance for horizontal/vertical edges.
#[allow(dead_code)]
pub fn manhattan<N>(object: &N) -> Dist
where
    N: Norm,
{
    object.manhattan_norm()
}

/// Euclidean (L2) distance squared for straight edges in any direction (standard Voronoi).
#[allow(dead_code)]
pub fn euclidean<N>(object: &N) -> Dist
where
    N: Norm,
{
    object.euclidean_norm()
}

#[allow(non_snake_case)]
#[allow(dead_code)]
/// L3 distance cubed for curved edges.
pub fn L3<N>(object: &N) -> Dist
where
    N: Norm,
{
    object.L3_norm()
}

#[allow(dead_code)]
pub fn pmanhattan<N>(object: &N) -> PseudoDist
where
    N: Norm,
{
    object.manhattan_pseudo()
}
#[allow(dead_code)]
pub fn peuclidean<N>(object: &N) -> PseudoDist
where
    N: Norm,
{
    object.euclidean_pseudo()
}

#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn pL3<N>(object: &N) -> PseudoDist
where
    N: Norm,
{
    object.L3_pseudo()
}

impl Norm for Step2D {
    fn manhattan_pseudo(&self) -> PseudoDist {
        PseudoDist {
            value: (self.dx.step.abs() + self.dy.step.abs()) as f64,
        }
    }
    fn euclidean_pseudo(&self) -> PseudoDist {
        PseudoDist {
            value: (self.dx.step.pow(2) + self.dy.step.pow(2)) as f64,
        }
    }

    #[allow(non_snake_case)]
    fn L3_pseudo(&self) -> PseudoDist {
        PseudoDist {
            value: (self.dx.step * self.dx.step * self.dx.step.abs()
                + self.dy.step * self.dy.step * self.dy.step.abs()) as f64,
        }
    }
}

macro_rules! impl_norm_for_dim {
    ( $T:ident ) => {
        impl Norm for $T {
            fn manhattan_pseudo(&self) -> PseudoDist {
                PseudoDist {
                    value: self.value as f64,
                }
            }
            fn euclidean_pseudo(&self) -> PseudoDist {
                PseudoDist {
                    value: self.value.pow(2) as f64,
                }
            }
            #[allow(non_snake_case)]
            fn L3_pseudo(&self) -> PseudoDist {
                PseudoDist {
                    value: self.value.pow(3) as f64,
                }
            }
        }
    };
}
impl_norm_for_dim!(X);
impl_norm_for_dim!(Y);

#[cfg(test)]
mod tests {
    use super::*;
    use point::{Step, Step2D};
    use std::f64::consts::PI;

    fn get_norm_funcs<N>() -> Vec<&'static Fn(&N) -> Dist>
    where
        N: Norm,
    {
        let mut funcs: Vec<&'static Fn(&N) -> Dist> = Vec::with_capacity(3);
        funcs.push(&manhattan);
        funcs.push(&euclidean);
        funcs.push(&L3);
        funcs
    }

    #[test]
    fn test_pure_axis() {
        for f in get_norm_funcs() {
            assert_eq!(Dist::fnew(1f64), f(&Step2D::new(dX::new(1), dY::new(0))));
            assert_eq!(Dist::fnew(5f64), f(&Step2D::new(dX::new(5), dY::new(0))));
            assert_eq!(Dist::fnew(1f64), f(&Step2D::new(dX::new(0), dY::new(1))));
            assert_eq!(Dist::fnew(5f64), f(&Step2D::new(dX::new(0), dY::new(5))));
        }
    }

    #[test]
    fn test_bounding_box() {
        for f in get_norm_funcs() {
            assert!(Dist::fnew(2f64) >= f(&Step2D::new(dX::new(1), dY::new(1))));
            assert!(Dist::fnew(10f64) >= f(&Step2D::new(dX::new(5), dY::new(5))));
        }
    }

    #[test]
    fn test_symmetrical() {
        for f in get_norm_funcs() {
            for a in 0..10 {
                for b in 0..10 {
                    assert_eq!(
                        f(&Step2D::new(dX::new(a), dY::new(b))),
                        f(&Step2D::new(dX::new(b), dY::new(a)))
                    );
                }
            }
        }
    }

    #[test]
    fn test_sign_no_effect() {
        for f in get_norm_funcs() {
            for x in 0..10 {
                for y in 0..10 {
                    assert_eq!(
                        f(&Step2D::new(dX::new(x), dY::new(y))),
                        f(&Step2D::new(dX::new(-x), dY::new(-y)))
                    );
                }
            }
        }
    }

    #[test]
    fn test_manhattan() {
        assert_approx_eq!(
            Dist::fnew(4f64),
            manhattan(&Step2D::new(dX::new(2), dY::new(2))),
            Dist::fnew(1e-10)
        );
    }

    #[test]
    fn test_euclidean() {
        assert_approx_eq!(
            Dist::fnew(4. * (PI / 4.).cos()),
            euclidean(&Step2D::new(dX::new(2), dY::new(2))),
            Dist::fnew(1e-10)
        );
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_L3() {
        assert_approx_eq!(
            Dist::fnew(16f64.powf(1. / 3.)),
            L3(&Step2D::new(dX::new(2), dY::new(2))),
            Dist::fnew(1e-10)
        );
    }
}
