//! Two balls, radius 5.511 about the origin and radius 3.317 about (4.231, 3.467, 2.653): the
//! lens common to both, its rim circle in a plane aslant to every axis.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = Sphere::new(5.511).build(m)?;
    let second = Sphere::new(3.317).with_frame(frame(p(4.231, 3.467, 2.653), Direction::Z)).build(m)?;
    boolean::intersection(m, first, second)
}
