//! Two balls of radius 5, about the origin and about (4.8, 0, 3.6), their poles on the verticals
//! through their centres: their union. The circle where they meet runs through a pole of each.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = Sphere::new(5.0).build(m)?;
    let second = Sphere::new(5.0).with_frame(frame(p(4.8, 0.0, 3.6), Direction::Z)).build(m)?;
    boolean::union(m, first, second)
}
