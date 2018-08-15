
/// These X and Y are indices (unsigned integers), not physical distances.

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct X {
    value: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Y {
    value: usize,
}

//noinspection RsStructNaming
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct dX(usize);

//noinspection RsStructNaming
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct dY(usize);

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Dist {
    value: f64,
}

impl Dist {
    pub fn new(value: f64) -> Option<Self> {
        if (value < 0) {
            return None;
        }
        Some(Dist { value })
    }

    pub fn fnew(value: f64) -> Self {
        Self::new(value).unwrap()
    }
}
