//! The cone frustum on the z axis from the disc of radius 6 at z = 0 to the disc of radius 3 at z = 4,
//! shelled inward to a wall 3.5 thick, open at its top. The base moved up 3.5 is
//! above the apex of the side moved 3.5 inward, at (0, 0, 13/6): nothing is left to hollow, and the
//! result is the frustum itself.
use crate::Refusal;
use crate::bench::p;
use crate::shared::offset::frustum;
use crate::shared::shell;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let solid = frustum(m, 6.0, 3.0, 4.0)?;
    // open at the top, z = 4; the frustum's edges are convex, where moved faces meet rather than
    // part, and the moved faces run on until they meet
    shell::inward(m, solid, &[p(0.0, 0.0, 4.0)], 3.5, Join::Arc)
}
