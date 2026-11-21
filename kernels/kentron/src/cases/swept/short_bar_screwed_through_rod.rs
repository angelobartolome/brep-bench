//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a thread cut by a
//! short bar, the box 4 ≤ x ≤ 12, −1.5 ≤ y ≤ 1.5, −1 ≤ z ≤ 1, carried along a screw about the z
//! axis rising 6 a turn, from a turn of −90° (wholly below the rod) to 1350° (wholly above it).
//! Everything the bar passes through, cut; its corners, inside the rod, carry the groove's walls
//! with its edges, which the outline's turn sweeps alike.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) screws only solids of revolution, so the
//! groove is built as elementary geometry gives it: screwed into itself, it is its section at
//! z = 0 screwed up the rod, and that section is what the bar's outline sweeps turning about the
//! z axis while the bar holds z = 0 (`shared::swept::rod_screw_cut_by_bar`).
use crate::shared::swept::rod_screw_cut_by_bar;
use crate::Refusal;
use kentron::math::Point2;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let outline = vec![Point2::new(4.0, -1.5), Point2::new(12.0, -1.5), Point2::new(12.0, 1.5), Point2::new(4.0, 1.5)];
    rod_screw_cut_by_bar(m, 5.0, 20.0, outline, 2.0, 6.0)
}
