//! The cone on the z axis, its base of radius 6 at z = 0 and its apex at z = 12, its tip rounded by
//! the ball of radius 1 tangent to its side, hollowed to a wall 1.5 thick with no opening. The ball
//! moved inward would have a radius below nothing and drops: the cavity is the plain cone of the side
//! moved 1.5 inward, on the base moved up to z = 1.5.
use crate::Refusal;
use crate::shared::offset::ball_tipped_cone;
use crate::shared::shell;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cone = ball_tipped_cone(m)?;
    shell::hollowed(m, cone, 1.5)
}
