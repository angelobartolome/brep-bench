//! A ball of radius 5 about the origin with its top cut flat above z = 3, leaving a disc of radius
//! 4. The flat is deleted: the ball closes over it, and the result is the plain ball.
use crate::bench::p;
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::primitives::{Cuboid, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let ball = Sphere::new(5.0).build(m)?;
    let above = Cuboid::from_corners(p(-6.0, -6.0, 3.0), p(6.0, 6.0, 6.0)).build(m)?;
    let flattened = boolean::difference(m, ball, above)?;
    deleted(m, flattened, &[p(0.0, 0.0, 3.0)])
}
