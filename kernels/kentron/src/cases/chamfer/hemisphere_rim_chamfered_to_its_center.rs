//! The upper half of a ball of radius 5 about the origin, z ≥ 0, the circle where its flat face
//! meets the ball chamfered with two distances, each in a straight line from the rim: 5 on the
//! flat face, the whole way to its centre, and 1 on the ball. The flat face shrinks to a point
//! and the solid is a spherical sector.
use crate::bench::p;
use crate::shared::chamfer::chamfer_rim;
use crate::Refusal;
use kentron::blend::ChamferSize;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let hemisphere = Sphere::new(5.0).with_latitudes(0.0, FRAC_PI_2).build(m)?;
    // (Points on the rim and on the flat face away from the ball's seam.)
    chamfer_rim(m, hemisphere, p(0.0, 5.0, 0.0), p(0.0, 1.0, 0.0), ChamferSize::Distances(5.0, 1.0))
}
