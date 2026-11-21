//! A small cube of side 0.0015 from the origin, and a copy of it raised 0.000015 along z, a
//! hundredth of its side, its sides in the cube's own planes. Their union: the box from the origin
//! to (0.0015, 0.0015, 0.001515).
use crate::bench::v;
use crate::shared::tangent::small_cube_and_copy;
use kentron::boolean;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (cube, raised) = small_cube_and_copy(m, v(0.0, 0.0, 0.000015))?;
    boolean::union(m, cube, raised)
}
