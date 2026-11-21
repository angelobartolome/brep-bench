//! The box from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 4 about the z axis sunk
//! from its top, its floor beveled: the cutter is the cylinder of radius 4 from z = 5 up through
//! the top joined to the frustum from radius 3 at z = 4 to radius 4 at z = 5. The bevel is
//! deleted: the floor runs on outward and the wall down until they meet in the circle of radius 4
//! at z = 4.
use crate::bench::{frame, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cone, Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(-10.0, -10.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let bore = Cylinder::new(4.0, 6.0).with_frame(frame(p(0.0, 0.0, 5.0), Direction::Z)).build(m)?;
    let bevel = Cone::new(3.0, 4.0, 1.0).with_frame(frame(p(0.0, 0.0, 4.0), Direction::Z)).build(m)?;
    let cutter = boolean::union(m, bore, bevel)?;
    let bored = boolean::difference(m, block, cutter)?;
    // The bevel halfway down, at radius 3.5.
    deleted(m, bored, &[p(0.0, 3.5, 4.5)])
}
