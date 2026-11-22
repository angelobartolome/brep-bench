//! An elliptic cylinder from z = 0 to z = 8 on the ellipse x²/10² + y²/6² = 1, the ellipse round
//! its top face chamfered by 2 on both faces: the kernel's prism on the exact rational ellipse,
//! and its chamfer of the rim, which sets the distance back on the top and down the side square
//! to the rim.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, make_ellipse, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let rim = make_ellipse(p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), 10.0, 6.0)?;
    let prism = extrude_profile_brep(&[rim], p(0.0, 0.0, 1.0), 8.0)?;
    // The rim, by its point on the minor axis; a right-angled edge, so the chamfer's distance is
    // its setback on both faces.
    Ok(fillet_edges(&prism, &[p(0.0, 6.0, 8.0)], None, 2.0, true, None)?)
}
