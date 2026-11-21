//! A cone frustum on the z axis, the base the disk of radius 6 at z = 0 and the side the cone
//! 2r + z = 12, its top rim beveled: the frustum from radius 6 at z = 0 to 3.5 at z = 5 joined to
//! the frustum from radius 3.5 at z = 5 to 2 at z = 6. The bevel is deleted: the top runs on
//! outward and the side up until they meet in the circle of radius 3 at z = 6.
use crate::bench::{frame, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cone;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let body = Cone::new(6.0, 3.5, 5.0).build(m)?;
    let bevel = Cone::new(3.5, 2.0, 1.0).with_frame(frame(p(0.0, 0.0, 5.0), Direction::Z)).build(m)?;
    let beveled = boolean::union(m, body, bevel)?;
    // The bevel halfway up, at radius 2.75.
    deleted(m, beveled, &[p(0.0, 2.75, 5.5)])
}
