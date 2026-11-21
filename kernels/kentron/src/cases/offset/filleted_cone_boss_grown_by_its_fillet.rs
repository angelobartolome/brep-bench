//! The plate, the cylinder of radius 8 about the z axis from z = −2 to z = 0, and on it the cone
//! frustum from radius 4 at z = 0 to radius 1 at z = 6, joined, the concave edge where they meet
//! filleted to radius 1; grown by 1: every point within 1 of it. The fillet grown by exactly its radius collapses onto its centre circle, and the
//! grown top and cone meet in a sharp concave edge.
use crate::Refusal;
use crate::shared::offset::on_plate;
use kentron::offset::{self, Join};
use kentron::primitives::Cone;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let boss = Cone::new(4.0, 1.0, 6.0).build(m)?;
    let solid = on_plate(m, 2.0, boss, 4.0, 1.0)?;
    Ok(offset::shape(m, solid, 1.0, Join::Arc)?)
}
