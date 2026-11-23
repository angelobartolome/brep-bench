//! The plate from (−15, −10, 0) to (15, 10, 4) less the elliptic cylinder on x²/6² + y²/3² = 1
//! (a hole right through it), the hole's rim on the top face rounded to radius 1. The edge is
//! selected by a point on it off the seam.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{boolean_operation, extrude_profile_brep, fillet_edges, make_box_brep, make_ellipse, BooleanOperation, BooleanOptions, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-15.0, -10.0, 0.0), 30.0, 20.0, 4.0)?;
    let ellipse = make_ellipse(p(0.0, 0.0, -1.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), 6.0, 3.0)?;
    let bore = extrude_profile_brep(&[ellipse], p(0.0, 0.0, 1.0), 6.0)?;
    let holed = boolean_operation(&plate, &bore, BooleanOperation::Subtract, &BooleanOptions::default())?;
    let a = 1.0f64;
    Ok(fillet_edges(&holed, &[p(6.0 * a.cos(), 3.0 * a.sin(), 4.0)], None, 1.0, false, None)?)
}
