//! The solid a torus fills as it tumbles: the torus centred at (3, 0, 2), axis parallel to the
//! z axis, radii 1 and 0.16, turned by 0.375t about the vertical line through (2, 0, 0) and then
//! by −t about the line through (0, h cos 30°, h sin 30°), h = 0.7071, along
//! (cos 20°, sin 20° cos 30°, sin 20° sin 30°), for t from −90° to 90°.
//!
//! Refused, as `swept/tumbling-torus` is: the two turns are about skew axes, so the torus's
//! axis of turning changes from instant to instant, and the boundary is the envelope of the
//! torus under that motion, which kentron has no operation to find.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid along a pair of turns about skew axes: `sweep::cut_by_motion` \
         takes one screw or turn of a solid of revolution, or planar motions of a prism; the boundary \
         is the envelope of the tumbling torus, which kentron has no operation to find",
    ))
}
