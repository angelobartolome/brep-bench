//! Two solid tori of middle-circle radius 3 and tube radius 0.5, linked through each other without
//! touching: one about the z axis at the origin, one about the line through (3, 0, 0) parallel
//! to y. Their union keeps both bodies.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Torus;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = Torus::new(3.0, 0.5).build(m)?;
    let second = Torus::new(3.0, 0.5).with_frame(frame(p(3.0, 0.0, 0.0), Direction::Y)).build(m)?;
    boolean::union(m, first, second)
}
