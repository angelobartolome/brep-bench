//! A prism from z = 0 to z = 6 on the pentagon (0, 0), (8, 0), (8, 5), (4, 5 + 4 tan 0.5°),
//! (0, 5): a block whose top is two faces meeting at 179° in a ridge along x = 4. The ridge
//! rounded to radius 180, an arc of a cylinder turning through 1°.
use crate::bench::{edge_through, p, v};
use crate::shared::fillet::prism;
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let peak = 5.0 + 4.0 * 0.5f64.to_radians().tan();
    let outline = [p(0.0, 0.0, 0.0), p(8.0, 0.0, 0.0), p(8.0, 5.0, 0.0), p(4.0, peak, 0.0), p(0.0, 5.0, 0.0)];
    let block = prism(m, &outline, v(0.0, 0.0, 6.0))?;
    let ridge = edge_through(m, block, p(4.0, peak, 3.0))?;
    Ok(blend::fillet(m, block, &[ridge], 180.0)?)
}
