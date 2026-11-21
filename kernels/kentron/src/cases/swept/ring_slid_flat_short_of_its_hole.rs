//! The solid a ring fills as it slides across, square to its own axis, by less than the width of
//! its hole: the torus about the z axis centred on the origin, radii 6 and 2, slid without turning
//! along the x axis by 6.
//!
//! kentron has no sweep of a solid (`sweep::extrude` takes faces, wires, edges, vertices and
//! compounds of them), so the sweep is built as elementary geometry gives it. At height z the
//! ring is the annulus between radii 6 ∓ w, w = √(4 − z²). Slid, it covers the stadium within
//! 6 + w of the segment from (0, 0) to (6, 0) (what the filled ring, everything within 2 of the
//! disc of radius 6, covers slid: `shared::swept::slid_filled_ring`) but for the points that stay
//! in its hole all the way: those in the hole where it starts and where it ends (the distance to
//! the moving centre being convex). The holes' common lens is the two cylinders of radius 6 in
//! common less the ring at each end; so the solid is the slid filled ring less the cylinders'
//! lens, with the ring at each end put back.
use crate::bench::{p, v};
use crate::shared::swept::slid_filled_ring;
use kentron::boolean;
use kentron::math::{Direction, Frame, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::{Cylinder, Torus};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let slide = v(6.0, 0.0, 0.0);
    let filled = slid_filled_ring(m, 6.0, 2.0, 6.0)?;
    let hole = Cylinder::new(6.0, 6.0).with_frame(Frame::new(p(0.0, 0.0, -3.0), Direction::Z, Direction::X)).build(m)?;
    let moved = modify::transform(m, hole, &Transform::translation(slide), TransformOptions::default())?;
    let lens = boolean::intersection(m, hole, moved)?;
    let mut body = boolean::difference(m, filled, lens)?;
    let start = Torus::new(6.0, 2.0).build(m)?;
    let end = modify::transform(m, start, &Transform::translation(slide), TransformOptions::default())?;
    for ring in [start, end] {
        body = boolean::union(m, body, ring)?;
    }
    Ok(body)
}
