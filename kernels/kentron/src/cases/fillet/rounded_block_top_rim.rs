//! A block from (−10, −6, 0) to (10, 6, 5) with its four vertical edges rounded to radius 3, the
//! rim of its top, eight edges each running on tangent into the next, rounded to radius 1.
use crate::bench::{edge_through, p};
use crate::shared::draft::rounded_box;
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = rounded_box(m, p(-10.0, -6.0, 0.0), p(10.0, 6.0, 5.0), 3.0)?;
    // One of the rim's straight edges: the rounding runs on round the whole rim.
    let rim = edge_through(m, block, p(0.0, 6.0, 5.0))?;
    Ok(blend::fillet(m, block, &[rim], 1.0)?)
}
