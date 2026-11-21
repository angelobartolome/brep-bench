//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove
//! ground by a wheel carried along a screw. The wheel: a torus, radii 6 and 2, centred at
//! (11.5, 0, 0), its axis (0, cos h, −sin h) with h = atan(2π·5.5 / 30). The screw: about the z
//! axis, right-handed, rising 30 a turn, from a turn of −108° to +348°. Everything the wheel
//! passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes the wheel as the solid of revolution
//! of its tube's circle and the screw as one step. The wheel starts wholly below the rod and ends
//! wholly above it, so within the rod what it fills is screwed into itself: the groove's section
//! square to the axis is the plane region the wheel's characteristic curve (where the screw runs
//! along it) projects to along the helices, the whole of the tube's circle on the rod's side
//! among it (set along the helix there, the wheel's surface runs along the screw all round it).
//! The rod's section less that is screwed up the rod.
use crate::bench::p;
use kentron::math::{Axis, Direction, Frame, Point2};
use kentron::sweep::{self, Blank, MeridianPiece, Motion, MotionStep, Tool};
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Blank::Disc { frame: Frame::WORLD, radius: 5.0, height: 20.0 };
    let h = (TAU * 5.5 / 30.0).atan();
    let frame = Frame::new(p(11.5, 0.0, 0.0), Direction::new(0.0, h.cos(), -h.sin()), Direction::X);
    let wheel = Tool::Revolved { frame, meridian: vec![MeridianPiece::Arc { centre: Point2::new(6.0, 0.0), radius: 2.0, start: 0.0, end: TAU }] };
    // The screw, t its turn in radians: rising 30 a turn.
    let screw = MotionStep { axis: Axis::new(p(0.0, 0.0, 0.0), Direction::Z), turn: 1.0, advance: 30.0 / TAU };
    let motion = Motion { steps: vec![screw], from: (-108f64).to_radians(), to: 348f64.to_radians() };
    sweep::cut_by_motion(m, &rod, &wheel, &motion)
}
