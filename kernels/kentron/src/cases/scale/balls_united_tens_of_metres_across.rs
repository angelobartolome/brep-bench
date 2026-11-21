//! A ball of radius 50000 about the origin, united with a ball of radius 40000 about
//! (20000, 30000, 60000), 70000 from the first.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = Sphere::new(50000.0).build(m)?;
    let second = Sphere::new(40000.0).with_frame(frame(p(20000.0, 30000.0, 60000.0), Direction::Z)).build(m)?;
    boolean::union(m, first, second)
}
