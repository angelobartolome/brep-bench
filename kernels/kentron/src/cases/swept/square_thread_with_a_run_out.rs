//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a thread cut
//! partway up it by a bar, the box 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1, carried along a screw about
//! the z axis rising 6 a turn, from a turn of −90° (wholly below the rod) to 900°, where it points
//! along −x, its tip at (−4, 0, 15), and then slid along −x by 3, out of its groove. Everything the
//! bar passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) screws only solids of revolution, and only
//! clean through the rod, so the groove is built as elementary geometry gives it. The run-out
//! slides the bar along its own length, outward: every place it takes then lies, within the rod,
//! inside the bar where the screw stopped, which the screw passes through anyway. So the groove
//! is the screw's: the bar's two faces it carries forward, each screwed from where the bar starts
//! to where it stops (`shared::swept::screwed_square_bar`), cut from the rod in turn.
use crate::shared::swept::{cut_each, screwed_square_bar};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    let pieces = screwed_square_bar(m, 6.0, (-90f64).to_radians(), 900f64.to_radians())?;
    cut_each(m, rod, &pieces)
}
