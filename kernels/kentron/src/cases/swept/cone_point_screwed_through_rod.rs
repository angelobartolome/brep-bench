//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by a pointed
//! cone carried along a screw: the cone about the x axis with its point at (4, 0, 0), opening away
//! from the rod at 60° to a flat base at x = 12, screwed about the z axis, right-handed, rising 6
//! a turn, from a turn of −90° to +1350°. Everything the cone passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes the cone as the solid of revolution
//! of its meridian triangle and the screw as one step. It takes a tool wholly beyond the rod's
//! ends where the motion starts and ends, and the cone's base, 4.6 across, is not: but every
//! point of the cone with x > 5 lies more than 5 from the z axis wherever the screw carries it,
//! so never enters the rod. The cone is cut off at x = 6, its base there 2 tan 30° < 1.5 across,
//! and the screw's lowering and raising carry that clear.
use crate::bench::p;
use kentron::math::{Axis, Direction, Frame, Point2};
use kentron::sweep::{self, Blank, MeridianPiece, Motion, MotionStep, Tool};
use kentron::topology::{Model, Shape};
use std::f64::consts::{PI, TAU};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Blank::Disc { frame: Frame::WORLD, radius: 5.0, height: 20.0 };
    // The meridian in (distance from the x axis, x): the point, the base's rim at x = 6, its
    // centre.
    let (point, rim, centre) = (Point2::new(0.0, 4.0), Point2::new(2.0 * (PI / 6.0).tan(), 6.0), Point2::new(0.0, 6.0));
    let meridian = vec![
        MeridianPiece::Line { from: point, to: rim },
        MeridianPiece::Line { from: rim, to: centre },
        MeridianPiece::Line { from: centre, to: point },
    ];
    let cone = Tool::Revolved { frame: Frame::new(p(0.0, 0.0, 0.0), Direction::X, Direction::Z), meridian };
    // The screw, t its turn in radians: rising 6 a turn.
    let screw = MotionStep { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), turn: 1.0, advance: 6.0 / TAU };
    let motion = Motion { steps: vec![screw], from: (-90f64).to_radians(), to: 1350f64.to_radians() };
    sweep::cut_by_motion(m, &rod, &cone, &motion)
}
