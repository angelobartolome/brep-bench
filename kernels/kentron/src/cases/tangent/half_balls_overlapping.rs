//! Two halves of the ball of radius 5 about the origin, united: the half where z ≥ 0, its poles on
//! the z axis, and the half where x ≥ 0, its poles on the x axis. Their curved faces coincide
//! where x > 0 and z > 0; the union is the ball less the quarter where x < 0 and z < 0.
use crate::bench::p;
use kentron::boolean;
use kentron::math::{Direction, Frame};
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    // each the northern half of its ball: latitudes from its equator to its pole
    let upper = Sphere::new(5.0).with_latitudes(0.0, FRAC_PI_2).build(m)?;
    let poles_on_x = Frame::new(p(0.0, 0.0, 0.0), Direction::X, Direction::Y);
    let toward_x = Sphere::new(5.0).with_frame(poles_on_x).with_latitudes(0.0, FRAC_PI_2).build(m)?;
    boolean::union(m, upper, toward_x)
}
