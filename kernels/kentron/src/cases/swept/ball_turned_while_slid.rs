//! The solid a ball of radius 1 about (5, 0, 0) fills as it is turned about the z axis by 180°·t
//! and slid by (25t, 0, 0) together, t from 0 to 1: its centre runs the curtate trochoid
//! (5 cos πt + 25t, 5 sin πt, 0).
//!
//! Refused. kentron has no sweep of a solid. Elementary geometry gives a ball's sweep as the tube
//! about its centre's path with a ball at each end, and kentron pipes a disc along a spine
//! (`sweep::pipe`), but it has no trochoid among its curves, and a spine fitted through points of
//! the path would be geometry made outside the kernel. `sweep::cut_by_motion` takes a turn with a
//! slide square to it only for a prism, cut from a disc, and `sweep::screw_extrude` slides only
//! along its axis.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid, and the ball's centre runs a trochoid, a curve kentron has \
         no construction for: it could pipe the ball's great disc only along a spine fitted outside \
         the kernel; its cut by a motion rolls only a prism, cut from a disc",
    ))
}
