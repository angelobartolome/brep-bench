//! Two balls of radius 5 about (−3, 0, 0) and (3, 0, 0), each bounded by the standard biquadratic
//! NURBS sphere moved to its center (no sphere is used), and the lens common to both: two caps of
//! height 2 base to base, meeting in the circle of radius 4 in the plane x = 0.
use crate::bench::p;
use crate::shared::nurbs::nurbs_ball;
use crate::Refusal;
use kentron::boolean;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let left = nurbs_ball(m, p(-3.0, 0.0, 0.0), 5.0)?;
    let right = nurbs_ball(m, p(3.0, 0.0, 0.0), 5.0)?;
    Ok(boolean::intersection(m, left, right)?)
}
