//! The drum of `geometry/saddle-drum` (radius 8 about the z axis, floor z = 0, top the saddle
//! z = 10 + xy/8 as the bilinear B-spline patch on the corners of the square −10 ≤ x, y ≤ 10)
//! with everything above the plane z = 10 cut away by a block reaching past it on every side but
//! its floor. The plane is tangent to the saddle at its saddle point.
use crate::bench::p;
use crate::shared::geometry::saddle;
use kentron::boolean;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let column = saddle(m)?;
    let drum = Cylinder::new(8.0, 30.0).build(m)?;
    let drum = boolean::intersection(m, column, drum)?;
    let above = Cuboid::from_corners(p(-20.0, -20.0, 10.0), p(20.0, 20.0, 40.0)).build(m)?;
    boolean::difference(m, drum, above)
}
