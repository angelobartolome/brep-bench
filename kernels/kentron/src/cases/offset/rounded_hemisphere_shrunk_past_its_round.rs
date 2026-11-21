//! The half of the ball of radius 5 about the origin above z = 0, its rim rounded to radius 1, shrunk
//! by 3/2: every point at least 3/2 from its outside. The round drops; the result is the part of the
//! ball of radius 7/2 above z = 3/2.
use crate::Refusal;
use crate::shared::offset::{on_rim, rounded};
use kentron::offset::{self, Join};
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let half = Sphere::new(5.0).with_latitudes(0.0, FRAC_PI_2).build(m)?;
    let rounded_half = rounded(m, half, &[on_rim(5.0, 0.0)], 1.0)?;
    Ok(offset::shape(m, rounded_half, -1.5, Join::Arc)?)
}
