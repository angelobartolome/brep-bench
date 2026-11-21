//! The cone frustum on the z axis from the disc of radius 6 at z = 0 to the disc of radius 3 at z = 6,
//! its top rim rounded to radius 1.5, shrunk by 2: every point at least 2 from its outside. The round
//! moved inward turns inside out and drops: the result is the sharp frustum shrunk by 2.
use crate::Refusal;
use crate::shared::offset::rounded_frustum;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let frustum = rounded_frustum(m, Some(1.5), None)?;
    Ok(offset::shape(m, frustum, -2.0, Join::Arc)?)
}
