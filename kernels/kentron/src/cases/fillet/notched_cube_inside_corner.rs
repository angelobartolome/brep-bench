//! A cube from the origin to (20, 20, 20) with the cube from (10, 10, 10) to (20, 20, 20) cut
//! from its corner. The notch's three concave edges, meeting at (10, 10, 10), rounded together
//! to radius 3, the inside corner filled by an eighth of a ball about (13, 13, 13).
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::boolean;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(20.0, 20.0, 20.0)).build(m)?;
    let notch = Cuboid::from_corners(p(10.0, 10.0, 10.0), p(20.0, 20.0, 20.0)).build(m)?;
    let notched = boolean::difference(m, cube, notch)?;
    let mut inside = Vec::new();
    for q in [p(15.0, 10.0, 10.0), p(10.0, 15.0, 10.0), p(10.0, 10.0, 15.0)] {
        inside.push(edge_through(m, notched, q)?);
    }
    Ok(blend::fillet(m, notched, &inside, 3.0)?)
}
