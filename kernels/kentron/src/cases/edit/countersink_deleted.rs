//! The box from the origin to (20, 10, 10), drilled right through along z by a hole of radius 2
//! about the vertical line through (10, 5), countersunk at the top by the cone about the same axis
//! from radius 2 at z = 8 to radius 4 at z = 10. The countersink is deleted: the wall runs on up
//! and the top closes in until they meet in the circle of radius 2 at z = 10.
use crate::bench::{frame, p};
use crate::shared::edit::{deleted, drilled_block};
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cone;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = drilled_block(m, [20.0, 10.0, 10.0], 10.0, 5.0, 2.0)?;
    let sink = Cone::new(2.0, 4.0, 2.0).with_frame(frame(p(10.0, 5.0, 8.0), Direction::Z)).build(m)?;
    let sunk = boolean::difference(m, block, sink)?;
    // The countersink halfway up, at radius 3.
    deleted(m, sunk, &[p(13.0, 5.0, 9.0)])
}
