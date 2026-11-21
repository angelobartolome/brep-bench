//! The plate, the cylinder of radius 8 about the z axis from z = −2 to z = 0, and on it the ball of
//! radius 3 about (0, 0, 1), joined, the concave edge where they meet filleted to radius 1.5; grown by
//! 3: every point within 3 of it. The fillet grown past its radius drops, and the grown top and
//! ball meet in a sharp concave edge.
use crate::Refusal;
use crate::bench::{frame, p};
use crate::shared::offset::on_plate;
use kentron::math::Direction;
use kentron::offset::{self, Join};
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let ball = Sphere::new(3.0).with_frame(frame(p(0.0, 0.0, 1.0), Direction::Z)).build(m)?;
    // the ball meets the plate's top in the circle of radius √8
    let solid = on_plate(m, 2.0, ball, 8f64.sqrt(), 1.5)?;
    Ok(offset::shape(m, solid, 3.0, Join::Arc)?)
}
