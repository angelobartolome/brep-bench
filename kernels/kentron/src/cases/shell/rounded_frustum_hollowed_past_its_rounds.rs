//! The cone frustum on the z axis from the disc of radius 6 at z = 0 to the disc of radius 3 at z = 6,
//! both rims rounded to radius 1.5, hollowed to walls 2 thick. The rounds moved inward drop: the
//! cavity is the sharp frustum shrunk by 2, from z = 2 to z = 4.
use crate::Refusal;
use crate::shared::offset::rounded_frustum;
use crate::shared::shell;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let solid = rounded_frustum(m, Some(1.5), Some(1.5))?;
    shell::hollowed(m, solid, 2.0)
}
