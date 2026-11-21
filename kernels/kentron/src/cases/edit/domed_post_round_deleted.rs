//! A post: the cylinder of radius 4 on the z axis from z = 0 to z = 6, capped by the part above
//! z = 6 of the ball of radius 5 about (0, 0, 3), the rim where they meet rounded to radius 1. The
//! round is deleted: the cylinder runs on up and the dome down until they meet again in the circle
//! of radius 4 at z = 6.
use crate::bench::{edge_through, frame, p};
use crate::shared::edit::{deleted, round_middle};
use crate::Refusal;
use kentron::math::Direction;
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};
use kentron::{blend, boolean};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let post = Cylinder::new(4.0, 6.0).build(m)?;
    // The ball's zone above z = 6, 3 over its center: latitude asin(3/5) up.
    let dome = Sphere::new(5.0).with_frame(frame(p(0.0, 0.0, 3.0), Direction::Z)).with_latitudes(0.6f64.asin(), FRAC_PI_2).build(m)?;
    let part = boolean::union(m, post, dome)?;
    let rim = edge_through(m, part, p(0.0, 4.0, 6.0))?;
    let rounded = blend::fillet(m, part, &[rim], 1.0)?;
    // The round's center, 1 in from the cylinder and from the ball: radius 3, 4 from the ball's
    // center. It touches the cylinder outward and the ball away from the ball's center.
    let center = (3.0, 3.0 + 7f64.sqrt());
    let (r, z) = round_middle(center, (1.0, 0.0), (3.0 / 4.0, 7f64.sqrt() / 4.0), 1.0);
    deleted(m, rounded, &[p(0.0, r, z)])
}
