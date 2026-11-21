//! A block, the box from (0, −5, 0) to (20, 5, 10), with a slot cut across its top by an end mill:
//! the solid cylinder of radius 2 about the vertical line through (−5, 0), from z = 7 up to 15,
//! slid without turning along the x axis by 30, from clear before the block to clear past it.
//! Everything the mill passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes a slide only of a prism through a
//! disc, so the sweep is built as elementary geometry gives it: each level of the mill is a disc,
//! which slid along x passes through itself at either end and the rectangle between, as wide as
//! the disc. So the mill passes through itself where it starts and where it ends and the box
//! −5 ≤ x ≤ 25, |y| ≤ 2, 7 ≤ z ≤ 15.
use crate::bench::{frame, p};
use crate::shared::swept::cut_each;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(0.0, -5.0, 0.0), p(20.0, 5.0, 10.0)).build(m)?;
    let start = Cylinder::new(2.0, 8.0).with_frame(frame(p(-5.0, 0.0, 7.0), Direction::Z)).build(m)?;
    let end = Cylinder::new(2.0, 8.0).with_frame(frame(p(25.0, 0.0, 7.0), Direction::Z)).build(m)?;
    let between = Cuboid::from_corners(p(-5.0, -2.0, 7.0), p(25.0, 2.0, 15.0)).build(m)?;
    cut_each(m, block, &[between, start, end])
}
