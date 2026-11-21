//! The solid a ball of radius 2 fills as its centre runs two right-handed turns of the helix of
//! radius 1.5 about the z axis rising 3 a turn, from (1.5, 0, 0) to (1.5, 0, 6): everything within
//! 2 of the helix's arc.
//!
//! Refused. kentron has no sweep of a solid. Elementary geometry gives a ball's sweep as the tube
//! its great disc square to the path sweeps (`sweep::pipe`, or `sweep::screw_extrude` along a
//! helix), with the ball at each end, but only where the path turns no tighter than the ball: this
//! helix turns on a radius of about 1.65, under 2, so the disc's sweep folds through itself, its
//! points beyond the centre of curvature running backwards, and the screwed disc is no solid.
//! The solid's wall is then only the part of that folded sheet the balls do not swallow, trimmed
//! where it folds, which kentron has no operation to find. And `sweep::cut_by_motion` screws a
//! solid of revolution only through a rod, cutting it rather than making the swept solid.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid, and a ball's tube along this helix, which turns on a radius \
         of 1.65, under the ball's 2, folds through itself: its screwed great disc is no solid, and \
         trimming the folded sheet back to what bounds the sweep is an envelope kentron does not find",
    ))
}
