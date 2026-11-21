//! The block from the origin to (20, 10, 10), the two long edges of its top, where it meets the
//! sides y = 0 and y = 10, rounded to radius 2: quarter cylinders about the lines y = 2, z = 8 and
//! y = 8, z = 8. Both rounds are deleted together, and the result is the box again.
use crate::bench::{edge_through, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};
use std::f64::consts::SQRT_2;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(20.0, 10.0, 10.0)).build(m)?;
    let front = edge_through(m, block, p(10.0, 0.0, 10.0))?;
    let back = edge_through(m, block, p(10.0, 10.0, 10.0))?;
    let rounded = blend::fillet(m, block, &[front, back], 2.0)?;
    // Each round halfway between the top and its side.
    deleted(m, rounded, &[p(10.0, 2.0 - SQRT_2, 8.0 + SQRT_2), p(10.0, 8.0 + SQRT_2, 8.0 + SQRT_2)])
}
