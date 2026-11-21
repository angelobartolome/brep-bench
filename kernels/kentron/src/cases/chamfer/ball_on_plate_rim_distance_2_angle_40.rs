//! A plate, the cylinder of radius 8 on the z axis from z = −6 to 0, joined to the ball of radius
//! 5 about the origin, whose upper half stands on the plate. The concave circle where the ball
//! meets the plate's top, chamfered by a distance and an angle: 2 out on the plate's top, the
//! chamfer making 40° with it, filling the groove.
use crate::bench::p;
use crate::shared::chamfer::{ball_on_plate, chamfer_rim};
use crate::Refusal;
use kentron::blend::ChamferSize;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = ball_on_plate(m)?;
    // (The rim, and a point on the plate's top, away from the seams.)
    chamfer_rim(m, plate, p(0.0, 5.0, 0.0), p(0.0, 6.5, 0.0), ChamferSize::DistanceAngle(2.0, 40f64.to_radians()))
}
