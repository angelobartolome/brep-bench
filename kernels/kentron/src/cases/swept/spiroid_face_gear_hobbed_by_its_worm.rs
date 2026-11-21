//! A face gear's 30 teeth cut whole by a tapered single-start worm turning in mesh with it: the
//! worm turned by t about the line through (0, 30, 6) along x and then by −t/30 about the z axis,
//! for t from 0 to 60π, cut from the conical ring between radii 40 and 52.
//!
//! Refused. The worm's thread is a thread on a cone, neither a prism nor a solid of revolution,
//! and its two turns are about skew axes: kentron's cut by a motion (`sweep::cut_by_motion`)
//! takes neither, and the teeth's flanks are envelopes of the thread kentron cannot find.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid along two turns about skew axes, nor of a tapered thread: \
         `sweep::cut_by_motion` takes a prism moved in a plane or one screw or turn of a solid of \
         revolution; the flanks are envelopes of the thread kentron has no operation to find",
    ))
}
