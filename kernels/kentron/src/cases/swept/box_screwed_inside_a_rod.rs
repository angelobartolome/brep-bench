//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a sealed helical
//! tunnel inside it: everything the box 2.5 ≤ x ≤ 3.5, |y| ≤ 0.5, 2 ≤ z ≤ 3 passes through as it is
//! carried along the screw about the z axis, right-handed, rising 4 a turn, from a turn of 0° to
//! 720°, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) screws only solids of revolution, and only
//! through a rod's ends, so the tunnel is built as elementary geometry gives it, as a box slid is.
//! A point the box covers at some instant was inside it when the motion started or was carried in
//! through one of its leading faces, where the screw's velocity v = (−y, x, 0) + (4/2π) ẑ points
//! out of it: its top (normal ẑ), its side y = 0.5 (normal ŷ, n·v = x), and the halves of its ends
//! x = 2.5 where y ≥ 0 (n·v = y) and x = 3.5 where y ≤ 0 (n·v = −y). So the tunnel is the box where
//! it starts and those four faces screwed (`sweep::screw_extrude`): none of them folds as it
//! screws, the velocity running along each only on its rim, and the turns, 4 apart, never meet.
use crate::bench::p;
use crate::shared::sweep::polyline;
use crate::shared::swept::cut_each;
use kentron::make;
use kentron::math::{Axis, Point};
use kentron::primitives::{Cuboid, Cylinder};
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    let start = Cuboid::from_corners(p(2.5, -0.5, 2.0), p(3.5, 0.5, 3.0)).build(m)?;
    // The leading faces, each by its corners.
    let faces: [[Point; 4]; 4] = [
        [p(2.5, -0.5, 3.0), p(3.5, -0.5, 3.0), p(3.5, 0.5, 3.0), p(2.5, 0.5, 3.0)],
        [p(2.5, 0.5, 2.0), p(3.5, 0.5, 2.0), p(3.5, 0.5, 3.0), p(2.5, 0.5, 3.0)],
        [p(2.5, 0.0, 2.0), p(2.5, 0.5, 2.0), p(2.5, 0.5, 3.0), p(2.5, 0.0, 3.0)],
        [p(3.5, -0.5, 2.0), p(3.5, 0.0, 2.0), p(3.5, 0.0, 3.0), p(3.5, -0.5, 3.0)],
    ];
    let mut pieces = vec![start];
    for corners in faces {
        let outline = polyline(m, &corners, true)?;
        let face = make::planar_face(m, outline)?;
        // Two turns, rising 4 each.
        pieces.push(sweep::screw_extrude(m, face, Axis::Z, 4.0, 8.0)?.shape);
    }
    cut_each(m, rod, &pieces)
}
