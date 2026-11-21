//! The solid a ring fills as it slides across, square to its own axis, farther than the width of
//! its hole: the torus about the z axis centred on the origin, radii 6 and 2, slid without turning
//! along the x axis by 15.
//!
//! kentron has no sweep of a solid (`sweep::extrude` takes faces, wires, edges, vertices and
//! compounds of them), so the sweep is built as elementary geometry gives it. At height z the
//! ring is the annulus between radii 6 ∓ w, w = √(4 − z²), and slid by 15, more than its hole's
//! width, it covers the stadium within 6 + w of the segment from (0, 0) to (15, 0): what the
//! filled ring, everything within 2 of the disc of radius 6, covers slid
//! (`shared::swept::slid_filled_ring`).
use crate::shared::swept::slid_filled_ring;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    slid_filled_ring(m, 6.0, 2.0, 15.0)
}
