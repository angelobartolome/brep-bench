//! The half of the cylinder of radius 5 on the z axis from z = 0 to z = 6 on the side x ≥ 0 (the
//! cylinder less the box over x < 0), the half circle where its curved side meets its top rounded
//! by the kernel's variable-radius fillet under a linear law over the edge's length 5π, from 1 to
//! 2 (r = 3/2 + φ/π, linear in the angle as in arc length). The edge is selected by a point on it.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{boolean_operation, fillet_edges_variable_law, make_box_brep, make_cylinder_brep, BooleanOperation, BooleanOptions, BrepSolid, LawSegment, RadiusLaw};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 6.0)?;
    let half = make_box_brep(p(-6.0, -6.0, -1.0), 6.0, 12.0, 8.0)?;
    let halved = boolean_operation(&cylinder, &half, BooleanOperation::Subtract, &BooleanOptions::default())?;
    let law = RadiusLaw::from_segments(&[LawSegment::Linear { length: 5.0 * std::f64::consts::PI, start_radius: 1.0, end_radius: 2.0 }])?;
    Ok(fillet_edges_variable_law(&halved, &[p(5.0 * 0.3f64.cos(), 5.0 * 0.3f64.sin(), 6.0)], None, &law, false, None)?)
}
