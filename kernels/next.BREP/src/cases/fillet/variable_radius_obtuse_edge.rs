//! The regular hexagonal prism of side 6 about the z axis from z = 0 to z = 10 (corners at
//! (6 cos 60k°, 6 sin 60k°)), its edge from (6, 0, 0) to (6, 0, 10) rounded with the kernel's
//! variable-radius fillet under a linear law over the edge's length, from 1 at z = 0 to 4 at
//! z = 10. The edge is selected by its middle.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges_variable_law, make_line, BrepSolid, LawSegment, RadiusLaw};

pub fn build() -> Result<BrepSolid, Refusal> {
    let corner = |k: usize| { let a = (60.0 * k as f64).to_radians(); p(6.0 * a.cos(), 6.0 * a.sin(), 0.0) };
    let lines = (0..6).map(|k| make_line(corner(k), corner((k + 1) % 6))).collect::<Result<Vec<_>, _>>()?;
    let prism = extrude_profile_brep(&lines, p(0.0, 0.0, 1.0), 10.0)?;
    let law = RadiusLaw::from_segments(&[LawSegment::Linear { length: 10.0, start_radius: 1.0, end_radius: 4.0 }])?;
    Ok(fillet_edges_variable_law(&prism, &[p(6.0, 0.0, 5.0)], None, &law, false, None)?)
}
