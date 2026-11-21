//! A hub, the tube about the z axis from z = 0 to z = 20, outer radius 10, bore radius 4, with a
//! keyway cut along the bore's whole length: the box −1 ≤ x ≤ 1, −5 ≤ y ≤ 0 cut away. The
//! keyway's floor and two walls are deleted: the bore closes over it, the ends grow back to whole
//! rings, and the result is the plain hub.
use crate::bench::{frame, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let outside = Cylinder::new(10.0, 20.0).build(m)?;
    let bore = Cylinder::new(4.0, 22.0).with_frame(frame(p(0.0, 0.0, -1.0), Direction::Z)).build(m)?;
    let hub = boolean::difference(m, outside, bore)?;
    let key = Cuboid::from_corners(p(-1.0, -5.0, -1.0), p(1.0, 0.0, 21.0)).build(m)?;
    let keyed = boolean::difference(m, hub, key)?;
    deleted(m, keyed, &[p(0.0, -5.0, 10.0), p(-1.0, -4.5, 10.0), p(1.0, -4.5, 10.0)])
}
