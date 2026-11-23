//! The L through (0, 0), (20, 0), (20, 4), (6, 4), (6, 12), (0, 12) in (x, z) extruded from
//! y = 0 to y = 10, its inside edge from (6, 0, 4) to (6, 10, 4) rounded with the kernel's
//! variable-radius fillet under a linear law over the edge's length, from 1 at y = 0 to 3 at
//! y = 10. The edge is selected by its middle.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges_variable_law, make_line, BrepSolid, LawSegment, RadiusLaw};

pub fn build() -> Result<BrepSolid, Refusal> {
    let l = [(0.0, 0.0), (20.0, 0.0), (20.0, 4.0), (6.0, 4.0), (6.0, 12.0), (0.0, 12.0)];
    let lines = (0..l.len()).map(|k| { let (a, b) = (l[k], l[(k + 1) % l.len()]); make_line(p(a.0, 0.0, a.1), p(b.0, 0.0, b.1)) }).collect::<Result<Vec<_>, _>>()?;
    let block = extrude_profile_brep(&lines, p(0.0, 1.0, 0.0), 10.0)?;
    let law = RadiusLaw::from_segments(&[LawSegment::Linear { length: 10.0, start_radius: 1.0, end_radius: 3.0 }])?;
    Ok(fillet_edges_variable_law(&block, &[p(6.0, 5.0, 4.0)], None, &law, false, None)?)
}
