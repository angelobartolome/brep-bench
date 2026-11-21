//! A plate, the box from the origin to (40, 30, 5), with a square boss joined to its top, the box
//! from (12, 7, 5) to (28, 23, 15). The boss's four upright edges rounded to radius 3, then its
//! root, now one tangent chain round it, rounded to radius 2. The boss's top stays sharp.
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::boolean;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(40.0, 30.0, 5.0)).build(m)?;
    let boss = Cuboid::from_corners(p(12.0, 7.0, 5.0), p(28.0, 23.0, 15.0)).build(m)?;
    let part = boolean::union(m, plate, boss)?;
    let mut uprights = Vec::new();
    for (x, y) in [(12.0, 7.0), (28.0, 7.0), (28.0, 23.0), (12.0, 23.0)] {
        uprights.push(edge_through(m, part, p(x, y, 10.0))?);
    }
    let part = blend::fillet(m, part, &uprights, 3.0)?;
    // One side's foot: the rounding runs on round the whole root.
    let root = edge_through(m, part, p(20.0, 7.0, 5.0))?;
    Ok(blend::fillet(m, part, &[root], 2.0)?)
}
