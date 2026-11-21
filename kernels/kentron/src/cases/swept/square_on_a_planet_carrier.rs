//! The solid a square bar fills as it is carried round on a planet: the prism from z = 0 to z = 2
//! on the square of side 2 centred at (5, 0), its sides along x and y, its centre carried round
//! the circle of radius 5 about the z axis to (5 cos t, 5 sin t) while it turns about its centre
//! by −2t, for t from 0° to 360°.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) cuts what a prism passes through, moved in
//! the plane square to its edges, from a disc. The motion is the bar's turn about its own centre
//! against the carrier, by −3t, then the carrier's turn by t about the z axis. Cut from the disc
//! of radius 8 about the z axis, as tall as the bar, it leaves the disc less the swept solid; the
//! swept solid is what of the disc that cut takes away.
use crate::bench::p;
use kentron::boolean;
use kentron::math::{Axis, Direction, Frame, Point2};
use kentron::primitives::Cylinder;
use kentron::sweep::{self, Blank, Motion, MotionStep, Tool};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (radius, height) = (8.0, 2.0);
    let disc = Blank::Disc { frame: Frame::WORLD, radius, height };
    let outline = [(4.0, -1.0), (6.0, -1.0), (6.0, 1.0), (4.0, 1.0)].iter().map(|&(x, y)| Point2::new(x, y)).collect();
    let bar = Tool::Prism { frame: Frame::WORLD, outline, from: 0.0, to: height };
    let spin = MotionStep { axis: Axis::new(p(5.0, 0.0, 0.0), Direction::Z), turn: -3.0, advance: 0.0 };
    let carrier = MotionStep { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), turn: 1.0, advance: 0.0 };
    let motion = Motion { steps: vec![spin, carrier], from: 0.0, to: TAU };
    let rest = sweep::cut_by_motion(m, &disc, &bar, &motion)?;
    let whole = Cylinder::new(radius, height).build(m)?;
    boolean::difference(m, whole, rest)
}
