//! A plate from (−15, −10, 0) to (15, 10, 4) with a hole right through it on the ellipse
//! x²/6² + y²/3² = 1, the hole's rim on the top face chamfered by 1 on both faces: the box less the
//! kernel's prism on the exact rational ellipse, and its chamfer of the rim at z = 4, a
//! countersink of constant slope. The rim on the bottom stays sharp.
use crate::solids::fillet::subtract;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, make_box_brep, make_ellipse, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-15.0, -10.0, 0.0), 30.0, 20.0, 4.0)?;
    // The hole's tool reaches past both faces.
    let rim = make_ellipse(p(0.0, 0.0, -1.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), 6.0, 3.0)?;
    let tool = extrude_profile_brep(&[rim], p(0.0, 0.0, 1.0), 6.0)?;
    let holed = subtract(&plate, &tool)?;
    // The top rim, by its point on the minor axis.
    Ok(fillet_edges(&holed, &[p(0.0, 3.0, 4.0)], None, 1.0, true, None)?)
}
