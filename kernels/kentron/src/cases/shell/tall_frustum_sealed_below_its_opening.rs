//! The cone frustum on the z axis from the disc of radius 7 at z = 0 to the disc of radius 1 at z = 8,
//! shelled inward to a wall 2 thick, open at its top. The side moved 2
//! inward is the cone with its apex at (0, 0, 6), which meets the base moved up to z = 2 below the
//! opening: the cavity is sealed under the top.
use crate::Refusal;
use crate::bench::p;
use crate::shared::offset::frustum;
use crate::shared::shell;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let solid = frustum(m, 7.0, 1.0, 8.0)?;
    // open at the top, z = 8; the frustum's edges are convex, where moved faces meet rather than
    // part, and the moved faces run on until they meet
    shell::inward(m, solid, &[p(0.0, 0.0, 8.0)], 2.0, Join::Arc)
}
