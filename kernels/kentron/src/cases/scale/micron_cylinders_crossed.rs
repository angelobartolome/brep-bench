//! Two cylinders of radius 0.003 and length 0.02 crossing at right angles at the origin, the first
//! along the x axis from x = −0.01 to 0.01, the second along the y axis from y = −0.01 to 0.01:
//! the solid common to both.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let along_x = Cylinder::new(0.003, 0.02).with_frame(frame(p(-0.01, 0.0, 0.0), Direction::X)).build(m)?;
    let along_y = Cylinder::new(0.003, 0.02).with_frame(frame(p(0.0, -0.01, 0.0), Direction::Y)).build(m)?;
    boolean::intersection(m, along_x, along_y)
}
