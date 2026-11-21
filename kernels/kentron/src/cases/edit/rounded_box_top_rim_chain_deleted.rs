//! The box from the origin to (20, 12, 8) with its four upright edges rounded to radius 3, then
//! the whole rim of its top rounded to radius 1: four quarter cylinders and four pieces of tori,
//! a closed tangent chain. The eight are deleted together: the top and the walls meet again.
use crate::bench::{edge_through, p};
use crate::shared::draft::rounded_box;
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_1_SQRT_2 as S;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = rounded_box(m, p(0.0, 0.0, 0.0), p(20.0, 12.0, 8.0), 3.0)?;
    // The corners' axes, and the way out from each, diagonally.
    let corners = [((3.0, 3.0), (-S, -S)), ((17.0, 3.0), (S, -S)), ((17.0, 9.0), (S, S)), ((3.0, 9.0), (-S, S))];
    let mut rim = Vec::new();
    for middle in [p(10.0, 0.0, 8.0), p(20.0, 6.0, 8.0), p(10.0, 12.0, 8.0), p(0.0, 6.0, 8.0)] {
        rim.push(edge_through(m, block, middle)?);
    }
    for ((x, y), (dx, dy)) in corners {
        rim.push(edge_through(m, block, p(x + 3.0 * dx, y + 3.0 * dy, 8.0))?);
    }
    let rounded = blend::fillet(m, block, &rim, 1.0)?;
    // Each round halfway between the top and its wall: 1 − 1/√2 in from the wall, 1 − 1/√2 down.
    let (inset, z) = (1.0 - S, 7.0 + S);
    let mut rounds = vec![p(10.0, inset, z), p(20.0 - inset, 6.0, z), p(10.0, 12.0 - inset, z), p(inset, 6.0, z)];
    for ((x, y), (dx, dy)) in corners {
        rounds.push(p(x + (3.0 - inset) * dx, y + (3.0 - inset) * dy, z));
    }
    deleted(m, rounded, &rounds)
}
