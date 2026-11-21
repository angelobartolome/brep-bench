//! The cone on the z axis, its base of radius 6 at z = 0 and its apex at z = 12, its tip rounded by
//! the ball of radius 1 tangent to its side, shelled inward to a wall 1.5 thick, open at its base. The
//! ball moved inward drops; the moved side runs on to its own apex and down through the base's plane.
use crate::Refusal;
use crate::bench::p;
use crate::shared::offset::ball_tipped_cone;
use crate::shared::shell;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cone = ball_tipped_cone(m)?;
    // open at the base, z = 0
    shell::inward(m, cone, &[p(0.0, 3.0, 0.0)], 1.5, Join::Arc)
}
