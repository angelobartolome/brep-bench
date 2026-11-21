//! The puck, the cylinder of radius 6 on the z axis from z = 0 to z = 10 with both rims rounded to
//! radius 2, shrunk by 3: every point at least 3 from its outside. The
//! rounds turn inside out and drop: the plain cylinder of radius 3 from z = 3 to z = 7.
use crate::Refusal;
use crate::shared::offset::rounded_puck;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let puck = rounded_puck(m, true)?;
    Ok(offset::shape(m, puck, -3.0, Join::Arc)?)
}
