//! A ball of radius 0.005 about the origin, united with a ball of radius 0.004 about
//! (0.002, 0.003, 0.006), 0.007 from the first.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = Sphere::new(0.005).build(m)?;
    let second = Sphere::new(0.004).with_frame(frame(p(0.002, 0.003, 0.006), Direction::Z)).build(m)?;
    boolean::union(m, first, second)
}
