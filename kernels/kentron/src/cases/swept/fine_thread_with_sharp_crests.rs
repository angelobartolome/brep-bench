//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by a bar
//! carried along a fine screw: the box 4.5 ≤ x ≤ 12, |y| ≤ 8, |z| ≤ 0.45, screwed about the z axis,
//! right-handed, rising 1 a turn, from a turn of −180° to +7380°. Everything the bar passes
//! through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) screws only solids of revolution, so the cut
//! is built as elementary geometry gives it, as a box slid is. The screw keeps each point's
//! distance from the z axis, so only the bar's part within 5 of it ever enters the rod: what lies
//! beyond x = 4.5 inside the circle of radius 5. A point that part covers at some instant was inside
//! it when the motion started (below the rod) or was carried in through one of its leading faces,
//! where the screw's velocity v = (−y, x, 0) + (1/2π) ẑ points out of it: its top z = 0.45
//! (normal ẑ), and the half of its tip x = 4.5 where y ≥ 0 (normal −x̂, n·v = y). So the rod loses
//! those two faces screwed (`sweep::screw_extrude`): neither folds as it screws, the velocity
//! running along each only on its rim, and each sweeps a ramp thinner than the screw's lead.
use crate::bench::p;
use crate::shared::sweep::polyline;
use crate::shared::swept::cut_each;
use kentron::make;
use kentron::math::{Axis, Direction, Frame, Transform, Vector};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::Cylinder;
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::PI;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Arc, Heading, Start, To};
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    // Where the tip's plane x = 4.5 meets the circle of radius 5: y = ±h, at polar angle ±α.
    let h = (25.0f64 - 4.5 * 4.5).sqrt();
    let alpha = h.atan2(4.5).to_degrees();
    // The top within the rod: from (4.5, −h) along the tip to (4.5, h), round the circle back.
    let top = [Start(4.5, -h), To(4.5, h), Heading(alpha - 90.0), Arc { radius: -5.0, angle: 2.0 * alpha }];
    let top = sketch::profile(m, Frame::new(p(0.0, 0.0, 0.45), Direction::Z, Direction::X), &top, Outcome::Face)?;
    let tip = polyline(m, &[p(4.5, 0.0, -0.45), p(4.5, h, -0.45), p(4.5, h, 0.45), p(4.5, 0.0, 0.45)], true)?;
    let tip = make::planar_face(m, tip)?;
    // The screw from a turn of −180° (lowered 0.5) through 21 turns, rising 21.
    let start = Transform::translation(Vector::new(0.0, 0.0, -0.5)) * Transform::rotation(Axis::Z, -PI);
    let mut pieces = Vec::new();
    for face in [top, tip] {
        let face = modify::transform(m, face, &start, TransformOptions::default())?;
        pieces.push(sweep::screw_extrude(m, face, Axis::Z, 1.0, 21.0)?.shape);
    }
    cut_each(m, rod, &pieces)
}
