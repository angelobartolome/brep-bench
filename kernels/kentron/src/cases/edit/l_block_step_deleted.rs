//! The L-block whose outline is the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and
//! (0, 15) in the xy plane, extruded 8 along +z. Its step, the riser x = 5 and the tread y = 5, is
//! deleted: the end y = 15 and the side x = 20 run on until they meet along x = 20, y = 15, and
//! the result is the box from the origin to (20, 15, 8).
use crate::bench::p;
use crate::shared::edit::deleted;
use crate::shared::offset::l_plate;
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = l_plate(m)?;
    deleted(m, block, &[p(5.0, 10.0, 4.0), p(12.0, 5.0, 4.0)])
}
