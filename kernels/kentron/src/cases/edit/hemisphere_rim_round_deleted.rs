//! The half above z = 0 of the ball of radius 5 about the origin, its rim rounded to radius 1: a
//! torus about the circle of radius √15 at z = 1. The round is deleted: the flat runs on outward
//! and the ball down until they meet in the circle of radius 5 at z = 0.
use crate::bench::{edge_through, p};
use crate::shared::edit::{deleted, round_middle};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let hemisphere = Sphere::new(5.0).with_latitudes(0.0, FRAC_PI_2).build(m)?;
    let rim = edge_through(m, hemisphere, p(0.0, 5.0, 0.0))?;
    let rounded = blend::fillet(m, hemisphere, &[rim], 1.0)?;
    // The round touches the flat below its center and the ball away from the ball's center, 4
    // from it.
    let (r, z) = round_middle((15f64.sqrt(), 1.0), (0.0, -1.0), (15f64.sqrt() / 4.0, 0.25), 1.0);
    deleted(m, rounded, &[p(0.0, r, z)])
}
