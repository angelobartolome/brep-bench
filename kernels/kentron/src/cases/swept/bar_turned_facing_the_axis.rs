//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, cut by a leaning bar turned past it: the box from its inner
//! side, the plane through (10, 0, 0) square to u = (cos 20°, 0, −sin 20°), to 3 beyond it along
//! u, within 2 of the plane y = 0 and within 3 along its length ℓ = (sin 20°, 0, cos 20°) of
//! (10, 0, 0), turned rigidly about the z axis from −140° to +140°. Everything the bar passes
//! through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) turns only solids of revolution, so the cut
//! is built as elementary geometry gives it. Every point of the bar lies beyond its inner side,
//! x ≥ 10 + z tan 20°, so at least that far from the z axis; and the bar's section by the plane
//! y = 0, the rectangle (10, 0, 0) + a u + b ℓ, 0 ≤ a ≤ 3, |b| ≤ 3, holds every point of the
//! block's meridian rectangle that far out. The turn carries the bar clear across the block's
//! 60°, so over the block it passes through exactly that section revolved.
use crate::bench::{p, v};
use crate::shared::primitives::rectangle_xz;
use crate::shared::sweep::polyline;
use kentron::boolean;
use kentron::make;
use kentron::math::{Axis, Direction, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_3;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let z = Axis::new(p(0.0, 0.0, 0.0), Direction::Z);
    let section = rectangle_xz(m, (8.0, -1.0), (12.0, 1.0))?;
    let block = sweep::revolve(m, section, z, FRAC_PI_3)?.shape;
    let (sin, cos) = 20f64.to_radians().sin_cos();
    let (u, length) = (v(cos, 0.0, -sin), v(sin, 0.0, cos));
    let inner = p(10.0, 0.0, 0.0);
    let corners = [inner - length * 3.0, inner - length * 3.0 + u * 3.0, inner + length * 3.0 + u * 3.0, inner + length * 3.0];
    let outline = polyline(m, &corners, true)?;
    let face = make::planar_face(m, outline)?;
    let face = modify::transform(m, face, &Transform::rotation(z, (-140f64).to_radians()), TransformOptions::default())?;
    let cut = sweep::revolve(m, face, z, 280f64.to_radians())?.shape;
    boolean::difference(m, block, cut)
}
