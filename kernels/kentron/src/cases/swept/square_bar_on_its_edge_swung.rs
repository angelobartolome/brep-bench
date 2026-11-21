//! The solid a square bar fills as it is swung a quarter turn about an axis it faces with an
//! edge: the prism from z = 0 to z = 3 on the square with corners (4, 0), (6, 2), (8, 0) and
//! (6, −2), turned rigidly about the z axis by 90°.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) cuts what a prism passes through, moved in
//! the plane square to its edges, from a disc. Cut from the disc of radius 10 about the z axis,
//! as tall as the bar, it leaves the disc less the swept solid; the swept solid is what of the
//! disc that cut takes away.
use crate::bench::p;
use kentron::boolean;
use kentron::math::{Axis, Direction, Frame, Point2};
use kentron::primitives::Cylinder;
use kentron::sweep::{self, Blank, Motion, MotionStep, Tool};
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (radius, height) = (10.0, 3.0);
    let disc = Blank::Disc { frame: Frame::WORLD, radius, height };
    let outline = vec![Point2::new(4.0, 0.0), Point2::new(6.0, -2.0), Point2::new(8.0, 0.0), Point2::new(6.0, 2.0)];
    let bar = Tool::Prism { frame: Frame::WORLD, outline, from: 0.0, to: height };
    let turn = MotionStep { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), turn: 1.0, advance: 0.0 };
    let motion = Motion { steps: vec![turn], from: 0.0, to: FRAC_PI_2 };
    let rest = sweep::cut_by_motion(m, &disc, &bar, &motion)?;
    let whole = Cylinder::new(radius, height).build(m)?;
    boolean::difference(m, whole, rest)
}
