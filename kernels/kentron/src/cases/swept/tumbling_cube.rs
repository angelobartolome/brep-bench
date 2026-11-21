//! The solid a cube fills as it tumbles: the box from (2.4, −0.6, 1.4) to (3.6, 0.6, 2.6), turned
//! by t/4 about the vertical line through (2, 0, 0) and then by −t about the line through
//! (0, 0.5, 0.5) parallel to the x axis, for t from −75° to 75°.
//!
//! Refused. kentron's cut by a motion (`sweep::cut_by_motion`) moves a prism only in the plane
//! square to its edges, and these turns are about skew axes, square to no common plane. Every
//! other sweep takes no solid, and the faces' and edges' envelopes under the tumble (where its
//! velocity runs along a face, on a line that moves over the face) are no curves kentron finds.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid along a pair of turns about skew axes: `sweep::cut_by_motion` \
         moves a prism only in the plane square to its edges; the cube's faces' and edges' envelopes \
         under the tumble are curves kentron has no operation to find",
    ))
}
