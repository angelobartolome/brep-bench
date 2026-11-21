//! A cone frustum about the z axis from radius 6 at z = 0 to radius 3 at z = 20, with a flat milled
//! along its side: everything with y < −4 cut away, the flat running up from the base until the
//! cone narrows past it. The flat is deleted: the cone closes over it, the base grows back to a
//! whole disc, and the result is the plain frustum.
use crate::bench::p;
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::primitives::{Cone, Cuboid};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let frustum = Cone::new(6.0, 3.0, 20.0).build(m)?;
    let below = Cuboid::from_corners(p(-7.0, -7.0, -1.0), p(7.0, -4.0, 21.0)).build(m)?;
    let flattened = boolean::difference(m, frustum, below)?;
    deleted(m, flattened, &[p(0.0, -4.0, 1.0)])
}
