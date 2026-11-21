//! A frustum on the z axis widening from radius 2 at z = 0 to 4 at z = 3, capped by the part of
//! the ball of radius 5 about the origin above z = 3. The convex rim where the ball meets the
//! frustum's cone, chamfered by a distance and an angle: 1 down the cone, the chamfer making 60°
//! with it.
use crate::bench::p;
use crate::shared::chamfer::{capped_frustum, chamfer_rim};
use crate::Refusal;
use kentron::blend::ChamferSize;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_3;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let capped = capped_frustum(m)?;
    // (The rim, and a point on the frustum's cone, away from the seams.)
    chamfer_rim(m, capped, p(0.0, 4.0, 3.0), p(0.0, 3.0, 1.5), ChamferSize::DistanceAngle(1.0, FRAC_PI_3))
}
