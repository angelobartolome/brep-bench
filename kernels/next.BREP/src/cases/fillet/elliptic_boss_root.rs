//! The plate from (−20, −15, 0) to (20, 15, 4) joined to the elliptic boss on x²/10² + y²/6² = 1
//! from z = 4 to z = 10, the boss's root, the ellipse where its side meets the plate's top, rounded
//! to radius 2 (concave: the round adds material). The edge is selected by a point on it off the
//! seam.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{boolean_operation, extrude_profile_brep, fillet_edges, make_box_brep, make_ellipse, BooleanOperation, BooleanOptions, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-20.0, -15.0, 0.0), 40.0, 30.0, 4.0)?;
    let ellipse = make_ellipse(p(0.0, 0.0, 4.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), 10.0, 6.0)?;
    let boss = extrude_profile_brep(&[ellipse], p(0.0, 0.0, 1.0), 6.0)?;
    let joined = boolean_operation(&plate, &boss, BooleanOperation::Union, &BooleanOptions::default())?;
    let a = 1.0f64;
    Ok(fillet_edges(&joined, &[p(10.0 * a.cos(), 6.0 * a.sin(), 4.0)], None, 2.0, false, None)?)
}
