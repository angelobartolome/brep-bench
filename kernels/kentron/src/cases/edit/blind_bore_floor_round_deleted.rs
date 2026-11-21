//! The box from (−10, −10, 0) to (10, 10, 10) with a blind bore of radius 4 about the z axis sunk
//! from its top to a floor at z = 4, the concave edge where the floor meets the wall rounded to
//! radius 1: a quarter torus about the circle of radius 3 at z = 5. The round is deleted: the floor
//! runs on outward and the wall down until they meet in the circle of radius 4 at z = 4.
use crate::bench::{edge_through, frame, p};
use crate::shared::edit::{deleted, round_middle};
use crate::Refusal;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};
use kentron::{blend, boolean};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(-10.0, -10.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let bore = Cylinder::new(4.0, 7.0).with_frame(frame(p(0.0, 0.0, 4.0), Direction::Z)).build(m)?;
    let bored = boolean::difference(m, block, bore)?;
    let floor_edge = edge_through(m, bored, p(0.0, 4.0, 4.0))?;
    let rounded = blend::fillet(m, bored, &[floor_edge], 1.0)?;
    // The round touches the floor below its center and the wall outward from it.
    let (r, z) = round_middle((3.0, 5.0), (0.0, -1.0), (1.0, 0.0), 1.0);
    deleted(m, rounded, &[p(0.0, r, z)])
}
