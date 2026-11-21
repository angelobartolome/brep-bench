//! A face gear's 30 teeth cut by a tapered worm (a spiroid pair): the worm's axis is the line
//! through (0, 30, 6) along x, its thread a rack of module 2 wound on the pitch cone of radius
//! R(x) = 6 + x·tan 5° (0 ≤ x ≤ 52); the worm turns by t about its axis while the gear turns by
//! t/30 about z, t from 0 to 60π. The blank is the ring between radii 40 and 52 under a conical
//! face falling from z = −0.3 to −1.7, down to z = −9.
//!
//! Seen from the gear the worm at t is turned by t about its axis and then by −t/30 about z, so
//! a point x of the gear is in the worm at t where y = Rz(t/30)·x, turned back by −t about the
//! worm's axis, meets the thread's inequalities (this thread is no screw's: its radius changes
//! along it). The motion comes back to its start after a turn of the gear, and each point of the
//! blank comes under the worm once in it, near where the turn brings it within the thread's reach.
//!
//! The 30 tooth spaces are one space turned by 12° steps. One is traced (`solids::gear`) on 13
//! cylinders about the gear's axis, r = 39.7 … 52.3, each seen from a point 6 above the face over
//! the space, its rays reaching from the middle of the tooth's top on one side to the middle of
//! the next (found on the face, by bisection); the slices are lofted into a cutter. The gear is
//! then 30 equal cells sewn where they meet (`by_cells`); where that cannot be, the cutter's 30
//! turned copies cut from the blank at once.
use crate::solids::gear::{air, runs, space, Generating, Slice};
use crate::solids::sweep::{moved, turn_z};
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, extrude_profile_brep, revolve_profile_brep, BooleanOperation, BrepSolid, Vec3};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

struct Worm;

const TIGHT: f64 = 1e-7;
/// The thread's farthest reach from the worm's axis: R(52) + 2.5.
fn reach() -> f64 {
    6.0 + 52.0 * 5f64.to_radians().tan() + 2.5
}

impl Generating for Worm {
    fn margin(&self, x: Vec3, t: f64) -> (f64, u32) {
        let (c, s) = ((t / 30.0).cos(), (t / 30.0).sin());
        let y = p(c * x.x - s * x.y, s * x.x + c * x.y, x.z);
        let (c, s) = ((-t).cos(), (-t).sin());
        let (u, v) = (y.y - 30.0, y.z - 6.0);
        let (a, b) = (c * u - s * v, s * u + c * v);
        let r = a.hypot(b);
        let theta = b.atan2(a);
        let big = 6.0 + y.x * 5f64.to_radians().tan();
        let xi = (y.x - theta + PI).rem_euclid(TAU) - PI;
        let h = FRAC_PI_2 - (r - big) * 20f64.to_radians().tan();
        let slack = [h - xi, h + xi, 2.5 - (r - big), 2.5 + (r - big), y.x, 52.0 - y.x];
        let least = slack.iter().copied().fold(f64::INFINITY, f64::min);
        let bind = slack.iter().enumerate().filter(|(_, &v)| v - least <= TIGHT).fold(0u32, |m, (i, _)| m | 1 << i);
        (least, bind)
    }

    fn window(&self, x: Vec3) -> Option<(f64, f64, f64)> {
        let rho = x.x.hypot(x.y);
        let dz = x.z - 6.0;
        if dz.abs() >= reach() {
            return None;
        }
        // Where the turned point comes within the thread's reach of the worm's axis, on the
        // worm's side (x ≥ 0).
        let w = (reach() * reach() - dz * dz).sqrt();
        let lo = ((30.0 - w) / rho).clamp(-1.0, 1.0).asin();
        let hi = ((30.0 + w) / rho).clamp(-1.0, 1.0).asin();
        let phi = x.y.atan2(x.x);
        Some((30.0 * (lo - 0.01 - phi), 30.0 * (hi + 0.01 - phi), 0.1))
    }
}

/// The face's height at radius r.
fn face(r: f64) -> f64 {
    -0.3 - 1.4 * (r - 40.0) / 12.0
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let meridian = [p(40.0, 0.0, -9.0), p(52.0, 0.0, -9.0), p(52.0, 0.0, -1.7), p(40.0, 0.0, -0.3)];
    let blank = revolve_profile_brep(&polygon(&meridian)?, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?;
    let step = 12f64.to_radians();
    // One cutter for a lift of its rim above the face.
    let make = |lift: f64| -> Result<(BrepSolid, Vec<(f64, f64, f64)>), Refusal> {
        // The space is followed from circle to circle about the axis, r = 40.3 … 51.7: on each, the
        // run of it on the face nearest where the last two circles' middles point (at the first,
        // about 18°). Its middle angle at each radius.
        let nr = 11;
        let radii: Vec<f64> = (0..=nr).map(|k| 40.3 + 11.4 * k as f64 / nr as f64).collect();
        let mut middles: Vec<f64> = Vec::new();
        let mut spans: Vec<(f64, f64)> = Vec::new();
        for &r in &radii {
            let guess = match middles.len() {
                0 => 18f64.to_radians(),
                1 => middles[0],
                n => 2.0 * middles[n - 1] - middles[n - 2],
            };
            let found = runs(&Worm, |phi| p(r * phi.cos(), r * phi.sin(), face(r) - 0.02), guess - step, guess + step, 480);
            let &(lo, hi) = found
                .iter()
                .min_by(|x, y| ((x.0 + x.1) / 2.0 - guess).abs().total_cmp(&((y.0 + y.1) / 2.0 - guess).abs()))
                .ok_or_else(|| Refusal::from("no tooth space found on the face"))?;
            if lo <= guess - step + 1e-9 || hi >= guess + step - 1e-9 || hi - lo >= step {
                return Err(Refusal::from("a tooth space on the face reaches past a whole tooth"));
            }
            middles.push(0.5 * (lo + hi));
            spans.push((lo, hi));
        }
        let n = radii.len();
        // Each slice: seen from a point 6 above the face over the space's middle, its rays reaching
        // a quarter of the way across the tooth's top on either side: the turned copies stay apart.
        let fan = |q: [f64; 2], left: [f64; 2], right: [f64; 2]| {
            let toward = [0.0, -1.0];
            let angle = |d: [f64; 2]| {
                let d = [d[0] - q[0], d[1] - q[1]];
                (toward[0] * d[1] - toward[1] * d[0]).atan2(toward[0] * d[0] + toward[1] * d[1])
            };
            let (x, y) = (angle(left), angle(right));
            (toward, x.min(y), x.max(y))
        };
        let mut slices = Vec::new();
        // The two ends: planes square to the radius through the space's middle, at 39.6 and 52.4
        // from the axis, so wholly inside the ring's bore and outside its rim there.
        let end_plane = |c: f64, mid: f64, span: f64| -> Result<Slice, Refusal> {
            let (u, across) = ([mid.cos(), mid.sin()], [-mid.sin(), mid.cos()]);
            let point = move |q: [f64; 2]| p(c * u[0] + q[0] * across[0], c * u[1] + q[0] * across[1], q[1]);
            let face_at = move |a: f64| face(c.hypot(a));
            let width = c * step;
            let found = runs(&Worm, |a| point([a, face_at(a) - 0.02]), -width, width, 480);
            let &(lo, hi) = found
                .iter()
                .min_by(|x, y| ((x.0 + x.1) / 2.0).abs().total_cmp(&((y.0 + y.1) / 2.0).abs()))
                .ok_or_else(|| Refusal::from("no tooth space found at an end"))?;
            let _ = span;
            let (left, right) = (lo - 0.25 * (lo + width - hi), hi + 0.25 * (lo + width - hi));
            let middle = 0.5 * (lo + hi);
            let q = [middle, face_at(middle) + 6.0];
            let (toward, from, to) = fan(q, [left, face_at(left) + lift], [right, face_at(right) + lift]);
            Ok(Slice {
                q,
                toward,
                from,
                to,
                reach: 18.0,
                place: Box::new(point),
                bound: air(move |q: [f64; 2]| q[1] > face_at(q[0]) + lift),
                invert: false,
                closed: false,
            })
        };
        let extrapolate = |k0: usize, k1: usize, r: f64| middles[k0] + (middles[k1] - middles[k0]) * (r - radii[k0]) / (radii[k1] - radii[k0]);
        slices.push(end_plane(39.6, extrapolate(0, 1, 39.6), 0.0)?);
        for (k, &r) in radii.iter().enumerate() {
            let (lo, hi) = spans[k];
            let top = face(r) + lift;
            let middle = 0.5 * (lo + hi);
            let q = [r * middle, face(r) + 6.0];
            let (toward, from, to) = fan(q, [r * (lo - 0.25 * (lo + step - hi)), top], [r * (hi + 0.25 * (lo + step - hi)), top]);
            slices.push(Slice {
                q,
                toward,
                from,
                to,
                reach: 18.0,
                place: Box::new(move |c: [f64; 2]| p(r * (c[0] / r).cos(), r * (c[0] / r).sin(), c[1])),
                bound: air(move |c: [f64; 2]| c[1] > top),
                invert: false,
                closed: false,
            });
        }
        slices.push(end_plane(52.4, extrapolate(n - 2, n - 1, 52.4), 0.0)?);
        let found: Vec<(f64, f64, f64)> = radii.iter().zip(&spans).map(|(&r, &(lo, hi))| (r, lo, hi)).collect();
        Ok((space(&Worm, &slices, 120, 25, 0.01)?, found))
    };
    // One tooth space, traced once.
    let (cutter, spans) = make(0.15)?;
    if let Some(gear) = by_cells(&blank, &cutter, &spans, step)? {
        return Ok(gear);
    }
    // Else its 30 turned copies cut from the blank in one n-ary subtract (neighbouring copies never
    // touch: half a tooth top lies between them).
    let mut operands = vec![blank];
    for k in 0..30 {
        operands.push(moved(&cutter, turn_z(step * k as f64, p(0.0, 0.0, 0.0))?)?);
    }
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}

/// The gear as 30 equal cells sewn where they meet (`solids::gear::pattern_sewn`), with no boolean
/// between them: the cells are divided by upright cylinders through the circle through the tooth's
/// middle on the face at the first, middle and last traced radii (the tooth spirals as the spaces
/// do), kept only where that circle stays inside the tooth at every traced radius. One cell is the
/// blank between two such cylinders 12° apart, less the traced space and its two neighbours (so it
/// is the gear's part there, wherever a space reaches). None where the circle leaves the tooth or
/// the cells do not sew.
fn by_cells(blank: &BrepSolid, cutter: &BrepSolid, spans: &[(f64, f64, f64)], step: f64) -> Result<Option<BrepSolid>, Refusal> {
    use crate::solids::gear::pattern_sewn;
    use crate::solids::swept::{planar, Piece};
    // The tooth after the traced space: from its end to the next space's start, on the face.
    let middle = |&(r, lo, hi): &(f64, f64, f64)| (r, 0.5 * (hi + lo + step));
    let at = |(r, a): (f64, f64)| [r * a.cos(), r * a.sin()];
    let n = spans.len();
    let (a, b, c) = (at(middle(&spans[0])), at(middle(&spans[n / 2])), at(middle(&spans[n - 1])));
    let (bx, by, cx, cy) = (b[0] - a[0], b[1] - a[1], c[0] - a[0], c[1] - a[1]);
    let det = 2.0 * (bx * cy - by * cx);
    if det.abs() < 1e-12 {
        return Ok(None);
    }
    let (bb, cc) = (bx * bx + by * by, cx * cx + cy * cy);
    let centre = [a[0] + (cy * bb - by * cc) / det, a[1] + (bx * cc - cx * bb) / det];
    let radius = (a[0] - centre[0]).hypot(a[1] - centre[1]);
    let wrap = |x: f64, near: f64| near + (x - near + PI).rem_euclid(TAU) - PI;
    // Where the circle about `o` meets the circle of `rho` about the axis, nearest the angle `near`.
    let meet = |o: [f64; 2], rho: f64, near: f64| -> Option<f64> {
        let d = o[0].hypot(o[1]);
        let cos = (rho * rho + d * d - radius * radius) / (2.0 * rho * d);
        if cos.abs() > 1.0 {
            return None;
        }
        let (base, off) = (o[1].atan2(o[0]), cos.acos());
        let (p1, p2) = (wrap(base + off, near), wrap(base - off, near));
        Some(if (p1 - near).abs() < (p2 - near).abs() { p1 } else { p2 })
    };
    // Inside the tooth (a tenth of its width clear of either space) at every traced radius.
    for span in spans {
        let (r, lo, hi) = *span;
        let (_, m) = middle(span);
        let Some(angle) = meet(centre, r, m) else { return Ok(None) };
        let clear = 0.1 * (lo + step - hi);
        if !(angle > hi + clear && angle < lo + step - clear) {
            return Ok(None);
        }
    }
    // The cell's plan: between the circle and the circle turned back 12°, from radius 38 to 54 (past
    // the ring), the tooth middle's angle extrapolated from the traced ends.
    let below = [centre[0] * step.cos() + centre[1] * step.sin(), -centre[0] * step.sin() + centre[1] * step.cos()];
    let (m0, m1) = (middle(&spans[0]), middle(&spans[1]));
    let (k0, k1) = (middle(&spans[n - 2]), middle(&spans[n - 1]));
    let near = |rho: f64| if rho < 46.0 { m0.1 + (m1.1 - m0.1) * (rho - m0.0) / (m1.0 - m0.0) } else { k1.1 + (k1.1 - k0.1) * (rho - k1.0) / (k1.0 - k0.0) };
    let point = |o: [f64; 2], rho: f64, shift: f64| -> Option<[f64; 2]> { meet(o, rho, near(rho) - shift).map(|t| [rho * t.cos(), rho * t.sin()]) };
    let (Some(a_in), Some(a_out), Some(b_in), Some(b_out)) = (point(centre, 38.0, 0.0), point(centre, 54.0, 0.0), point(below, 38.0, step), point(below, 54.0, step)) else {
        return Ok(None);
    };
    let angle = |o: [f64; 2], q: [f64; 2]| (q[1] - o[1]).atan2(q[0] - o[0]);
    let arc = |o: [f64; 2], r: f64, from: [f64; 2], to: [f64; 2]| {
        let s = angle(o, from);
        Piece::Arc { centre: o, radius: r, start: s, end: wrap(angle(o, to), s) }
    };
    let plan = [
        arc(centre, radius, a_in, a_out),
        arc([0.0, 0.0], 54.0, a_out, b_out),
        arc(below, radius, b_out, b_in),
        arc([0.0, 0.0], 38.0, b_in, a_in),
    ];
    let prism = extrude_profile_brep(&planar(&plan, p(0.0, 0.0, -10.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0))?, p(0.0, 0.0, 1.0), 11.0)?;
    let piece = boolean_operation_nary(&[blank.clone(), prism], BooleanOperation::Intersect)?;
    let turned = |k: f64| -> Result<BrepSolid, String> { moved(cutter, turn_z(step * k, p(0.0, 0.0, 0.0))?) };
    let cell = boolean_operation_nary(&[piece, turned(-1.0)?, cutter.clone(), turned(1.0)?], BooleanOperation::Subtract)?;
    let on = |o: [f64; 2], q: Vec3| ((q.x - o[0]).hypot(q.y - o[1]) - radius).abs() < 1e-7 * radius;
    Ok(pattern_sewn(&cell, step, 30, &|q| on(centre, q) || on(below, q)).ok())
}
