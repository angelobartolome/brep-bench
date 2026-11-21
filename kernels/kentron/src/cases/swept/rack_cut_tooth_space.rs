//! A gear blank with one tooth space cut by a rack tooth rolled round it (module 2, 20 teeth, 20°
//! pressure angle). The blank: the disc of radius 22 about the y axis from y = 0 to y = 6, pitch
//! circle 20. The tooth: the prism from y = −2 to y = 8 on the trapezoid with corners
//! (18, ±0.842856) and (24, ±3.026666) in (x, z). The rack's pitch line x = 20 rolls without
//! slipping on the pitch circle from 60° on one side of (20, 0, 0) to 60° on the other; cut from
//! the blank everything the tooth passes through.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes the tooth as a prism and the roll as
//! the blank sees it: the rack slid along the pitch line by 20φ, then turned by φ about the y
//! axis, for φ from −60° to 60° (the pitch line's point (20, 0, −20φ) carried to the pitch circle
//! at φ, where it stands still). Both are in the blank's plane and the tooth runs through all of
//! its height, so the cut is planar: the blank's disc less the region the trapezoid sweeps, its
//! flanks' envelopes (the involutes) and its tip corners' paths (the root), extruded through the
//! blank.
use crate::bench::p;
use kentron::math::{Axis, Direction, Frame, Point2};
use kentron::sweep::{self, Blank, Motion, MotionStep, Tool};
use kentron::topology::{Model, Shape};
use std::f64::consts::PI;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    // (The blank's and the tooth's frames: y their axis, x their first; in (x, z) the frames'
    // second coordinate is −z, which the symmetric trapezoid does not see.)
    let plane = Frame::new(p(0.0, 0.0, 0.0), Direction::Y, Direction::X);
    let blank = Blank::Disc { frame: plane, radius: 22.0, height: 6.0 };
    let corners = [[18.0, -0.842856], [24.0, -3.026666], [24.0, 3.026666], [18.0, 0.842856]];
    let tooth = Tool::Prism { frame: plane, outline: corners.iter().map(|&[x, y]| Point2::new(x, y)).collect(), from: -2.0, to: 8.0 };
    let slide = MotionStep { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), turn: 0.0, advance: 20.0 };
    let turn = MotionStep { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Y), turn: 1.0, advance: 0.0 };
    let motion = Motion { steps: vec![slide, turn], from: -PI / 3.0, to: PI / 3.0 };
    sweep::cut_by_motion(m, &blank, &tooth, &motion)
}
