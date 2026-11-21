//! The cone frustum on the z axis from the disc of radius 6 at z = 0 to the disc of radius 3 at z = 4,
//! shelled inward to a wall 2.7 thick, open at both its base and its top. The side moved 2.7 inward is
//! the cone with its apex at (0, 0, 3.5), which closes below the top: the base's rim is the annulus
//! out from radius 2.625, and the top is no opening of the cavity.
use crate::Refusal;
use crate::bench::p;
use crate::shared::offset::frustum;
use crate::shared::shell;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let solid = frustum(m, 6.0, 3.0, 4.0)?;
    // open at the base, z = 0, and the top, z = 4; the frustum's edges are convex, where moved
    // faces meet rather than part, and the moved side runs on to its apex
    shell::inward(m, solid, &[p(0.0, 0.0, 0.0), p(0.0, 0.0, 4.0)], 2.7, Join::Arc)
}
