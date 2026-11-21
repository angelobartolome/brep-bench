//! The solid a rod fills as it is spun partway about an upright line through it: the solid
//! cylinder of radius 0.5 about the x axis from x = −3 to x = 5, turned rigidly about the z axis
//! by 120°.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) turns a solid of revolution only across a
//! ring sector it starts and ends clear of, and none is here, so the sweep is built as elementary
//! geometry gives it. Take an arm of the rod, the part along +x out to length L (5, or 3 along
//! −x). At height z its points fill the circle of radius r about the z axis over the polar
//! angles |φ| ≤ asin(w/r), w = √(0.25 − z²), as far as r² ≤ L² + w², that is r² + z² ≤ L² + 0.25
//! (whole circles where r ≤ w, which the rod at either end covers; past r = L its end face cuts
//! that down to acos(L/r) ≤ |φ| ≤ asin(w/r), a gap about φ = 0 narrower than the turn, which the
//! arm turning on sweeps over). Turned by up to 120°, the arm
//! fills, of each such circle, the polar angles from −asin(w/r) to 120° + asin(w/r): within the
//! turn, the slab |z| ≤ 0.5 inside the ball of radius √(L² + 0.25) about the origin, turned through
//! 120°; outside it, its arm's own side (|y| ≤ w), as far as that ball, at the start and at the
//! end. So the sweep is the rod with its ends made those balls' caps, where it starts and where it
//! ends, and the two arms' slices of slab and ball between.
use crate::bench::p;
use kentron::boolean;
use kentron::math::{Axis, Direction, Frame, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let turn = 120f64.to_radians();
    let (long, short) = ((25.25f64).sqrt(), (9.25f64).sqrt());
    // The rod with its ends the caps of the balls of radius √(L² + 0.25), L its arms' lengths.
    let rod = Cylinder::new(0.5, 10.0).with_frame(Frame::new(p(-4.0, 0.0, 0.0), Direction::X, Direction::Y)).build(m)?;
    let far = Sphere::new(long).build(m)?;
    let rod = boolean::intersection(m, rod, far)?;
    let tail = Cylinder::new(0.5, 3.0).with_frame(Frame::new(p(-5.0, 0.0, 0.0), Direction::X, Direction::Y)).build(m)?;
    let near = Sphere::new(short).build(m)?;
    let tail = boolean::difference(m, tail, near)?;
    let rod = boolean::difference(m, rod, tail)?;
    let end = modify::transform(m, rod, &Transform::rotation(Axis::Z, turn), TransformOptions::default())?;
    // Each arm's slice: the slab |z| ≤ 0.5 within its ball, turned through 120° from the arm.
    let mut body = boolean::union(m, rod, end)?;
    for (from, radius) in [(Direction::X, long), (Direction::NEG_X, short)] {
        let slab = Cylinder::new(radius + 1.0, 1.0).with_angle(turn).with_frame(Frame::new(p(0.0, 0.0, -0.5), Direction::Z, from)).build(m)?;
        let ball = Sphere::new(radius).build(m)?;
        let slice = boolean::intersection(m, slab, ball)?;
        body = boolean::union(m, body, slice)?;
    }
    Ok(body)
}
