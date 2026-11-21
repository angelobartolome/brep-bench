//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical V groove
//! ground by a wheel with a sharp 60° V rim carried along a screw. The wheel: the solid of
//! revolution, about the axis through (14, 0, 0) along (0, cos h, −sin h) with
//! h = atan(2π·4.5 / 30), of the triangle with its tip at (4, 0, 0) and its other corners 2.5
//! further out, ±2.5·tan 30° along the axis. The screw: about the z axis, right-handed, rising 30
//! a turn, from a turn of −144° to +384°. Everything the wheel passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes the wheel as the solid of revolution
//! of its triangle, (10, 0) its tip and (7.5, ±2.5·tan 30°) its other corners in (distance from
//! the wheel's axis, height along it), and the screw as one step. The wheel starts wholly below the
//! rod and ends wholly above it, so within the rod what it fills is screwed into itself: the
//! groove's section square to the axis is the plane region bounded where the flanks'
//! characteristic curves (where the screw runs along the cones) and the sharp rim project along
//! the helices. The rod's section less that is screwed up the rod.
use crate::bench::p;
use kentron::math::{Axis, Direction, Frame, Point2};
use kentron::sweep::{self, Blank, MeridianPiece, Motion, MotionStep, Tool};
use kentron::topology::{Model, Shape};
use std::f64::consts::{PI, TAU};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Blank::Disc { frame: Frame::WORLD, radius: 5.0, height: 20.0 };
    let h = (TAU * 4.5 / 30.0).atan();
    let frame = Frame::new(p(14.0, 0.0, 0.0), Direction::new(0.0, h.cos(), -h.sin()), Direction::X);
    let half = 2.5 * (PI / 6.0).tan();
    let (tip, near, far) = (Point2::new(10.0, 0.0), Point2::new(7.5, half), Point2::new(7.5, -half));
    let meridian = vec![
        MeridianPiece::Line { from: far, to: tip },
        MeridianPiece::Line { from: tip, to: near },
        MeridianPiece::Line { from: near, to: far },
    ];
    let wheel = Tool::Revolved { frame, meridian };
    // The screw, t its turn in radians: rising 30 a turn.
    let screw = MotionStep { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), turn: 1.0, advance: 30.0 / TAU };
    let motion = Motion { steps: vec![screw], from: (-144f64).to_radians(), to: 384f64.to_radians() };
    sweep::cut_by_motion(m, &rod, &wheel, &motion)
}
