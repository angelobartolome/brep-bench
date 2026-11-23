//! The elliptic cylinder on x²/10² + y²/6² = 1 from z = 0 to z = 8 (the ellipse extruded, its
//! side one face), the ellipse where its side meets its top rounded to radius 2 by the kernel's
//! fillet: a pipe along the rim. The edge is selected by a point on it off the seam.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, make_ellipse, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let ellipse = make_ellipse(p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), 10.0, 6.0)?;
    let solid = extrude_profile_brep(&[ellipse], p(0.0, 0.0, 1.0), 8.0)?;
    let a = 1.0f64;
    Ok(fillet_edges(&solid, &[p(10.0 * a.cos(), 6.0 * a.sin(), 8.0)], None, 2.0, false, None)?)
}
