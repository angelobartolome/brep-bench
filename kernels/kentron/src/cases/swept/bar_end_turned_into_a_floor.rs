//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, its floor cut by the flat end of a leaning bar turned past it:
//! in the plane through (10, 0, 0) spanned by u = (cos 20°, 0, −sin 20°) and v = (0, 1, 0), the
//! 3 × 4 rectangle 0 ≤ a cos 5° + b sin 5° ≤ 3, |b cos 5° − a sin 5°| ≤ 2 of points (a, b), carried 3
//! along n = (sin 20°, 0, cos 20°), turned rigidly about the z axis from −140° to +140°.
//! Everything the bar passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) turns only solids of revolution, so the cut
//! is built as elementary geometry gives it, as a box slid is. A point the bar covers at some
//! instant was inside it where the turn started (clear of the block) or was carried in through
//! the part of a face leading the turn, where the velocity ẑ × r points out of the bar: on a flat
//! face of normal n, where r·(n × ẑ) > 0, on one side of the plane through the z axis square to
//! n × ẑ. Each face's leading part is cut off by that plane (`leading_part`) and revolved from
//! −140° to 140°: inside it the velocity never runs along the face, so it never folds.
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
    let (s5, c5) = 5f64.to_radians().sin_cos();
    let (u, w, n) = (v(c20, 0.0, -s20), v(0.0, 1.0, 0.0), v(s20, 0.0, c20));
    // The rectangle's sides' directions, 5° round from u and v; its corner where a cos 5° + b sin 5°
    // is 0 and b cos 5° − a sin 5° is −2.
    let (e1, e2) = (u * c5 + w * s5, w * c5 - u * s5);
    let o = p(10.0, 0.0, 0.0) - e2 * 2.0;
    let corner = |i: f64, j: f64, k: f64| -> Point { o + e1 * (3.0 * i) + e2 * (4.0 * j) + n * (3.0 * k) };
    // Each face by its corners and its outward normal.
    let faces: [([Point; 4], Vector); 6] = [
        ([corner(0., 0., 0.), corner(1., 0., 0.), corner(1., 1., 0.), corner(0., 1., 0.)], -n),
        ([corner(0., 0., 1.), corner(1., 0., 1.), corner(1., 1., 1.), corner(0., 1., 1.)], n),
        ([corner(0., 0., 0.), corner(0., 1., 0.), corner(0., 1., 1.), corner(0., 0., 1.)], -e1),
        ([corner(1., 0., 0.), corner(1., 1., 0.), corner(1., 1., 1.), corner(1., 0., 1.)], e1),
        ([corner(0., 0., 0.), corner(1., 0., 0.), corner(1., 0., 1.), corner(0., 0., 1.)], -e2),
        ([corner(0., 1., 0.), corner(1., 1., 0.), corner(1., 1., 1.), corner(0., 1., 1.)], e2),
    ];
    let start = Transform::rotation(z, (-140f64).to_radians());
    let mut pieces = Vec::new();
    for (corners, normal) in faces {
        let outline = polyline(m, &corners, true)?;
        let face = make::planar_face(m, outline)?;
        if let Some(part) = leading_part(m, face, normal, z)? {
            let part = modify::transform(m, part, &start, TransformOptions::default())?;
            pieces.push(sweep::revolve(m, part, z, 280f64.to_radians())?.shape);
        }
    }
    cut_each(m, block, &pieces)
}
