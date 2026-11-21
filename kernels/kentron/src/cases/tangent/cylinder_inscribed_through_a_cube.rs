//! A cube of side 10 from (−5, −5, −5) to (5, 5, 5), united with a solid cylinder of radius 5 on
//! the x axis from x = −10 to x = 10. Inside the cube the cylinder's wall touches the faces
//! y = ±5 and z = ±5 along their middle lines; its seam is turned to the line y = z of the wall,
//! off those lines.
use crate::bench::{d, p};
use kentron::boolean;
use kentron::math::{Direction, Frame};
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cube = Cuboid::from_corners(p(-5.0, -5.0, -5.0), p(5.0, 5.0, 5.0)).build(m)?;
    // the seam where the frame's x points: along y = z
    let along_x = Frame::new(p(-10.0, 0.0, 0.0), Direction::X, d(0.0, 1.0, 1.0));
    let cylinder = Cylinder::new(5.0, 20.0).with_frame(along_x).build(m)?;
    boolean::union(m, cube, cylinder)
}
