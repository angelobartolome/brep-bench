//! A ball of radius 5 about the origin standing in a conical seat: the cylinder of radius 8 about
//! the z axis from z = −6 to 5 with everything above the cone z = 1 + r/2 hollowed out, the ball
//! joined to it. The seat's cone meets the ball in the concave circle of radius 4 at z = 3,
//! chamfered by a distance and an angle: 2 up the seat's cone, the chamfer making 30° with it,
//! filling the groove.
use crate::bench::p;
use crate::shared::chamfer::{ball_in_conical_seat, chamfer_rim};
use crate::Refusal;
use kentron::blend::ChamferSize;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_6;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let seated = ball_in_conical_seat(m)?;
    // (The rim, and a point on the seat's cone, away from the seams.)
    chamfer_rim(m, seated, p(0.0, 4.0, 3.0), p(0.0, 6.0, 4.0), ChamferSize::DistanceAngle(2.0, FRAC_PI_6))
}
