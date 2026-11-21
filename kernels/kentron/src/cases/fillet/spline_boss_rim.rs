//! The prism from z = 0 to z = 6 on the periodic uniform cubic B-spline through the control
//! points (−10, −5), (0, −7), (10, −5), (12, 4), (4, 7), (0, 3), (−4, 7), (−12, 4), a bean dented
//! at the top, its top rim rounded to radius 1: a pipe, convex round the ends and saddle-shaped
//! through the dent.
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
    Ok(blend::fillet(m, boss, &[rim], 1.0)?)
}
