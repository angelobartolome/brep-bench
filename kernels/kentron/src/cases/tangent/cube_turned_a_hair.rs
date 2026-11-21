//! A cube of side 10 from (−5, −5, 0) to (5, 5, 10), united with a copy of itself turned 0.001°
//! about the z axis: a prism over the sixteen-sided union of the two squares.
use crate::bench::p;
use kentron::boolean;
use kentron::math::{Axis, Transform};
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cube = Cuboid::from_corners(p(-5.0, -5.0, 0.0), p(5.0, 5.0, 10.0)).build(m)?;
    let turned = cube.moved(&Transform::rotation(Axis::Z, 0.001_f64.to_radians()));
    boolean::union(m, cube, turned)
}
