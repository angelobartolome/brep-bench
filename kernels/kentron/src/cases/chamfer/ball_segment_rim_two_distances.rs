//! The part of a ball of radius 5 about the origin below the plane z = 3, its flat face the disc
//! of radius 4 at z = 3. The rim where that face meets the ball, at about 126.9°, chamfered with
//! two distances, each in a straight line from the rim: 2 on the flat face and 1 on the ball.
use crate::bench::p;
use crate::shared::chamfer::chamfer_rim;
use crate::Refusal;
use kentron::blend::ChamferSize;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // The sphere primitive up to latitude asin(3/5), closed by its flat disc.
    let segment = Sphere::new(5.0).with_latitudes(-FRAC_PI_2, (3.0f64 / 5.0).asin()).build(m)?;
    // (The rim, and a point on the flat face, away from the ball's seam.)
    chamfer_rim(m, segment, p(0.0, 4.0, 3.0), p(0.0, 1.0, 3.0), ChamferSize::Distances(2.0, 1.0))
}
