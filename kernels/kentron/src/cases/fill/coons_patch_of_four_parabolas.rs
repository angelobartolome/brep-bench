//! A block over the square from (0, 0) to (10, 10), capped by the bilinearly blended Coons patch
//! of four parabolic arcs (quadratic Bézier curves in the four side planes), a saddle.
//!
//! Refused: kentron has no Coons fill. Its filling (`fill::n_sided`) makes the plate of least
//! bending energy over the loop, a different surface, and its loft (`loft::loft`) joins two
//! opposite arcs without the other two as boundaries.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no Coons patch: fill::n_sided fills a loop with the surface of least bending \
         energy, not the bilinearly blended Coons patch",
    ))
}
