//! A cylinder of radius 6 on the z axis from z = 0 to z = 10, its top rim rounded to radius 2: a
//! quarter torus about the circle of radius 4 at z = 8. The round is deleted: the top and the side
//! run on to meet, and the result is the plain cylinder.
use crate::bench::{edge_through, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};
use std::f64::consts::SQRT_2;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cylinder = Cylinder::new(6.0, 10.0).build(m)?;
    let rim = edge_through(m, cylinder, p(0.0, 6.0, 10.0))?;
    let rounded = blend::fillet(m, cylinder, &[rim], 2.0)?;
    // The round halfway between the top and the side.
    deleted(m, rounded, &[p(0.0, 4.0 + SQRT_2, 8.0 + SQRT_2)])
}
