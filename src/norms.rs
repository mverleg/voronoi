
//noinspection RsTypeAliasNaming
pub type norm = Fun(&dX, &dY) -> Dist;

/// Manhattan (L1) distance for horizontal/vertical edges.
pub fn manhattan(dx: &dX, dy: &dY) -> Dist {
    Dist.fnew(dx.abs() + dy.abs())
}

/// Euclidean (L2) distance squared for straight edges in any direction (standard Voronoi).
pub fn euclidean2(dx: &dX, dy: &dY) -> Dist {
    Dist.fnew(dx*dx + dy*dy)
}

//noinspection RsFunctionNaming
/// L3 distance cubed for curved edges.
pub fn L3(dx: &dX, dy: &dY) -> Dist {
    Dist.fnew(dx*dx*dx + dy*dy*dy)
}

