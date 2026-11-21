//! The upper half of the ball of radius 5 about the origin, shelled outward to a wall 2 thick,
//! open at its dome. The flat face moves 2 down and is extended until it meets the sphere: the
//! result is the slice of the ball between z = −2 and z = 0.
use crate::Refusal;
use crate::bench::p;
use crate::shared::shell;
use kentron::offset::Join;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let half = Sphere::new(5.0).with_latitudes(0.0, FRAC_PI_2).build(m)?;
    // open at the dome, picked by its pole
    shell::outward(m, half, &[p(0.0, 0.0, 5.0)], 2.0, Join::Arc)
}
