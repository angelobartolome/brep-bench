//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved all round by a
//! wheel with a sharp 60° V rim turned once round it: the solid of revolution, about the line
//! through (14, 0, 10) along (0, sin 20°, cos 20°), of the triangle with its tip at (4, 0, 10) and
//! its other corners at (6.5, 0, 10) ± 2.5 tan 30°·(0, sin 20°, cos 20°), turned rigidly a whole
//! turn about the z axis. Everything the wheel passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes the wheel as the solid of revolution
//! of its triangle, (10, 0) its tip and (7.5, ±2.5·tan 30°) its other corners in (distance from
//! the wheel's axis, height along it), the rod as the whole ring sector about the z axis from
//! radius 0 to 5, and the whole turn as one step: what a whole turn fills is turned into itself,
//! so the groove's section in the meridian half-plane is what the wheel covers there, revolved.
use crate::bench::p;
use kentron::math::{Axis, Direction, Frame, Point2};
use kentron::sweep::{self, Blank, MeridianPiece, Motion, MotionStep, Tool};
use kentron::topology::{Model, Shape};
use std::f64::consts::{PI, TAU};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Blank::RingSector { frame: Frame::WORLD, radii: [0.0, 5.0], heights: [0.0, 20.0], angles: [0.0, TAU] };
    let lean = 20f64.to_radians();
    let frame = Frame::new(p(14.0, 0.0, 10.0), Direction::new(0.0, lean.sin(), lean.cos()), Direction::NEG_X);
    let half = 2.5 * (PI / 6.0).tan();
    let (tip, near, far) = (Point2::new(10.0, 0.0), Point2::new(7.5, half), Point2::new(7.5, -half));
    let meridian = vec![
        MeridianPiece::Line { from: far, to: tip },
        MeridianPiece::Line { from: tip, to: near },
        MeridianPiece::Line { from: near, to: far },
    ];
    let wheel = Tool::Revolved { frame, meridian };
    let turn = MotionStep { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), turn: 1.0, advance: 0.0 };
    let motion = Motion { steps: vec![turn], from: 0.0, to: TAU };
    sweep::cut_by_motion(m, &rod, &wheel, &motion)
}
