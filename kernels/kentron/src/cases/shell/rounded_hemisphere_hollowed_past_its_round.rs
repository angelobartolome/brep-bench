//! The half of the ball of radius 5 about the origin above z = 0, its rim rounded to radius 1,
//! hollowed to walls 2 thick. The round moved inward drops: the cavity is the part of the ball of
//! radius 3 above z = 2.
use crate::Refusal;
use crate::shared::offset::{on_rim, rounded};
use crate::shared::shell;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let half = Sphere::new(5.0).with_latitudes(0.0, FRAC_PI_2).build(m)?;
    let rounded_half = rounded(m, half, &[on_rim(5.0, 0.0)], 1.0)?;
    shell::hollowed(m, rounded_half, 2.0)
}
