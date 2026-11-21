//! A plate, the disk of radius 8 about the z axis from z = −2 to z = 0, joined to the ball of
//! radius 3 about (0, 0, 1), the concave edge where they meet, the circle of radius √8 at z = 0,
//! rounded to radius 1: a torus about the circle of radius 4 at z = 1. The round is deleted: the
//! plate's top runs on inward and the ball down until they meet again.
use crate::bench::{edge_through, frame, p};
use crate::shared::edit::{deleted, round_middle};
use crate::Refusal;
use kentron::math::Direction;
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};
use kentron::{blend, boolean};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = Cylinder::new(8.0, 2.0).with_frame(frame(p(0.0, 0.0, -2.0), Direction::Z)).build(m)?;
    let ball = Sphere::new(3.0).with_frame(frame(p(0.0, 0.0, 1.0), Direction::Z)).build(m)?;
    let part = boolean::union(m, plate, ball)?;
    let root = edge_through(m, part, p(0.0, 8f64.sqrt(), 0.0))?;
    let rounded = blend::fillet(m, part, &[root], 1.0)?;
    // The round touches the top straight below its center and the ball toward the ball's center.
    let (r, z) = round_middle((4.0, 1.0), (0.0, -1.0), (-1.0, 0.0), 1.0);
    deleted(m, rounded, &[p(0.0, r, z)])
}
