//! The turned post, the cylinder of radius 5 on the z axis from z = 0 to z = 6 topped by the cone
//! frustum from radius 5 at z = 6 to radius 2 at z = 10, the edge between them rounded to radius 1,
//! shrunk by 2: every point at least 2 from its outside. The round drops; the moved cylinder and
//! cone meet in a sharp edge.
use crate::Refusal;
use crate::shared::offset::turned_post;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let post = turned_post(m)?;
    Ok(offset::shape(m, post, -2.0, Join::Arc)?)
}
