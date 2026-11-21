//! A rod, the cylinder of radius 5 about the z axis from z = 0 to 20, threaded by the bar
//! 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1 turned by θ about the z axis while rising 6s + 0.3s²,
//! s = θ/360°, for θ from −90° to 1260°.
//!
//! Refused. The lead grows, so no screw carries the bar: kentron's cut by a motion
//! (`sweep::cut_by_motion`) and screw extrusion (`sweep::screw_extrude`) take a constant lead, and
//! a pipe would need the variable-lead path, which kentron's helices (`helix::helix`) make only
//! in parts of constant pitch, so it would be a curve fitted by hand; and the bar's faces would
//! still have to be split where the motion runs along them.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "no kentron sweep carries a solid or its faces along a turn whose rise is quadratic in it: \
         `sweep::cut_by_motion` and `sweep::screw_extrude` take a constant lead, and the variable-lead \
         path would be a curve fitted outside the kernel",
    ))
}
