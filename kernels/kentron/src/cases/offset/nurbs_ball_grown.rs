//! The ball of radius 5 about the origin whose whole surface is the standard biquadratic NURBS sphere
//! of `geometry/nurbs-sphere`, grown by 2: every point within 2 of it, the ball of radius 7.
use crate::Refusal;
use crate::cases::geometry::nurbs_sphere;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let ball = nurbs_sphere::build(m)?;
    Ok(offset::shape(m, ball, 2.0, Join::Arc)?)
}
