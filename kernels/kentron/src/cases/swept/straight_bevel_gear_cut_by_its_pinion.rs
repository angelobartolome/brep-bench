//! A bevel gear blank (the 30-tooth gear's, about the z axis) with one tooth space cut by a
//! straight-sided tooth of its 15-tooth pinion, turned by t about the x axis while the gear turns
//! by −t/2 about the z axis, for t from −60° to 60°.
//!
//! Refused. Seen from the blank the tooth turns about two axes through the origin at once, a
//! spherical motion: kentron's cut by a motion (`sweep::cut_by_motion`) takes a prism moved in a
//! plane, or one screw or turn of a solid of revolution, and the space's flanks are cones over
//! spherical envelopes of the tooth's flanks, which kentron has no operation to find.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid along two turns about meeting axes (a spherical motion): \
         `sweep::cut_by_motion` takes a prism moved in a plane or one screw or turn of a solid of \
         revolution; the tooth space's flanks are spherical envelopes kentron cannot find",
    ))
}
