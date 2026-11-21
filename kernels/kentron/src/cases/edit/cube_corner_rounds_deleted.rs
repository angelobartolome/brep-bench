//! The cube from the origin to (10, 10, 10), the three edges meeting at its corner (10, 10, 10)
//! rounded to radius 2: three quarter cylinders and the eighth of a ball about (8, 8, 8) between
//! them. All four, the cube's faces that are not flat, are deleted together: the cube again.
use crate::bench::{edge_through, p};
use crate::shared::edit::curved_faces;
use crate::Refusal;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};
use kentron::{blend, defeature};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let mut corner = Vec::new();
    for middle in [p(5.0, 10.0, 10.0), p(10.0, 5.0, 10.0), p(10.0, 10.0, 5.0)] {
        corner.push(edge_through(m, cube, middle)?);
    }
    let rounded = blend::fillet(m, cube, &corner, 2.0)?;
    let rounds = curved_faces(m, rounded);
    Ok(defeature::remove_faces(m, rounded, &rounds)?.shape)
}
