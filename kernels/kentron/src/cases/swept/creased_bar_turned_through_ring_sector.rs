//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, grooved by a bar drafted both ways from a parting plane, turned
//! past it: with u = (1, 0, 0), v = (0, cos 20°, −sin 20°) and n = (0, sin 20°, cos 20°), every
//! a u + b v + h n with |h| ≤ 4, 10 + |h| tan 2° ≤ a ≤ 13 − |h| tan 2° and |b| ≤ 2 − |h| tan 2°,
//! turned rigidly about the z axis from −140° to +140°. Everything the bar passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) turns only solids of revolution, so the cut
//! is built as elementary geometry gives it, as a box slid is. A point the bar covers at some
//! instant was inside it where the turn started (clear of the block) or was carried in through
//! the part of a face leading the turn, where the velocity ẑ × r points out of the bar: on a flat
//! face of normal n_f, where r·(n_f × ẑ) > 0, on one side of the plane through the z axis square
//! to n_f × ẑ. The bar is two drafted halves meeting in the parting rectangle, ten flat faces; each
//! face's leading part is cut off by that plane (`leading_part`) and revolved from −140° to 140°:
//! inside it the velocity never runs along the face, so it never folds.
use crate::bench::{p, v};
use crate::shared::primitives::rectangle_xz;
use crate::shared::sweep::polyline;
use crate::shared::swept::{cut_each, leading_part};
use kentron::make;
use kentron::math::{Axis, Direction, Point, Transform, Vector};
use kentron::modify::{self, TransformOptions};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_3;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let z = Axis::new(p(0.0, 0.0, 0.0), Direction::Z);
    let section = rectangle_xz(m, (8.0, -1.0), (12.0, 1.0))?;
    let block = sweep::revolve(m, section, z, FRAC_PI_3)?.shape;
    let (s20, c20) = 20f64.to_radians().sin_cos();
    let (u, w, n) = (v(1.0, 0.0, 0.0), v(0.0, c20, -s20), v(0.0, s20, c20));
    let draft = 2f64.to_radians().tan();
    let at = |a: f64, b: f64, h: f64| -> Point { Point::ORIGIN + u * a + w * b + n * h };
    // The rectangle's corners at height h, counter-clockwise about n.
    let ring = |h: f64| {
        let d = h.abs() * draft;
        [at(10.0 + d, -2.0 + d, h), at(13.0 - d, -2.0 + d, h), at(13.0 - d, 2.0 - d, h), at(10.0 + d, 2.0 - d, h)]
    };
    let (middle, centre) = (ring(0.0), at(11.5, 0.0, 0.0));
    let mut faces: Vec<Vec<Point>> = Vec::new();
    for h in [4.0, -4.0] {
        let end = ring(h);
        faces.push(end.to_vec());
        for i in 0..4 {
            faces.push(vec![middle[i], middle[(i + 1) % 4], end[(i + 1) % 4], end[i]]);
        }
    }
    let start = Transform::rotation(z, (-140f64).to_radians());
    let mut pieces = Vec::new();
    for corners in faces {
        // The face's normal, turned out of the bar (away from its centre).
        let normal = (corners[1] - corners[0]).cross(corners[2] - corners[0]);
        let normal: Vector = if normal.dot(corners[0] - centre) < 0.0 { -normal } else { normal };
        let outline = polyline(m, &corners, true)?;
        let face = make::planar_face(m, outline)?;
        if let Some(part) = leading_part(m, face, normal, z)? {
            let part = modify::transform(m, part, &start, TransformOptions::default())?;
            pieces.push(sweep::revolve(m, part, z, 280f64.to_radians())?.shape);
        }
    }
    cut_each(m, block, &pieces)
}
