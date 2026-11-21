//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, with a helical groove milled through it by a drum: the solid
//! cylinder of radius 1 about the vertical line through (10, 0), |z| ≤ 0.5, carried along the
//! screw about the z axis, right-handed, rising 0.5 a turn, from a turn of −140° to +140°.
//! Everything the drum passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) screws a tool only through a rod, so the
//! sweep is built as elementary geometry gives it, as a box slid is: the drum where it starts and
//! where it ends (both clear of the block), and its faces screwed (`sweep::screw_extrude`). A point
//! the drum covers at some instant that no face ever crosses stays inside it all the way, so is in
//! it at both ends. The side's horizontal normal, out from the drum's axis, is square to the
//! screw's velocity ẑ × r + (0.5/2π) ẑ only where it points along r, on the side's lines nearest
//! and farthest from the z axis, so the side is screwed in its halves either side of y = 0, each
//! then sweeping without folding. The flat ends add nothing: a point is inside the drum over an
//! interval of the screw's turn no longer than the drum's angular width (under 12°), while its
//! ends take a whole 720° to rise past it, so it enters or leaves through the side.
use crate::bench::p;
use crate::shared::primitives::rectangle_xz;
use crate::shared::swept::cut_each;
use kentron::geometry::surface::{Cylinder as CylinderSurface, Surface};
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
    // The screw: rising 0.5 a turn, through 280° from a turn of −140°.
    let (lead, from, to) = (0.5, (-140f64).to_radians(), 140f64.to_radians());
    let start = Transform::translation(Vector::new(0.0, 0.0, lead * from / TAU)) * Transform::rotation(z, from);
    // The drum's side as placed, its angle u about its axis from +x: the half from u = 0 to π,
    // where y ≥ 0, and the other.
    let side = Surface::Cylinder(CylinderSurface::new(Frame::new(p(10.0, 0.0, -0.5), Direction::Z, Direction::X), 1.0));
    let mut pieces = Vec::new();
    for (u1, u2) in [(0.0, PI), (PI, TAU)] {
        let half = make::face_on_domain(m, side.clone(), u1, u2, 0.0, 1.0)?;
        let half = modify::transform(m, half, &start, TransformOptions::default())?;
        pieces.push(sweep::screw_extrude(m, half, z, lead, lead * (to - from) / TAU)?.shape);
    }
    cut_each(m, block, &pieces)
}
