use dims::{dX, dY};
use dims::Dim;
use points::Step2D;
use std::ops::Add;
use std::ops::Mul;

/// Trait for objects that have a length (norm). Specifically L1, L2 and L3.
#[allow(non_snake_case)]
pub trait Norm {
    // Default implementations assume that pseudo-norm is just |x|^n + |y|^n without any roots taken.
    fn manhattan_norm(&self) -> Dist {
        Dist { value: self.manhattan_pseudo().value }
    }
    fn euclidean_norm(&self) -> Dist {
        Dist { value: self.euclidean_pseudo().value.sqrt() }
    }
    fn L3_norm(&self) -> Dist {
        Dist { value: self.L3_pseudo().value.cbrt() }
    }
    fn manhattan_pseudo(&self) -> PseudoDist;
    fn euclidean_pseudo(&self) -> PseudoDist;
    fn L3_pseudo(&self) -> PseudoDist;
}

#[allow(non_camel_case_types)]
pub type norm = Fn(&dX, &dY) -> Dist;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Dist {
    value: f64,
}

/// Pseudo-distance is not a valid norm, but rather a value with the only guarantees
/// that is has the exact same partial order as the real norm ([Dist]) and is non-negative.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct PseudoDist {
    value: f64,
}

impl PseudoDist {
    pub fn new(value: f64) -> Option<Self> {
        if value < 0.0 {
            return None;
        }
        Some(PseudoDist { value })
    }

    pub fn fnew(value: f64) -> Self {
        Self::new(value).unwrap()
    }
}

impl Dist {
    pub fn new(value: f64) -> Option<Self> {
        if value < 0.0 {
            return None;
        }
        Some(Dist { value })
    }

    pub fn fnew(value: f64) -> Self {
        Self::new(value).unwrap()
    }
}

impl Add<Dist> for Dist {
    type Output = Dist;

    fn add(self, other: Dist) -> Self::Output {
        Dist { value: self.value + other.value }
    }
}

impl Mul<Dist> for Dist {
    type Output = PseudoDist;

    fn mul(self, other: Dist) -> Self::Output {
        PseudoDist { value: self.value * other.value }
    }
}

/// Manhattan (L1) distance for horizontal/vertical edges.
pub fn manhattan<N>(object: &N) -> Dist where N: Norm {
    object.manhattan_norm()
}

/// Euclidean (L2) distance squared for straight edges in any direction (standard Voronoi).
pub fn euclidean<N>(object: &N) -> Dist where N: Norm {
    object.euclidean_norm()
}

#[allow(non_snake_case)]
/// L3 distance cubed for curved edges.
pub fn L3<N>(object: &N) -> Dist where N: Norm {
    object.L3_norm()
}

pub fn pmanhattan<N>(object: &N) -> PseudoDist where N: Norm {
    object.manhattan_pseudo()
}

pub fn peuclidean<N>(object: &N) -> PseudoDist where N: Norm {
    object.euclidean_pseudo()
}

#[allow(non_snake_case)]
pub fn pL3<N>(object: &N) -> PseudoDist where N: Norm {
    object.L3_pseudo()
}

impl Norm for Step2D {
    fn manhattan_pseudo(&self) -> PseudoDist {
        PseudoDist { value: (self.dx.step.abs() + self.dy.step.abs()) as f64 }
    }

    fn euclidean_pseudo(&self) -> PseudoDist {
        PseudoDist { value: (self.dx.step.pow(2) + self.dy.step.pow(2)) as f64 }
    }

    #[allow(non_snake_case)]
    fn L3_pseudo(&self) -> PseudoDist {
        PseudoDist { value: (self.dx.step.abs().pow(3) + self.dy.step.abs().pow(3)) as f64 }
    }
}

impl<D> Norm for D where D: Dim {
    fn manhattan_pseudo(&self) -> PseudoDist {
        PseudoDist { value: self._expose().abs() as f64 }
    }

    fn euclidean_pseudo(&self) -> PseudoDist {
        PseudoDist { value: self._expose().pow(2) as f64 }
    }

    #[allow(non_snake_case)]
    fn L3_pseudo(&self) -> PseudoDist {
        PseudoDist { value: self._expose().pow(3) as f64 }
    }
}

///// Manhattan (L1) distance for horizontal/vertical edges.
//pub fn manhattan(step: Step) -> Dist {
//    let dx = dx.abs();
//    let dy = dy.abs();
//    (dx + dy).into()
//}
//
///// Euclidean (L2) distance squared for straight edges in any direction (standard Voronoi).
//pub fn euclidean(dx: &dX, dy: &dY) -> Dist {
//    let dx = dx.abs();
//    let dy = dy.abs();
//    Dist::fnew(dx*dx + dy*dy)
//}
//
////noinspection RsFunctionNaming
///// L3 distance cubed for curved edges.
//pub fn L3(dx: &dX, dy: &dY) -> Dist {
//    let dx = dx.abs().dist();
//    let dy = dy.abs().dist();
//    Dist::fnew(dx*dx*dx + dy*dy*dy)
//}

#[cfg(test)]
mod tests {
    use points::{Step, Step2D};
    use super::*;

    //TODO @mark: use 'norm' type alias if possible
    fn get_norm_funcs<N>() -> Vec<&'static Fn(&N) -> Dist> where N: Norm {
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
            assert_eq!(Dist::fnew(2f64), f(&Step2D::new(dX::new(1), dY::new(1))));
            assert_eq!(Dist::fnew(10f64), f(&Step2D::new(dX::new(5), dY::new(5))));
        }
    }

    #[test]
    fn test_symmetrical() {
        for f in get_norm_funcs() {
            for a in 0 .. 10 {
                for b in 0 .. 10 {
                    assert_eq!(f(&Step2D::new(dX::new(a), dY::new(b))), f(&Step2D::new(dX::new(b), dY::new(a))));
                }
            }
        }
    }

    #[test]
    fn test_sign_no_effect() {
        for f in get_norm_funcs() {
            for x in 0 .. 10 {
                for y in 0 .. 10 {
                    assert_eq!(f(&Step2D::new(dX::new(x), dY::new(y))), f(&Step2D::new(dX::new(-x), dY::new(-y))));
                }
            }
        }
    }

    #[test]
    fn test_manhattan() {
        assert_eq!(Dist::fnew(4f64), manhattan(&Step2D::new(dX::new(2), dY::new(2))));
    }

    #[test]
    fn test_euclidean() {
        assert_eq!(Dist::fnew(4f64), euclidean(&Step2D::new(dX::new(2), dY::new(2))));
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_L3() {
        assert_eq!(Dist::fnew(4f64), L3(&Step2D::new(dX::new(2), dY::new(2))));
    }
}