//! The dumbbell plate, two 10 × 10 squares joined by a neck 4 long and 2 wide in the xy plane,
//! extruded 6 along +z, shrunk by 1.5: every point at least 1.5 from its outside. The neck vanishes,
//! leaving two bodies, both kept.
use crate::Refusal;
use crate::shared::shell::plate;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let outline = [
        (0.0, 0.0), (10.0, 0.0), (10.0, 4.0), (14.0, 4.0), (14.0, 0.0), (24.0, 0.0),
        (24.0, 10.0), (14.0, 10.0), (14.0, 6.0), (10.0, 6.0), (10.0, 10.0), (0.0, 10.0),
    ];
    let dumbbell = plate(m, &outline, 6.0)?;
    Ok(offset::shape(m, dumbbell, -1.5, Join::Arc)?)
}
