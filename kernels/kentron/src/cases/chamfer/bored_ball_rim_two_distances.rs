//! A ball of radius 5 about the origin with a hole of radius 2 bored through it along the z axis.
//! The upper of the two circles where the bore meets the ball, at z = √21, chamfered with two
//! distances, each in a straight line from it: 2 down the bore and 1 on the ball. The lower rim
//! stays sharp.
use crate::bench::{frame, p};
use crate::shared::chamfer::chamfer_rim;
use crate::Refusal;
use kentron::blend::ChamferSize;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let ball = Sphere::new(5.0).build(m)?;
    let bore = Cylinder::new(2.0, 12.0).with_frame(frame(p(0.0, 0.0, -6.0), Direction::Z)).build(m)?;
    let bored = boolean::difference(m, ball, bore)?;
    // (The upper rim, and a point on the bore, away from the seams.)
    chamfer_rim(m, bored, p(0.0, 2.0, 21f64.sqrt()), p(0.0, 2.0, 0.0), ChamferSize::Distances(2.0, 1.0))
}
