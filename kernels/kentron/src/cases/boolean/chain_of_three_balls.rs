//! Three balls united one after another: radius 5 about the origin, radius 4 about (7, 0, 0) and
//! radius 3 about (10, 4, 0). The middle ball overlaps both others, which do not touch.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = Sphere::new(5.0).build(m)?;
    let second = Sphere::new(4.0).with_frame(frame(p(7.0, 0.0, 0.0), Direction::Z)).build(m)?;
    let third = Sphere::new(3.0).with_frame(frame(p(10.0, 4.0, 0.0), Direction::Z)).build(m)?;
    let two = boolean::union(m, first, second)?;
    boolean::union(m, two, third)
}
