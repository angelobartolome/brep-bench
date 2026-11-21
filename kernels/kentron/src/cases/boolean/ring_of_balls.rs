//! Twelve balls of radius 3, their centres evenly spaced on the circle of radius 10 about the z
//! axis in the plane z = 0, the first at (10, 0, 0), each overlapping its two neighbours: joined
//! one after another into one ring.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::{Axis, Direction, Transform};
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = Sphere::new(3.0).with_frame(frame(p(10.0, 0.0, 0.0), Direction::Z)).build(m)?;
    let mut ring = first;
    for k in 1..12 {
        let ball = first.moved(&Transform::rotation(Axis::Z, (30.0 * k as f64).to_radians()));
        ring = boolean::union(m, ring, ball)?;
    }
    Ok(ring)
}
