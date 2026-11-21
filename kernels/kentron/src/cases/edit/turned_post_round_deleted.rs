//! A turned post on the z axis: the cylinder of radius 5 from z = 0 to z = 6 and on it the cone
//! frustum from radius 5 at z = 6 to radius 2 at z = 10, made one solid, its shoulder (the circle
//! of radius 5 at z = 6) rounded to radius 1. The round is deleted: the cylinder and the cone meet
//! in the shoulder again.
use crate::bench::{edge_through, frame, p};
use crate::shared::edit::{corner_round_middle, deleted};
use crate::Refusal;
use kentron::math::Direction;
use kentron::primitives::{Cone, Cylinder};
use kentron::topology::{Model, Shape};
use kentron::{blend, boolean};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let post = Cylinder::new(5.0, 6.0).build(m)?;
    let cone = Cone::new(5.0, 2.0, 4.0).with_frame(frame(p(0.0, 0.0, 6.0), Direction::Z)).build(m)?;
    let part = boolean::union(m, post, cone)?;
    let shoulder = edge_through(m, part, p(0.0, 5.0, 6.0))?;
    let rounded = blend::fillet(m, part, &[shoulder], 1.0)?;
    // The cylinder's outward normal, and the cone's: it narrows 3 as it rises 4.
    let (r, z) = corner_round_middle((5.0, 6.0), (1.0, 0.0), (0.8, 0.6), 1.0);
    deleted(m, rounded, &[p(0.0, r, z)])
}
