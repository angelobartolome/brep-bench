//! The bean of `fillet/spline-boss-rim`, the prism from z = 0 to z = 6 on a periodic uniform
//! cubic B-spline dented at the top, its top rim chamfered by 1 on both faces: a ruled surface of
//! constant slope 45°, convex round the bean's ends and concave through the dent.
use crate::bench::{edge_through, p};
use crate::shared::fillet::spline_boss;
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let boss = spline_boss(m)?;
    // (A point on the rim away from the knots, where the side's seam may be: the middle of the
    // segment on the first four control points, their average weighted 1, 23, 23, 1.)
    let rim = edge_through(m, boss, p(232.0 / 48.0, -277.0 / 48.0, 6.0))?;
    Ok(blend::chamfer(m, boss, &[rim], 1.0)?)
}
