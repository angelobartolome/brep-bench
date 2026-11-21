//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a flat milled
//! along its whole length: everything with y < −4 cut away. The flat is deleted: the cylinder
//! closes over it, the ends grow back to whole discs, and the result is the plain shaft.
use crate::bench::p;
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let shaft = Cylinder::new(5.0, 20.0).build(m)?;
    let below = Cuboid::from_corners(p(-6.0, -6.0, -1.0), p(6.0, -4.0, 21.0)).build(m)?;
    let flattened = boolean::difference(m, shaft, below)?;
    deleted(m, flattened, &[p(0.0, -4.0, 10.0)])
}
