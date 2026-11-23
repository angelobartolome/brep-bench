//! The half (x ≥ 0) of the cylinder of radius 5 on the z axis from z = 0 to z = 6, its curved top
//! rim chamfered by the same distance on both faces, growing linearly round it, d = 3/2 + φ/π: the
//! kernel's variable chamfer under a linear law over the rim's length 5π, from 1 where the rim
//! starts at (0, −5, 6) to 2 where it ends at (0, 5, 6). (Either way along the rim reads the same
//! solid: each section depends on d alone, and the law reversed is the rim's mirror in y = 0.)
use crate::solids::fillet::subtract;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{fillet_edges_variable_law, make_box_brep, make_cylinder_brep, BrepSolid, LawSegment, RadiusLaw};

pub fn build() -> Result<BrepSolid, Refusal> {
    let cylinder = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 6.0)?;
    let half = subtract(&cylinder, &make_box_brep(p(-6.0, -6.0, -1.0), 6.0, 12.0, 8.0)?)?;
    let law = RadiusLaw::from_segments(&[LawSegment::Linear {
        length: 5.0 * std::f64::consts::PI,
        start_radius: 1.0,
        end_radius: 2.0,
    }])?;
    // The rim, by its point on the x axis.
    Ok(fillet_edges_variable_law(&half, &[p(5.0, 0.0, 6.0)], None, &law, true, None)?)
}
