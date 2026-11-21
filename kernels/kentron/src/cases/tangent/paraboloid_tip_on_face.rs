//! A block from (−10, −10, 0) to (10, 10, 10), united with the solid paraboloid x² + y² ≤ 8z,
//! z ≤ 12.5, whose vertex touches the block's bottom face from inside at the origin and which
//! stands out of its top. The paraboloid is the parabola turned about the z axis.
use crate::bench::p;
use crate::shared::tangent::paraboloid;
use kentron::boolean;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(-10.0, -10.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let solid = paraboloid(m, 2.0, 12.5)?;
    boolean::union(m, block, solid)
}
