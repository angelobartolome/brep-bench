//! A block, the box from (−10, −10, −5) to (10, 10, 0), pocketed by an end mill turned about a
//! line through its own flat end: the solid cylinder of radius 3 about the vertical line through
//! (1.5, 0), from z = −2 to 4, turned rigidly about the z axis by 120°. Everything the mill passes
//! through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes a turn that carries its tool clear
//! past a ring sector, and this one turns about a line through the mill, so the sweep is built as
//! elementary geometry gives it: at each height the mill is a disc about (1.5, 0), which meets each
//! circle about the z axis (out to 4.5) in an arc centred on polar angle 0 (the whole circle,
//! within 1.5); turned, that arc runs from where it starts to where it ends. So the mill passes
//! through itself where it starts and where it ends and, between, the sector of the cylinder of
//! radius 4.5 about the z axis from polar angle 0 to 120°.
use crate::bench::{frame, p};
use crate::shared::swept::cut_each;
use kentron::math::{Axis, Direction, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(-10.0, -10.0, -5.0), p(10.0, 10.0, 0.0)).build(m)?;
    let start = Cylinder::new(3.0, 6.0).with_frame(frame(p(1.5, 0.0, -2.0), Direction::Z)).build(m)?;
    let turn = Transform::rotation(Axis::new(p(0.0, 0.0, 0.0), Direction::Z), 120f64.to_radians());
    let end = modify::transform(m, start, &turn, TransformOptions::default())?;
    let fan = Cylinder::new(4.5, 6.0).with_angle(120f64.to_radians()).with_frame(frame(p(0.0, 0.0, -2.0), Direction::Z)).build(m)?;
    cut_each(m, block, &[fan, start, end])
}
