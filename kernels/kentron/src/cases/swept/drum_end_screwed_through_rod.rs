//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by the flat end
//! of an end mill carried along a screw: the solid cylinder of radius 1.2 about the x axis from
//! x = 4 to 12, screwed about the z axis, right-handed, rising 6 a turn, from a turn of −90° to
//! +1350°. Everything the mill passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes the mill as the solid of revolution of
//! its meridian rectangle and the screw as one step: the mill, 2.4 across, is wholly below the rod
//! where the screw starts (lowered 1.5) and wholly above it where it ends.
use crate::bench::p;
use kentron::math::{Axis, Direction, Frame, Point2};
use kentron::sweep::{self, Blank, MeridianPiece, Motion, MotionStep, Tool};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Blank::Disc { frame: Frame::WORLD, radius: 5.0, height: 20.0 };
    // The meridian in (distance from the x axis, x): the rectangle from the axis out to 1.2,
    // from x = 4 to 12.
    let corners = [Point2::new(0.0, 4.0), Point2::new(1.2, 4.0), Point2::new(1.2, 12.0), Point2::new(0.0, 12.0)];
    let meridian = (0..4).map(|i| MeridianPiece::Line { from: corners[i], to: corners[(i + 1) % 4] }).collect();
    let mill = Tool::Revolved { frame: Frame::new(p(0.0, 0.0, 0.0), Direction::X, Direction::Z), meridian };
    // The screw, t its turn in radians: rising 6 a turn.
    let screw = MotionStep { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), turn: 1.0, advance: 6.0 / TAU };
    let motion = Motion { steps: vec![screw], from: (-90f64).to_radians(), to: 1350f64.to_radians() };
    sweep::cut_by_motion(m, &rod, &mill, &motion)
}
