//! The cone on the z axis, its base of radius 6 at z = 0 and its apex at z = 12, its tip rounded by
//! the ball of radius 1 tangent to its side, shrunk by 1.5: every point at least 1.5 from its outside.
//! The ball moved inward would have a radius below nothing and drops; the moved side runs on to its
//! own apex, 1.5√5 lower, and the result is a plain cone on the base moved up to z = 1.5.
use crate::Refusal;
use crate::shared::offset::ball_tipped_cone;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cone = ball_tipped_cone(m)?;
    Ok(offset::shape(m, cone, -1.5, Join::Arc)?)
}
