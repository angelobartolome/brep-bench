//! The block under the bicubic Bézier dome of `nurbs/bezier-dome`, on the square from the origin
//! to (20, 20), the edge where the patch meets the side y = 0 rounded to radius 1: a rolling-ball
//! round between the freeform top and the flat side, trimmed by the sides x = 0 and x = 20.
use crate::bench::{edge_through, p};
use crate::shared::nurbs::bezier_dome;
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = bezier_dome(m)?;
    // The patch's edge v = 0 at its middle, u = 1/2: the heights 4, 6, 5, 3 weighted 1, 3, 3, 1
    // over 8.
    let edge = edge_through(m, block, p(10.0, 0.0, 5.0))?;
    Ok(blend::fillet(m, block, &[edge], 1.0)?)
}
