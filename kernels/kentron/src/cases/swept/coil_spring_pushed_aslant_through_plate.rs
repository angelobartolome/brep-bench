//! A plate, the box from (−15, −15, 0) to (15, 15, 4), with a hole punched by a coil spring pushed
//! aslant through it: everything a ball of radius 0.4 passes through as its centre runs three
//! turns of the helix of radius 2 about the z axis from (2, 0, −12), rising 1.5 a turn, slid
//! without turning along (3, 1, 18). Everything the spring passes through, cut.
//!
//! Refused. kentron has no sweep of a solid. The spring slid is the ball swept over the helix slid
//! along the push, a ruled strip, so its wall is that strip's offset by the ball's radius; or, read
//! as the spring slid, its tube's silhouette along the push, the curves on its pipe surface where
//! the normal is square to the push, swept. Neither is a surface kentron makes, nor a face of the
//! spring that could be split along plane sections and slid whole: it is an envelope, which kentron
//! has no operation to find.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid, and the spring's sweep along the push is bounded by its \
         tube's silhouettes slid (an offset of a ruled strip), an envelope kentron has no operation \
         to find",
    ))
}
