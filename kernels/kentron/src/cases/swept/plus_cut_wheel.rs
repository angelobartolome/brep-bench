//! A wheel cut by a plus-shaped tool turning in mesh with it, as a gear shaper cuts. The tool: a
//! plus centred on the origin, arms 30 long and 8 wide along ±x and ±y, extruded from z = −10 to
//! z = 0. The blank: the disc of radius 40 about the vertical line through (−50, 0), from z = −8
//! to z = −2. As the tool turns by t about the z axis the wheel turns by −2t/3 about its own axis,
//! for t from 0° to 540°; cut from the wheel everything the tool passes through, seen from it.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes the plus as a prism and the motion
//! as the wheel sees it: the plus's own turn by t about the z axis, then the wheel's taken back,
//! a turn by 2t/3 about its axis. Both turns are square to the wheel, and the plus runs through
//! all of its height, so the cut is planar: the wheel's disc less the region the plus's outline
//! sweeps (bounded by its convex corners' paths and its sides' envelopes, each kept where no pose
//! of the plus covers it), extruded through the wheel.
use crate::bench::p;
use kentron::math::{Axis, Direction, Frame, Point2};
use kentron::sweep::{self, Blank, Motion, MotionStep, Tool};
use kentron::topology::{Model, Shape};
use std::f64::consts::PI;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let wheel = Blank::Disc { frame: Frame::new(p(-50.0, 0.0, -8.0), Direction::Z, Direction::X), radius: 40.0, height: 6.0 };
    let (a, w) = (30.0, 4.0);
    let corners = [[a, -w], [a, w], [w, w], [w, a], [-w, a], [-w, w], [-a, w], [-a, -w], [-w, -w], [-w, -a], [w, -a], [w, -w]];
    let plus = Tool::Prism { frame: Frame::WORLD, outline: corners.iter().map(|&[x, y]| Point2::new(x, y)).collect(), from: -10.0, to: 0.0 };
    // The plus's turn about the z axis, then the wheel's (−2t/3 about its axis) taken back.
    let tool_turn = MotionStep { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), turn: 1.0, advance: 0.0 };
    let wheel_turn_back = MotionStep { axis: Axis::new(p(-50.0, 0.0, 0.0), Direction::Z), turn: 2.0 / 3.0, advance: 0.0 };
    let motion = Motion { steps: vec![tool_turn, wheel_turn_back], from: 0.0, to: 3.0 * PI };
    sweep::cut_by_motion(m, &wheel, &plus, &motion)
}
