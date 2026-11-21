//! A cone frustum on the z axis from the disk of radius 6 at z = 0 to the disk of radius 3 at
//! z = 6, its top rim rounded to radius 1. The round is deleted: the top runs on outward and the
//! cone up until they meet in the circle of radius 3 at z = 6.
use crate::bench::{edge_through, p};
use crate::shared::edit::{corner_round_middle, deleted};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cone;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let frustum = Cone::new(6.0, 3.0, 6.0).build(m)?;
    let rim = edge_through(m, frustum, p(0.0, 3.0, 6.0))?;
    let rounded = blend::fillet(m, frustum, &[rim], 1.0)?;
    // The cone 2r + z = 12, its outward normal (2, 1)/√5, meets the top, its normal up.
    let (r, z) = corner_round_middle((3.0, 6.0), (2.0 / 5f64.sqrt(), 1.0 / 5f64.sqrt()), (0.0, 1.0), 1.0);
    deleted(m, rounded, &[p(0.0, r, z)])
}
