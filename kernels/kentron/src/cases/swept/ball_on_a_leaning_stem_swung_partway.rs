//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, notched by a ball on a leaning stem swung partway into it: the
//! ball of radius 0.8 about (10, 0, 0) and the solid cylinder of radius 0.4 from its centre 3 along
//! (0, sin 20°, cos 20°), turned rigidly about the z axis from −140° to +30°. Everything the tool
//! passes through, cut.
//!
//! Refused. The tool, ball and stem, is a solid of revolution about the stem's axis, but
//! `sweep::cut_by_motion` turns one only clear past a ring sector, and this one stops inside the
//! block. Its ball's sweep is elementary (the
//! disc of its section through the z axis revolved, with the ball at each end), but the stem
//! leans across the turn, its axis skew to the z axis: on its side at angle θ about its axis, with
//! normal n, the turn runs along it where (c + t a)·(n × ẑ) = 0, a curve t(θ) that is neither a
//! line of the cylinder nor a circle nor a plane section. Its side would have to be split along
//! that curve before each part could be revolved without folding, and kentron has no operation
//! that finds it: the stem's share of the notch is an envelope.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron's cut by a motion turns a solid of revolution only clear past a ring sector, and this \
         tool stops in the block; its leaning stem touches its sweep along curves (where \
         its normal is square to the turn) that are no lines, circles or plane sections, an envelope \
         kentron has no operation to find",
    ))
}
