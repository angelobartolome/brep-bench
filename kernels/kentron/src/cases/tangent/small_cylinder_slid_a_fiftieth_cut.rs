//! A small cylinder of radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy of
//! it slid 0.000015 along x, a fiftieth of its radius, their sides crossing in two vertical lines.
//! The cylinder less the copy: a crescent prism 0.000015 thick at its widest.
use crate::bench::v;
use crate::shared::tangent::small_cylinder_and_copy;
use kentron::boolean;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (cylinder, slid) = small_cylinder_and_copy(m, v(0.000015, 0.0, 0.0))?;
    boolean::difference(m, cylinder, slid)
}
