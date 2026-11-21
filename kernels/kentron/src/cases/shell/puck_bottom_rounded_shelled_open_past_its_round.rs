//! The puck, the cylinder of radius 6 on the z axis from z = 0 to z = 10, its bottom rim rounded to
//! radius 2, shelled inward to walls 3 thick, open at its top. The round moved inward turns inside out
//! and drops: the cavity is the cylinder of radius 3 from z = 3 up through the top.
use crate::Refusal;
use crate::bench::p;
use crate::shared::offset::rounded_puck;
use crate::shared::shell;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let puck = rounded_puck(m, false)?;
    // open at the top, z = 10
    shell::inward(m, puck, &[p(0.0, 0.0, 10.0)], 3.0, Join::Arc)
}
