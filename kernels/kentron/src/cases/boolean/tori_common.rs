//! Two solid tori about the z axis, middle circle of radius 5 and tube of radius 2, the first's
//! middle circle in z = −1 and the second's in z = 1: their common part, a ring whose section is
//! the lens of two disks.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Torus;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let lower = Torus::new(5.0, 2.0).with_frame(frame(p(0.0, 0.0, -1.0), Direction::Z)).build(m)?;
    let upper = Torus::new(5.0, 2.0).with_frame(frame(p(0.0, 0.0, 1.0), Direction::Z)).build(m)?;
    boolean::intersection(m, lower, upper)
}
