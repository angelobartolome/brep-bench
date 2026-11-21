//! A post, the cylinder of radius 3 on the z axis from z = 0 to 10, joined to the ball of radius 5
//! about (0, 0, 10). The concave circle where the post's side meets the ball, of radius 3 at
//! z = 6, chamfered with two distances, each in a straight line from it: 2 down the post and 1 on
//! the ball, filling the groove.
use crate::bench::{frame, p};
use crate::shared::chamfer::chamfer_rim;
use crate::Refusal;
use kentron::blend::ChamferSize;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let post = Cylinder::new(3.0, 10.0).build(m)?;
    let ball = Sphere::new(5.0).with_frame(frame(p(0.0, 0.0, 10.0), Direction::Z)).build(m)?;
    let joined = boolean::union(m, post, ball)?;
    // (The rim, and a point on the post's side, away from the seams.)
    chamfer_rim(m, joined, p(0.0, 3.0, 6.0), p(0.0, 3.0, 2.0), ChamferSize::Distances(2.0, 1.0))
}
