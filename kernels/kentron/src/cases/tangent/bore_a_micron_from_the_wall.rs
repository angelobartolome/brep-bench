//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, bored right through along z by a
//! hole of radius 4 about the line x = 0.999999, y = 0: the wall between them is a millionth
//! thick at (5, 0, z). The drill runs from z = −1 to z = 11.
use crate::bench::p;
use kentron::boolean;
use kentron::math::{Direction, Frame};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cylinder = Cylinder::new(5.0, 10.0).build(m)?;
    // its seam on the far side from the thin wall, not beside the cylinder's own
    let axis = Frame::new(p(0.999999, 0.0, -1.0), Direction::Z, Direction::new(-1.0, 0.0, 0.0));
    let drill = Cylinder::new(4.0, 12.0).with_frame(axis).build(m)?;
    boolean::difference(m, cylinder, drill)
}
