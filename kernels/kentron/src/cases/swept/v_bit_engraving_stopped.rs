//! A block, the box from (0, −10, −10) to (30, 10, 0), engraved by a V bit that stops inside it at
//! both ends: the solid cone about the vertical line through (5, 0), its point at (5, 0, −2),
//! opening upward at 90° to a flat top of radius 5 at z = 3, slid without turning along the x axis
//! by 15. Everything the bit passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) slides a prism, not a solid of revolution, so the
//! sweep is built as elementary geometry gives it: the cone is convex and symmetric about the
//! plane x = 5 through its axis, so each line along the slide meets it in an interval about that
//! plane, and the slide carries the interval through itself, then on through the plane's section
//! slid 15. So the bit passes through itself where it starts and where it ends and its axial
//! section in x = 5, the triangle from its point to its top's diameter, slid 15 along x.
use crate::bench::{frame, p, v};
use crate::shared::chamfer::prism;
use crate::shared::swept::cut_each;
use kentron::math::Direction;
use kentron::primitives::{Cone, Cuboid};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(0.0, -10.0, -10.0), p(30.0, 10.0, 0.0)).build(m)?;
    let start = Cone::new(0.0, 5.0, 5.0).with_frame(frame(p(5.0, 0.0, -2.0), Direction::Z)).build(m)?;
    let end = Cone::new(0.0, 5.0, 5.0).with_frame(frame(p(20.0, 0.0, -2.0), Direction::Z)).build(m)?;
    let section = [p(5.0, 0.0, -2.0), p(5.0, 5.0, 3.0), p(5.0, -5.0, 3.0)];
    let between = prism(m, &section, v(15.0, 0.0, 0.0))?;
    cut_each(m, block, &[between, start, end])
}
