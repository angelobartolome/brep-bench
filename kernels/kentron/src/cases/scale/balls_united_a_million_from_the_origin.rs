//! A ball of radius 5 about (600000, −800000, 0), united with a ball of radius 4 about
//! (600002, −799997, 6), 7 from the first along (2, 3, 6)/7.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = Sphere::new(5.0).with_frame(frame(p(600000.0, -800000.0, 0.0), Direction::Z)).build(m)?;
    let second = Sphere::new(4.0).with_frame(frame(p(600002.0, -799997.0, 6.0), Direction::Z)).build(m)?;
    boolean::union(m, first, second)
}
