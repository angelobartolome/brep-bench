//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, with a helical groove milled through it by a tapered cutter:
//! the frustum about the vertical line through (10, 0), of radius 0.4 at z = −0.6 widening to 1 at
//! z = 1.4, carried along the screw about the z axis, right-handed, rising 2 a turn, from a turn
//! of −140° to +140°. Everything the cutter passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) screws a tool only through a rod, so the
//! sweep is built as elementary geometry gives it, as a box slid is: the cutter where it starts
//! and where it ends (both clear of the block), and its faces screwed (`sweep::screw_extrude`). A
//! point the cutter covers at some instant that no face ever crosses stays inside it all the way,
//! so is in it at both ends. At angle u about the cutter's axis its side's normal is
//! (cos u, sin u, −0.3)/√1.09 and the screw's velocity (ẑ × r) + k ẑ, k = 2/2π, so they are square
//! where 10 sin u = 0.3 k: on the two lines of the cone at u₀ = asin(0.03 k) and π − u₀. Split
//! there, each part screws without folding. The flat ends add nothing: a point is inside the
//! cutter over an interval of the screw's turn no longer than the cutter's angular width (under
//! 12°), while its ends take a whole turn to rise past it, so it enters or leaves through the side.
use crate::bench::p;
use crate::shared::primitives::rectangle_xz;
use crate::shared::swept::cut_each;
use kentron::geometry::surface::{Cone as ConeSurface, Surface};
use kentron::make;
use kentron::math::{Axis, Direction, Frame, Transform, Vector};
use kentron::modify::{self, TransformOptions};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::{FRAC_PI_3, PI, TAU};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let z = Axis::new(p(0.0, 0.0, 0.0), Direction::Z);
    let section = rectangle_xz(m, (8.0, -1.0), (12.0, 1.0))?;
    let block = sweep::revolve(m, section, z, FRAC_PI_3)?.shape;
    // The screw: rising 2 a turn, through 280° from a turn of −140°.
    let (lead, from, to) = (2.0, (-140f64).to_radians(), 140f64.to_radians());
    let start = Transform::translation(Vector::new(0.0, 0.0, lead * from / TAU)) * Transform::rotation(z, from);
    // The cutter's side as placed, its angle u about its axis from +x, its v along its lines
    // (2/cos a long), split at its two lines of contact.
    let a = 0.3f64.atan();
    let side = Surface::Cone(ConeSurface::new(Frame::new(p(10.0, 0.0, -0.6), Direction::Z, Direction::X), a, 0.4));
    let u0 = (0.03 * lead / TAU).asin();
    let mut pieces = Vec::new();
    for (u1, u2) in [(u0, PI - u0), (PI - u0, TAU + u0)] {
        let part = make::face_on_domain(m, side.clone(), u1, u2, 0.0, 2.0 / a.cos())?;
        let part = modify::transform(m, part, &start, TransformOptions::default())?;
        pieces.push(sweep::screw_extrude(m, part, z, lead, lead * (to - from) / TAU)?.shape);
    }
    cut_each(m, block, &pieces)
}
