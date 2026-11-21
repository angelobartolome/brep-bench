//! A cylinder of radius 5 and height 10 about the z axis from z = 0, less a cylinder of radius
//! 4.99997 on the same axis between the same two planes: a tube whose wall is 0.00003 thick.
use kentron::boolean;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let outer = Cylinder::new(5.0, 10.0).build(m)?;
    let bore = Cylinder::new(4.99997, 10.0).build(m)?;
    boolean::difference(m, outer, bore)
}
