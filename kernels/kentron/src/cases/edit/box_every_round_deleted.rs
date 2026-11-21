//! The box from the origin to (20, 16, 12) with every edge rounded to radius 2: twelve quarter
//! cylinders and eight eighths of a ball. All twenty rounds, every face of it that is not flat,
//! are deleted together: the six flat faces run on until they meet in the box's edges again.
use crate::bench::{edges, p};
use crate::shared::edit::curved_faces;
use crate::Refusal;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};
use kentron::{blend, defeature};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(20.0, 16.0, 12.0)).build(m)?;
    let all = edges(m, block);
    let rounded = blend::fillet(m, block, &all, 2.0)?;
    let rounds = curved_faces(m, rounded);
    Ok(defeature::remove_faces(m, rounded, &rounds)?.shape)
}
