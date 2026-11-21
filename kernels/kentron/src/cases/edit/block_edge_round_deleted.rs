//! The block from the origin to (20, 10, 10), the edge where its top meets its side x = 20 rounded
//! to radius 2, a quarter cylinder about the line x = 18, z = 8. The round is deleted: the top and
//! the side run on to meet, and the result is the box again.
use crate::bench::{edge_through, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};
use std::f64::consts::SQRT_2;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(20.0, 10.0, 10.0)).build(m)?;
    let edge = edge_through(m, block, p(20.0, 5.0, 10.0))?;
    let rounded = blend::fillet(m, block, &[edge], 2.0)?;
    // The round halfway between the top and the side.
    deleted(m, rounded, &[p(18.0 + SQRT_2, 5.0, 8.0 + SQRT_2)])
}
