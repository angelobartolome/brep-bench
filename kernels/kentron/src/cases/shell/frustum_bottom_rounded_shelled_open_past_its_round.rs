//! The cone frustum on the z axis from the disc of radius 6 at z = 0 to the disc of radius 3 at z = 6,
//! its bottom rim rounded to radius 1.5, shelled inward to walls 2 thick, open at its top. The round
//! moved inward turns inside out and drops: the cavity is the frustum from z = 2 up through the top,
//! its side the cone moved 2 inward.
use crate::Refusal;
use crate::bench::p;
use crate::shared::offset::rounded_frustum;
use crate::shared::shell;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let solid = rounded_frustum(m, None, Some(1.5))?;
    // open at the top, z = 6
    shell::inward(m, solid, &[p(0.0, 0.0, 6.0)], 2.0, Join::Arc)
}
