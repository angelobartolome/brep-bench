//! The solid a cone fills as it is spun about a line through it: the cone about the line through
//! the origin along w = (cos 30°, 0, sin 30°), its point at −2w, opening at 25° from its axis to a
//! flat base at 5w of radius 7 tan 25°, turned rigidly about the z axis by 150°.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) cuts a blank, and turns a tool clear past
//! it, so the solid is built as elementary geometry gives it. A circle about the z axis lies on a
//! sphere about the origin, which is on the cone's axis; at angle φ from +x on the circle, p·w is
//! a cos φ + b, and whether p is in the cone depends on p·w alone. Fill the cone out to the ball
//! through its rim, of radius R = √(25 + 49 tan² 25°): K′, its base replaced by that ball's cap.
//! K′ meets each sphere about the origin in a cap about w and, nearer than the point, one about −w;
//! so it meets each circle in an arc about φ = 0, one about φ = 180°, or both, or all of it. Where
//! the sphere passes the base the cone leaves a gap about φ = 0 that K′ fills, but no wider than
//! 150° (cos φ ≥ (5 − z/2)/(0.866 ρ) ≥ 0.26 there, as ρ, |z| ≤ R), so the turn carries the cone
//! over it and K′ passes through no more than the cone. An arc about 0° turned through 150° covers
//! what K′ covers where it starts and where it ends and the angles from 0° to 150° between; one
//! about 180°, from 180° to 330°. So the solid is K′ where it starts and where it ends, and K′'s
//! section by the plane y = 0 on each side of the z axis revolved through 150°.
use crate::shared::parts::xz_plane;
use kentron::boolean;
use kentron::math::{Axis, Direction, Point, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Arc, Heading, Start, To};
    let (sin, cos) = 30f64.to_radians().sin_cos();
    // In the xz plane, (x, z): w and u, square to it.
    let at = |along: f64, out: f64| (along * cos - out * sin, along * sin + out * cos);
    let base = 7.0 * 25f64.to_radians().tan();
    let reach = (25.0 + base * base).sqrt();
    let (point, near, far) = (at(-2.0, 0.0), at(5.0, base), at(5.0, -base));
    let angle = |q: (f64, f64)| q.1.atan2(q.0).to_degrees();
    // K′: its half section, from the point out to the rim, round the ball's arc to its axis at
    // R w, and back along the axis, turned whole about the axis.
    let half = [Start(point.0, point.1), To(near.0, near.1), Heading(angle(near) - 90.0), Arc { radius: -reach, angle: angle(near) - 30.0 }];
    let half = sketch::profile(m, xz_plane(0.0), &half, Outcome::Face)?;
    let start = sweep::revolve(m, half, Axis::new(Point::ORIGIN, Direction::new(cos, 0.0, sin)), TAU)?.shape;
    let spin = Transform::rotation(Axis::Z, 150f64.to_radians());
    let end = modify::transform(m, start, &spin, TransformOptions::default())?;
    // K′'s section by y = 0: the z axis crosses its sides from the point at `low` and `high`.
    // Where x ≥ 0: from there out to the rim, round the ball's arc, and back to the axis.
    let cross = |q: (f64, f64)| point.1 + (q.1 - point.1) * (-point.0 / (q.0 - point.0));
    let (low, high) = (cross(far), cross(near));
    let section = [
        Start(0.0, low),
        To(far.0, far.1),
        Heading(angle(far) + 90.0),
        Arc { radius: reach, angle: angle(near) - angle(far) },
        To(0.0, high),
    ];
    let section = sketch::profile(m, xz_plane(0.0), &section, Outcome::Face)?;
    // Where x ≤ 0: the triangle between the point and the axis.
    let behind = [Start(point.0, point.1), To(0.0, low), To(0.0, high)];
    let behind = sketch::profile(m, xz_plane(0.0), &behind, Outcome::Face)?;
    let mut body = boolean::union(m, start, end)?;
    for section in [section, behind] {
        let turned = sweep::revolve(m, section, Axis::Z, 150f64.to_radians())?.shape;
        body = boolean::union(m, body, turned)?;
    }
    Ok(body)
}
