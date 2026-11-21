//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −2 to 2
//! and polar angle from −30° to 30°, with a helical groove milled into it by a ball-end mill: about
//! the vertical line through (10, 0), the lower half of the ball of radius 1 about (10, 0, 0) under
//! a cylinder of radius 1 up to z = 3, carried along the screw about the z axis, right-handed,
//! rising 3 a turn, from a turn of −60° to +60°. Everything the mill passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) screws a tool only through a rod, so the
//! sweep is built as elementary geometry gives it. The mill is the ball and the cylinder above its
//! centre, and passes through what each does. The ball passes through the tube its great disc
//! square to its centre's helix sweeps, kept square to it (screwed), with the ball where it starts
//! and where it ends. The cylinder passes through itself where it starts and where it ends and
//! what its faces pass through: its bottom is the ball's equator, inside the ball's sweep, and its
//! top stays above the block; its side's horizontal normal is square to the screw's velocity only
//! where it points along r, on the side's lines nearest and farthest from the z axis, so the side
//! is screwed in its halves either side of y = 0, each sweeping without folding. The mill starts
//! and ends clear of the block.
use crate::bench::p;
use crate::shared::primitives::rectangle_xz;
use crate::shared::sweep::disc;
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
    let section = rectangle_xz(m, (8.0, -2.0), (12.0, 2.0))?;
    let section = modify::transform(m, section, &Transform::rotation(z, -FRAC_PI_3 / 2.0), TransformOptions::default())?;
    let block = sweep::revolve(m, section, z, FRAC_PI_3)?.shape;
    // The screw: rising 3 a turn, through 120° from a turn of −60°.
    let (lead, from, to) = (3.0, (-60f64).to_radians(), 60f64.to_radians());
    let rise = lead * (to - from) / TAU;
    let start = Transform::translation(Vector::new(0.0, 0.0, lead * from / TAU)) * Transform::rotation(z, from);
    // The ball's great disc square to its centre's helix, which leaves (10, 0, 0) along
    // (0, 10, lead/2π), screwed.
    let great = disc(m, p(10.0, 0.0, 0.0), Direction::new(0.0, 10.0, lead / TAU), 1.0)?;
    let great = modify::transform(m, great, &start, TransformOptions::default())?;
    let mut pieces = vec![sweep::screw_extrude(m, great, z, lead, rise)?.shape];
    // The cylinder's side as placed, its angle u about its axis from +x: the half from u = 0 to
    // π, where y ≥ 0, and the other, each screwed.
    let side = Surface::Cylinder(CylinderSurface::new(Frame::new(p(10.0, 0.0, 0.0), Direction::Z, Direction::X), 1.0));
    for (u1, u2) in [(0.0, PI), (PI, TAU)] {
        let half = make::face_on_domain(m, side.clone(), u1, u2, 0.0, 3.0)?;
        let half = modify::transform(m, half, &start, TransformOptions::default())?;
        pieces.push(sweep::screw_extrude(m, half, z, lead, rise)?.shape);
    }
    cut_each(m, block, &pieces)
}
