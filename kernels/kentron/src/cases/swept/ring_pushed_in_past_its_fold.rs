//! A block, the box from (−8, −8, −8) to (8, 8, 0), with a hole bored by a ring pushed straight
//! down 8 into it at a lean past its fold: the torus centred at (0, 0, 4), its axis
//! (sin 65°, 0, cos 65°), radii 3 and 1.
//!
//! Refused, as `swept/ring-pushed-in-at-a-lean` is. kentron slides faces (`sweep::extrude`) but no
//! solid, and its cut by a motion (`sweep::cut_by_motion`) takes no slide alone. Built from faces,
//! the torus would have to be split where its normal is level, curves on it, none of its circles
//! and here cusped, that kentron has no operation to find.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid along a slide: `sweep::cut_by_motion` takes no slide alone, and \
         sliding the torus's faces needs them split where its normal is level, which leaning are \
         cusped curves on the torus (none of its circles) kentron has no operation to find",
    ))
}
