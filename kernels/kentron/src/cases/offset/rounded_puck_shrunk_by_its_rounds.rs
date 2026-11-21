//! The puck, the cylinder of radius 6 on the z axis from z = 0 to z = 10 with both rims rounded to
//! radius 2, shrunk by 2: every point at least 2 from its outside. Each round collapses onto its centre circle, an edge of
//! the result: the plain cylinder of radius 4 from z = 2 to z = 8.
use crate::Refusal;
use crate::shared::offset::rounded_puck;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let puck = rounded_puck(m, true)?;
    Ok(offset::shape(m, puck, -2.0, Join::Arc)?)
}
