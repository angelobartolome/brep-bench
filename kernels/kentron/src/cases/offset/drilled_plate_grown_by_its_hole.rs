//! The plate from (−10, −10, 0) to (10, 10, 4) with a hole of radius 3 drilled through it along the
//! z axis, grown by 3: every point within 3 of it, exactly the hole's radius: the hole closes onto
//! the axis, and over each of its ends the grown face dips in a horn torus about the rim, down to a
//! cusp on the axis.
use crate::shared::offset::drilled_plate;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = drilled_plate(m)?;
    offset::shape(m, plate, 3.0, Join::Arc)
}
