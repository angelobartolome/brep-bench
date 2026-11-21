//! A worm wheel's 30 teeth cut whole by its single-start worm (module 2, pitch radius 10, axis
//! through (0, 40, 0) along x) turning by t while the wheel, the disc of radius 32 from z = −6 to
//! 6, turns by t/30, for t from 0 to 60π.
//!
//! Refused. Seen from the wheel the worm is a rack slid along its axis while the wheel turns, a
//! planar motion, but of a thread, which is no prism: kentron's planar cut by a motion
//! (`sweep::cut_by_motion`) takes only a prism square to the plane, the same at every height. The
//! teeth's flanks, leaning off the mid-plane, are envelopes of the thread kentron cannot find.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron's cut by a planar motion (`sweep::cut_by_motion`) takes only a prism square to the \
         plane, and the worm's thread is no prism: its section changes with height, and the \
         teeth's flanks are envelopes of the helicoid kentron has no operation to find",
    ))
}
