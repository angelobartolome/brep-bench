//! The box from the origin to (20, 10, 10), drilled through by a hole of radius 2 about the
//! vertical line x = 10, y = 5, both the hole's rims rounded to radius 1: quarter tori about the
//! circles of radius 3 at z = 9 and z = 1. Both rounds are deleted together: the top, the bottom
//! and the wall run on until they meet in the circles of radius 2 at z = 10 and z = 0.
use crate::bench::{edge_through, p};
use crate::shared::edit::{deleted, drilled_block};
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_1_SQRT_2 as S;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = drilled_block(m, [20.0, 10.0, 10.0], 10.0, 5.0, 2.0)?;
    let top = edge_through(m, block, p(10.0, 7.0, 10.0))?;
    let bottom = edge_through(m, block, p(10.0, 7.0, 0.0))?;
    let rounded = blend::fillet(m, block, &[top, bottom], 1.0)?;
    // Each round halfway between its face and the wall, 3 − 1/√2 from the axis.
    deleted(m, rounded, &[p(10.0, 8.0 - S, 9.0 + S), p(10.0, 8.0 - S, 1.0 - S)])
}
