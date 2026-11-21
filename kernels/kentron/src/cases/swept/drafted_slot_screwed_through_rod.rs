//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, threaded by a drafted bar
//! carried along a screw: its section in the plane x = 12 the slot of two half discs of radius 1
//! about (12, ±1.5, 0) and the rectangle between, carried along −x to x = 4 and drafted 3° inward,
//! screwed about the z axis, right-handed, rising 6 a turn, from a turn of −90° to +1350°.
//! Everything the bar passes through, cut.
//!
//! Refused. `sweep::cut_by_motion` screws only a solid of revolution, and this bar is none. Built
//! as a box slid is, from its faces each screwed (`sweep::screw_extrude`), each face would first
//! be split where the screw's velocity runs along it, so as to sweep without folding. On its two
//! flat sides that is a line, but on its two half cones, whose axes lie along x, it is where
//! n·(ẑ × r) + (6/2π) n_z = 0, a curve on each cone that is neither one of its lines nor a circle
//! nor a plane section. kentron has no operation that finds it: the groove's wall there is an
//! envelope.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron's cut by a motion screws only solids of revolution, and this drafted bar is none; its \
         half cones touch their sweep along curves (where the normal is square to the screw) that \
         are no lines, circles or plane sections, an envelope kentron has no operation to find",
    ))
}
