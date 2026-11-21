//! A small cylinder of radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy of
//! it raised 0.000015 along z, a hundredth of its height, its side in the cylinder's own side.
//! The cylinder less the copy: the disc from z = 0 to z = 0.000015.
use crate::bench::v;
use crate::shared::tangent::small_cylinder_and_copy;
use kentron::boolean;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (cylinder, raised) = small_cylinder_and_copy(m, v(0.0, 0.0, 0.000015))?;
    boolean::difference(m, cylinder, raised)
}
