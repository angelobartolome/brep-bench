//! The prism from z = 0 to z = 6 on the periodic uniform cubic B-spline of the case's eight
//! control points (the bean), the spline where the side meets the top rounded to radius 1. The
//! edge is selected by the point of the rim 0.37 of the way round the spline's parameter.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, fillet_edges, make_periodic_bspline, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let poles = [(-10.0, -5.0), (0.0, -7.0), (10.0, -5.0), (12.0, 4.0), (4.0, 7.0), (0.0, 3.0), (-4.0, 7.0), (-12.0, 4.0)];
    let poles: Vec<_> = poles.iter().map(|&(x, y)| p(x, y, 0.0)).collect();
    let bean = make_periodic_bspline(3, &poles)?;
    let prism = extrude_profile_brep(&[bean.clone()], p(0.0, 0.0, 1.0), 6.0)?;
    let [t0, t1] = bean.domain()?;
    let at = bean.evaluate(t0 + 0.37 * (t1 - t0))?;
    Ok(fillet_edges(&prism, &[p(at.x, at.y, 6.0)], None, 1.0, false, None)?)
}
