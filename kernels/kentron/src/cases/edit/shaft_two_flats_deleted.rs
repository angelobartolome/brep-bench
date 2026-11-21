//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with two opposite
//! flats milled along its whole length: everything with |x| > 4 cut away. Both flats are deleted
//! together: the cylinder closes over them, the ends grow back to whole discs, and the result is
//! the plain shaft.
use crate::bench::p;
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let shaft = Cylinder::new(5.0, 20.0).build(m)?;
    let between = Cuboid::from_corners(p(-4.0, -6.0, -1.0), p(4.0, 6.0, 21.0)).build(m)?;
    let flattened = boolean::intersection(m, shaft, between)?;
    deleted(m, flattened, &[p(4.0, 0.0, 10.0), p(-4.0, 0.0, 10.0)])
}
