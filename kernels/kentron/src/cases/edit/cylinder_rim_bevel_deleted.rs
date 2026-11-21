//! A cylinder of radius 6 on the z axis from z = 0 to z = 10, its top rim beveled 2 back on each
//! face: the cylinder from z = 0 to z = 8 with the frustum from radius 6 at z = 8 to radius 4 at
//! z = 10 on it. The bevel, the frustum's side, is deleted: the top and the side run on to meet,
//! and the result is the plain cylinder.
use crate::bench::{frame, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cone, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let body = Cylinder::new(6.0, 8.0).build(m)?;
    let bevel = Cone::new(6.0, 4.0, 2.0).with_frame(frame(p(0.0, 0.0, 8.0), Direction::Z)).build(m)?;
    let beveled = boolean::union(m, body, bevel)?;
    deleted(m, beveled, &[p(0.0, 5.0, 9.0)])
}
