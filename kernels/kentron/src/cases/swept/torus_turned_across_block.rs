//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to
//! 1 and polar angle from 0° to 60°, grooved by a leaning cutter turned past it: the torus centred
//! at (5, 0, 0) with radii 2.5 and 2, its axis (0, sin 25°, cos 25°), turned rigidly about the z
//! axis from −140° to +140°. Everything the cutter passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes the cutter as the solid of revolution
//! of its tube's circle and the turn as one step. The cutter starts and ends clear of the block,
//! so over the block what it fills is turned into itself: the groove's section in the meridian
//! half-plane (distance from the z axis, height) is the region the cutter's characteristic curve
//! (where the turn runs along it, none of its circles, its axis being skew to the turn's)
//! projects to along the circles about the z axis. The block's meridian rectangle less that is
//! revolved through the block's 60°.
use crate::bench::p;
use kentron::math::{Axis, Direction, Frame, Point2};
use kentron::sweep::{self, Blank, MeridianPiece, Motion, MotionStep, Tool};
use kentron::topology::{Model, Shape};
use std::f64::consts::{PI, TAU};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Blank::RingSector { frame: Frame::WORLD, radii: [8.0, 12.0], heights: [-1.0, 1.0], angles: [0.0, PI / 3.0] };
    let lean = 25f64.to_radians();
    let frame = Frame::new(p(5.0, 0.0, 0.0), Direction::new(0.0, lean.sin(), lean.cos()), Direction::X);
    let cutter = Tool::Revolved { frame, meridian: vec![MeridianPiece::Arc { centre: Point2::new(2.5, 0.0), radius: 2.0, start: 0.0, end: TAU }] };
    let turn = MotionStep { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), turn: 1.0, advance: 0.0 };
    let motion = Motion { steps: vec![turn], from: (-140f64).to_radians(), to: 140f64.to_radians() };
    sweep::cut_by_motion(m, &block, &cutter, &motion)
}
