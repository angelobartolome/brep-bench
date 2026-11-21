//! A small cylinder of radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy of
//! it slid 0.000015 along x, a fiftieth of its radius, their sides crossing in two vertical lines.
//! The part common to both: the prism over the lens the two circles share.
use crate::bench::v;
use crate::shared::tangent::small_cylinder_and_copy;
use kentron::boolean;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (cylinder, slid) = small_cylinder_and_copy(m, v(0.000015, 0.0, 0.0))?;
    boolean::intersection(m, cylinder, slid)
}
