//! A block from (0, 0, 0) to (10, 10, 5) with its vertical edge at x = 10, y = 10 broken by a flat
//! 0.0001√2 wide: the prism on the pentagon (0, 0), (10, 0), (10, 9.9999), (9.9999, 10), (0, 10).
//! The two edges where the flat meets the sides rounded together to radius 1: the round runs from
//! side to side and the flat is gone, a quarter cylinder about the line x = 9, y = 9.
use crate::bench::{edge_through, p, v};
use crate::shared::fillet::prism;
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let outline = [p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 9.9999, 0.0), p(9.9999, 10.0, 0.0), p(0.0, 10.0, 0.0)];
    let block = prism(m, &outline, v(0.0, 0.0, 5.0))?;
    let one = edge_through(m, block, p(10.0, 9.9999, 2.5))?;
    let other = edge_through(m, block, p(9.9999, 10.0, 2.5))?;
    Ok(blend::fillet(m, block, &[one, other], 1.0)?)
}
