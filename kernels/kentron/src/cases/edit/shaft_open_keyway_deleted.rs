//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with an open keyway
//! cut into it: the box −1 ≤ x ≤ 1, y ≤ −3, z ≥ 8 cut away. The keyway's floor, two walls and end
//! are deleted: the cylinder closes over it, the top grows back to a whole disc, and the result is
//! the plain shaft.
use crate::bench::p;
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let shaft = Cylinder::new(5.0, 20.0).build(m)?;
    let key = Cuboid::from_corners(p(-1.0, -6.0, 8.0), p(1.0, -3.0, 21.0)).build(m)?;
    let keyed = boolean::difference(m, shaft, key)?;
    let (floor, end) = (p(0.0, -3.0, 15.0), p(0.0, -3.5, 8.0));
    deleted(m, keyed, &[floor, p(-1.0, -3.5, 15.0), p(1.0, -3.5, 15.0), end])
}
