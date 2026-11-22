//! A box from the origin to (20, 10, 6), the four edges round its top chamfered 1 across the top
//! and 2 down the sides.
use anvil_brep::Solid;

use crate::shared::{block, chamfer_distances};

pub fn build() -> Result<Solid, String> {
    let mut b = block([0., 0., 0.], [20., 10., 6.])?;
    for edge in [[10., 0., 6.], [20., 5., 6.], [10., 10., 6.], [0., 5., 6.]] {
        b = chamfer_distances(&b, edge, [10., 5., 6.], 1., 2.)?;
    }
    Ok(b)
}
