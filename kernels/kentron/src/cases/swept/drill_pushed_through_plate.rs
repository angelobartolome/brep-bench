//! A plate, the box from (0, −5, 0) to (20, 5, 4), with a hole bored through it by a drill about
//! the vertical line through (10, 0): a cylinder of radius 2 from z = 6 + 2/tan 59° up to 14, ended
//! below in a cone of 118° whose point is (10, 0, 6), slid without turning straight down its axis
//! by 20. Everything the drill passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes screws and turns, not a slide, so the
//! sweep is built as elementary geometry gives it: each circle of the drill about its axis slides
//! along the axis through the cylinder of its radius, so the drill passes through itself where it
//! ends and its cylinder from there up to where it starts.
use crate::bench::{frame, p};
use crate::shared::swept::cut_each;
use kentron::math::Direction;
use kentron::primitives::{Cone, Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(0.0, -5.0, 0.0), p(20.0, 5.0, 4.0)).build(m)?;
    let point_length = 2.0 / 59f64.to_radians().tan();
    // The drill's point where it ends, 20 below where it starts.
    let point = Cone::new(0.0, 2.0, point_length).with_frame(frame(p(10.0, 0.0, -14.0), Direction::Z)).build(m)?;
    // Its cylinder, from its foot where the drill ends up to its top where the drill starts.
    let foot = -14.0 + point_length;
    let body = Cylinder::new(2.0, 14.0 - foot).with_frame(frame(p(10.0, 0.0, foot), Direction::Z)).build(m)?;
    cut_each(m, plate, &[body, point])
}
