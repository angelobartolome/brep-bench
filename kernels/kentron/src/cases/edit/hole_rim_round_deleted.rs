//! The box from the origin to (20, 10, 10), drilled through by a hole of radius 2 about the
//! vertical line x = 10, y = 5, the hole's rim in the top rounded to radius 1: a quarter torus
//! about the circle of radius 3 at z = 9. The round is deleted: the top runs on inward and the wall
//! up until they meet in the circle of radius 2 at z = 10.
use crate::bench::{edge_through, p};
use crate::shared::edit::{deleted, drilled_block};
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_1_SQRT_2 as S;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = drilled_block(m, [20.0, 10.0, 10.0], 10.0, 5.0, 2.0)?;
    let rim = edge_through(m, block, p(10.0, 7.0, 10.0))?;
    let rounded = blend::fillet(m, block, &[rim], 1.0)?;
    // The round halfway between the top and the wall, 3 − 1/√2 from the axis.
    deleted(m, rounded, &[p(10.0, 8.0 - S, 9.0 + S)])
}
