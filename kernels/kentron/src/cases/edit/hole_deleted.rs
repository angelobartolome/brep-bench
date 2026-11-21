//! The box from the origin to (20, 10, 10), drilled right through along z by a hole of radius 3
//! about the vertical line through (10, 5). The hole's wall is deleted: the top and the bottom
//! close over its ends, and the result is the box.
use crate::bench::p;
use crate::shared::edit::{deleted, drilled_block};
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = drilled_block(m, [20.0, 10.0, 10.0], 10.0, 5.0, 3.0)?;
    deleted(m, block, &[p(13.0, 5.0, 5.0)])
}
