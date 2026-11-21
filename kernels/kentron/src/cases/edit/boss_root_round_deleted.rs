//! A plate, the cylinder of radius 8 on the z axis from z = −4 to z = 0, with a boss on it, the
//! cylinder of radius 3 from z = 0 to z = 8, the concave edge at the boss's foot rounded to radius
//! 1: a quarter torus about the circle of radius 4 at z = 1. The round is deleted: the plate's top
//! runs in and the boss's side down until they meet in the circle of radius 3 at z = 0.
use crate::bench::{edge_through, frame, p};
use crate::shared::edit::{deleted, round_middle};
use crate::Refusal;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};
use kentron::{blend, boolean};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = Cylinder::new(8.0, 4.0).with_frame(frame(p(0.0, 0.0, -4.0), Direction::Z)).build(m)?;
    let boss = Cylinder::new(3.0, 8.0).build(m)?;
    let part = boolean::union(m, plate, boss)?;
    let foot = edge_through(m, part, p(0.0, 3.0, 0.0))?;
    let rounded = blend::fillet(m, part, &[foot], 1.0)?;
    // The round touches the plate's top below its center and the boss's side toward the axis.
    let (r, z) = round_middle((4.0, 1.0), (0.0, -1.0), (-1.0, 0.0), 1.0);
    deleted(m, rounded, &[p(0.0, r, z)])
}
