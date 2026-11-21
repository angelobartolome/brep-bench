//! A plate, the disk of radius 8 about the z axis from z = −2 to z = 0, joined to the ball of
//! radius 3 about (0, 0, 1), and the groove where they meet beveled: the frustum from radius 4 at
//! z = 0 to radius 3 at z = 1 joined too. The bevel is deleted: the plate's top runs on inward and
//! the ball down until they meet in the circle of radius √8 at z = 0.
use crate::bench::{frame, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cone, Cylinder, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = Cylinder::new(8.0, 2.0).with_frame(frame(p(0.0, 0.0, -2.0), Direction::Z)).build(m)?;
    let ball = Sphere::new(3.0).with_frame(frame(p(0.0, 0.0, 1.0), Direction::Z)).build(m)?;
    let bevel = Cone::new(4.0, 3.0, 1.0).build(m)?;
    let part = boolean::union(m, plate, ball)?;
    let beveled = boolean::union(m, part, bevel)?;
    // The bevel halfway up, at radius 3.5.
    deleted(m, beveled, &[p(0.0, 3.5, 0.5)])
}
