//! The box from the origin to (20, 10, 10) with a slot cut into its side y = 0, top to bottom: from
//! x = 8 to x = 12, running in to y = 5 and closed there by a half cylinder of radius 2 about the
//! vertical line through (10, 5). The slot's two flats and half cylinder are deleted: the side runs
//! on across the mouth, and the result is the box.
use crate::bench::{frame, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(20.0, 10.0, 10.0)).build(m)?;
    let straight = Cuboid::from_corners(p(8.0, -1.0, -1.0), p(12.0, 5.0, 11.0)).build(m)?;
    let end = Cylinder::new(2.0, 12.0).with_frame(frame(p(10.0, 5.0, -1.0), Direction::Z)).build(m)?;
    let slot = boolean::union(m, straight, end)?;
    let slotted = boolean::difference(m, block, slot)?;
    deleted(m, slotted, &[p(8.0, 2.0, 5.0), p(12.0, 2.0, 5.0), p(10.0, 7.0, 5.0)])
}
