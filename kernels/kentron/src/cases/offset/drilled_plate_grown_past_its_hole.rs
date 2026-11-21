//! The plate from (−10, −10, 0) to (10, 10, 4) with a hole of radius 3 drilled through it along the
//! z axis, grown by 4: every point within 4 of it, more than the hole's radius: the hole fills, and
//! over each of its ends the grown face dips in the inner part of a spindle torus about the rim.
use crate::shared::offset::drilled_plate;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = drilled_plate(m)?;
    offset::shape(m, plate, 4.0, Join::Arc)
}
