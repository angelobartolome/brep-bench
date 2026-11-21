//! The union of the two torus tumbles of `swept/tumbling-torus-crossed-forward` and
//! `swept/tumbling-torus-crossed-reverse`: the torus centred at (3, 0, 2), radii 1 and 0.22,
//! turned by 0.7t about the vertical line through (2, 0, 0) and then by ∓t about a second, skew
//! line, for t from −100° to 100°, each sweep over its own t.
//!
//! Refused. kentron would take the union of the two completed sweeps with its booleans, but it
//! cannot make either: each is the envelope of a torus under turns about skew axes, which no
//! sweep of kentron's carries and no operation of kentron's finds.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "neither operand can be built: kentron has no sweep of a solid along a pair of turns about \
         skew axes (`sweep::cut_by_motion` takes one screw or turn of a solid of revolution, or \
         planar motions of a prism), and each tumble's boundary is an envelope kentron cannot find",
    ))
}
