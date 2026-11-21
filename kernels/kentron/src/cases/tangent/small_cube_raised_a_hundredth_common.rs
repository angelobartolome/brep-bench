//! A small cube of side 0.0015 from the origin, and a copy of it raised 0.000015 along z, a
//! hundredth of its side, its sides in the cube's own planes. The part common to both: the box from
//! (0, 0, 0.000015) to (0.0015, 0.0015, 0.0015).
use crate::bench::v;
use crate::shared::tangent::small_cube_and_copy;
use kentron::boolean;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (cube, raised) = small_cube_and_copy(m, v(0.0, 0.0, 0.000015))?;
    boolean::intersection(m, cube, raised)
}
