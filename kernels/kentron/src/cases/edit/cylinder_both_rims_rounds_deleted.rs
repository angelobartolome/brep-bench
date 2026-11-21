//! A cylinder of radius 6 on the z axis from z = 0 to z = 10, both its rims rounded to radius 2:
//! quarter tori about the circles of radius 4 at z = 8 and z = 2. Both rounds are deleted
//! together: the ends and the side run on to meet, and the result is the plain cylinder.
use crate::bench::{edge_through, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};
use std::f64::consts::SQRT_2;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cylinder = Cylinder::new(6.0, 10.0).build(m)?;
    let top = edge_through(m, cylinder, p(0.0, 6.0, 10.0))?;
    let bottom = edge_through(m, cylinder, p(0.0, 6.0, 0.0))?;
    let rounded = blend::fillet(m, cylinder, &[top, bottom], 2.0)?;
    // Each round halfway between its end and the side.
    deleted(m, rounded, &[p(0.0, 4.0 + SQRT_2, 8.0 + SQRT_2), p(0.0, 4.0 + SQRT_2, 2.0 - SQRT_2)])
}
