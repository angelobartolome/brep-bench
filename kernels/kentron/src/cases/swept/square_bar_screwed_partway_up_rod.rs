//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a thread cut
//! partway up it by a bar, the box 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1, carried along a screw about
//! the z axis rising 6 a turn, from a turn of −120° (wholly below the rod) to 600°, where it stops
//! inside the rod. Everything the bar passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) screws only solids of revolution, and only
//! clean through the rod, so the groove is built as elementary geometry gives it: the bar's two
//! faces the screw carries forward, each screwed from where the bar starts to where it stops
//! (`shared::swept::screwed_square_bar`), cut from the rod in turn.
use crate::shared::swept::{cut_each, screwed_square_bar};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    let pieces = screwed_square_bar(m, 6.0, (-120f64).to_radians(), 600f64.to_radians())?;
    cut_each(m, rod, &pieces)
}
