//! The prism from z = 0 to z = 6 on the periodic uniform cubic B-spline of eight control points, a
//! bean dented at the top, the spline round its top face chamfered by 1 on both faces: the
//! kernel's prism on the spline and its chamfer of the rim, convex round the bean's ends and
//! concave through the dent.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, make_periodic_bspline, BrepSolid};

const POLES: [(f64, f64); 8] =
    [(-10.0, -5.0), (0.0, -7.0), (10.0, -5.0), (12.0, 4.0), (4.0, 7.0), (0.0, 3.0), (-4.0, 7.0), (-12.0, 4.0)];

pub fn build() -> Result<BrepSolid, Refusal> {
    let poles: Vec<_> = POLES.iter().map(|&(x, y)| p(x, y, 0.0)).collect();
    let rim = make_periodic_bspline(3, &poles)?;
    let prism = extrude_profile_brep(&[rim.clone()], p(0.0, 0.0, 1.0), 6.0)?;
    // The top rim, by the spline's point half way along its first segment, lifted to z = 6.
    let at = rim.evaluate(0.5)?;
    Ok(fillet_edges(&prism, &[p(at.x, at.y, 6.0)], None, 1.0, true, None)?)
}
