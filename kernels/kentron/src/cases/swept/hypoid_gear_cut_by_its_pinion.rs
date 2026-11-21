//! The straight bevel gear's blank with one tooth space cut by a pinion tooth on a shaft 12 from
//! the gear's: the tooth turned by t about the line through (0, 12, 0) along x and then by 7t/15
//! about the z axis, for t from −70° to 70°.
//!
//! Refused. The two turns are about skew axes: kentron's cut by a motion (`sweep::cut_by_motion`)
//! takes a prism moved in a plane, or one screw or turn of a solid of revolution, and the tooth
//! is neither moved so. The flanks are envelopes of planes under that motion, which kentron has
//! no operation to find.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid along two turns about skew axes: `sweep::cut_by_motion` takes \
         a prism moved in a plane or one screw or turn of a solid of revolution; the flanks are \
         envelopes of the tooth's planes kentron has no operation to find",
    ))
}
