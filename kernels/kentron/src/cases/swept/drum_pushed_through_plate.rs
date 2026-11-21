//! A plate, the box from (0, −5, 0) to (20, 5, 4), with a slot punched through it by a drum: the
//! solid cylinder of radius 2 about the line y = 0, z = −5, from x = 6 to 14, slid without turning
//! straight up by 16, from wholly below the plate to wholly above it. Everything the drum passes
//! through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes screws and turns, not a slide, so the
//! sweep is built as elementary geometry gives it: each section of the drum square to its axis is
//! a disc, which slid along z passes through itself at either end and the rectangle between, as
//! wide as the disc, its centre's path along the middle. So the drum passes through itself where
//! it starts and where it ends and the box 6 ≤ x ≤ 14, |y| ≤ 2 from its axis's first height to its
//! last.
use crate::bench::{frame, p};
use crate::shared::swept::cut_each;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(0.0, -5.0, 0.0), p(20.0, 5.0, 4.0)).build(m)?;
    let start = Cylinder::new(2.0, 8.0).with_frame(frame(p(6.0, 0.0, -5.0), Direction::X)).build(m)?;
    let end = Cylinder::new(2.0, 8.0).with_frame(frame(p(6.0, 0.0, 11.0), Direction::X)).build(m)?;
    let between = Cuboid::from_corners(p(6.0, -2.0, -5.0), p(14.0, 2.0, 11.0)).build(m)?;
    cut_each(m, plate, &[between, start, end])
}
