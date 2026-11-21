//! A face gear's 30 teeth cut by a worm turning in mesh with it: from the ring about the z axis
//! between radii 40 and 52, z from −6 to 2, everything the worm's thread (about the line through
//! (0, 30, 10) along x, lead 2π, a rack of module 2 in section) passes through as it turns by t
//! about its axis and the gear by t/30 about the z axis, t from 0 to 60π, seen from the gear.
//!
//! Refused. kentron's cut by a motion (`sweep::cut_by_motion`) takes the motion as steps, the worm's
//! turn and the gear's taken back, but cuts only a prism moved in its plane, or a solid of
//! revolution screwed or turned about a single axis: the thread is no solid of revolution, and
//! two turns about skew axes make no plane problem. Seen from the gear the thread is a rack slid
//! along its axis while the gear turns, so the teeth's flanks are the envelope of the rack's
//! flanks under that rolling, which kentron has no operation to find.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron's cut by a motion takes a solid of revolution screwed or turned about one axis, or a \
         prism moved in its plane; the worm's thread is neither, turned in mesh about two skew axes, \
         and the teeth's flanks are an envelope kentron has no operation to find",
    ))
}
