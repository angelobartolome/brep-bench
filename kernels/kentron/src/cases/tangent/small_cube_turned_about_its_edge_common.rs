//! A small cube of side 0.001 from the origin to (0.001, 0.001, 0.001), and a copy of it turned
//! 1.2° about the z axis, the line of one of its edges: the part common to both, a prism over the
//! quadrilateral the two squares share.
use kentron::boolean;
use kentron::math::{Axis, Transform};
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let cube = Cuboid::new(0.001, 0.001, 0.001).build(m)?;
    let turned = cube.moved(&Transform::rotation(Axis::Z, 1.2_f64.to_radians()));
    boolean::intersection(m, cube, turned)
}
