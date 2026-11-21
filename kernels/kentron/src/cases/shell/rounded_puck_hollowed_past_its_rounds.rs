//! The puck, the cylinder of radius 6 on the z axis from z = 0 to z = 10 with both rims rounded to
//! radius 2, hollowed to walls 3 thick. The rounds moved inward drop: the cavity is the cylinder of
//! radius 3 from z = 3 to z = 7.
use crate::Refusal;
use crate::shared::offset::rounded_puck;
use crate::shared::shell;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let puck = rounded_puck(m, true)?;
    shell::hollowed(m, puck, 3.0)
}
