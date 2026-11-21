//! The L-block whose outline is the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and
//! (0, 15) in the xy plane, extruded 8 along +z, its inside corner along x = 5, y = 5 rounded to
//! radius 2: a quarter cylinder about the line x = 7, y = 7. The round is deleted: the faces x = 5
//! and y = 5 run back until they meet, and the result is the L-block with its corner sharp.
use crate::bench::{edge_through, p};
use crate::shared::edit::deleted;
use crate::shared::offset::l_plate;
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};
use std::f64::consts::SQRT_2;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = l_plate(m)?;
    let corner = edge_through(m, block, p(5.0, 5.0, 4.0))?;
    let rounded = blend::fillet(m, block, &[corner], 2.0)?;
    // The round halfway between the two faces, toward the corner from its axis.
    deleted(m, rounded, &[p(7.0 - SQRT_2, 7.0 - SQRT_2, 4.0)])
}
