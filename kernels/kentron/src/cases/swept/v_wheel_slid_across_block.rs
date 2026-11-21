//! A block, the box from (0, −5, 0) to (20, 5, 10), V-grooved across its top by a wheel with a
//! sharp rim: the solid of revolution, about the line through (−8, 0, 14) along y, of the
//! triangle in the plane x = −8 with corners (y, z) = (0, 8) and (±3 tan 30°, 11), slid without
//! turning along x by 36. Everything the wheel passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes no slide alone, so the groove is built
//! as elementary geometry gives it. A point of the wheel at (x, y, z) is ρ ≥ 14 − z from its axis,
//! with (y, ρ) in the triangle, which narrows as ρ grows; so below the block's top (14 − z ≥ 4,
//! inside the triangle's span of ρ from 3 to 6) (y, z) lies in the triangle as drawn at x = −8,
//! whose points are the wheel's own. The wheel starts and ends clear of the block, so within it
//! the groove is that triangle slid along the whole slide.
use crate::bench::{p, v};
use crate::shared::sweep::polyline;
use kentron::boolean;
use kentron::make;
use kentron::primitives::Cuboid;
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::PI;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = Cuboid::from_corners(p(0.0, -5.0, 0.0), p(20.0, 5.0, 10.0)).build(m)?;
    let half = 3.0 * (PI / 6.0).tan();
    let outline = polyline(m, &[p(-8.0, 0.0, 8.0), p(-8.0, half, 11.0), p(-8.0, -half, 11.0)], true)?;
    let section = make::planar_face(m, outline)?;
    let groove = sweep::extrude(m, section, v(36.0, 0.0, 0.0))?.shape;
    boolean::difference(m, block, groove)
}
