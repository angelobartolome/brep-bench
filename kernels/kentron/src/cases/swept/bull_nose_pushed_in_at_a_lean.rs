//! A block, the box from (−8, −8, −8) to (8, 8, 0), bored into from its top by a leaning bull-nose
//! mill pushed straight down: about the axis through c = (0, 0, 4) along a = (sin 30°, 0, cos 30°),
//! from c to c + 3a, a cylinder of radius 2 whose end corner is rounded to 0.5, slid without
//! turning down by 8. Everything the mill passes through, cut.
//!
//! Refused. kentron has no sweep of a solid, and `sweep::cut_by_motion` takes no slide of a solid
//! of revolution. Built as a box slid is, from the mill where it starts and where it ends and its
//! faces slid (`sweep::extrude`), each face would first be split where the slide runs along it,
//! where its normal is level, so as to slide without folding. The flat end never is, and the side
//! is on two of its lines; but on the round, a quarter torus whose axis leans to the slide, that
//! is the torus's silhouette seen from above, a curve that is neither one of its circles nor a
//! plane section. kentron has no operation that finds it: the hole's wall there is an envelope.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid, and its cut by a motion takes no slide of a solid of \
         revolution; the leaning mill's round, a quarter torus, touches its sweep along its \
         silhouette from above, no circle, line or plane section, an envelope kentron has no \
         operation to find",
    ))
}
