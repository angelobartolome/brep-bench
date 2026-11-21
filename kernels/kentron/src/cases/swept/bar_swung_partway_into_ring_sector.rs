//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, notched by a bar swung partway into it: the box 10 ≤ x ≤ 11,
//! |y| ≤ 0.5, |z| ≤ 0.6, turned rigidly about the z axis from −140° to +30°. Everything the bar
//! passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) turns only solids of revolution, and only
//! clear past the blank, so the sweep is built as elementary geometry gives it, as a box slid is: the
//! bar where it starts (clear of the block) and where it ends, and each of its faces turned. A
//! point the bar covers at some instant that no face ever crosses stays inside it all the way, so
//! is in it at both ends. A face turned passes through exactly the solid its revolve makes where
//! no two of its points lie on one circle about the z axis: so the upright faces x = 10 and 11 are
//! taken in halves either side of y = 0, and the level faces z = ±0.6 turn within their own planes,
//! filling nothing.
use crate::bench::p;
use crate::shared::primitives::rectangle_xz;
use crate::shared::sweep::polyline;
use crate::shared::swept::cut_each;
use kentron::make;
use kentron::math::{Axis, Direction, Point, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::Cuboid;
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_3;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let z = Axis::new(p(0.0, 0.0, 0.0), Direction::Z);
    let section = rectangle_xz(m, (8.0, -1.0), (12.0, 1.0))?;
    let block = sweep::revolve(m, section, z, FRAC_PI_3)?.shape;
    let bar = Cuboid::from_corners(p(10.0, -0.5, -0.6), p(11.0, 0.5, 0.6)).build(m)?;
    let end = modify::transform(m, bar, &Transform::rotation(z, 30f64.to_radians()), TransformOptions::default())?;
    // The upright faces as placed, each from its foot (x0, y0) to (x1, y1): the sides y = ±0.5
    // whole, the ends x = 10 and 11 in halves.
    let uprights = [
        ((10.0, 0.5), (11.0, 0.5)),
        ((10.0, -0.5), (11.0, -0.5)),
        ((10.0, -0.5), (10.0, 0.0)),
        ((10.0, 0.0), (10.0, 0.5)),
        ((11.0, -0.5), (11.0, 0.0)),
        ((11.0, 0.0), (11.0, 0.5)),
    ];
    let start = Transform::rotation(z, (-140f64).to_radians());
    let mut pieces = Vec::new();
    for ((x0, y0), (x1, y1)) in uprights {
        let corners: [Point; 4] = [p(x0, y0, -0.6), p(x1, y1, -0.6), p(x1, y1, 0.6), p(x0, y0, 0.6)];
        let outline = polyline(m, &corners, true)?;
        let face = make::planar_face(m, outline)?;
        let face = modify::transform(m, face, &start, TransformOptions::default())?;
        pieces.push(sweep::revolve(m, face, z, 170f64.to_radians())?.shape);
    }
    pieces.push(end);
    cut_each(m, block, &pieces)
}
