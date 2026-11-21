//! The solid paraboloid x² + y² ≤ 8z, z ≤ 8 (the parabola turned about the z axis), common with a
//! block reaching past it on every side but its top, which is the plane z = 10⁻⁴: the paraboloid's
//! tip, a disk 0.057 across over a bowl 10⁻⁴ deep.
use crate::bench::p;
use crate::shared::tangent::paraboloid;
use kentron::boolean;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let solid = paraboloid(m, 2.0, 8.0)?;
    let below = Cuboid::from_corners(p(-10.0, -10.0, -1.0), p(10.0, 10.0, 1e-4)).build(m)?;
    boolean::intersection(m, solid, below)
}
