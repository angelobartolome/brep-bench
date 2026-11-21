//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, slotted by a pin swung through it: the solid cylinder of radius
//! 0.5 about the x axis from x = 6 to x = 11, turned rigidly about the z axis from −140° to +140°.
//! Everything the pin passes through, cut.
//!
//! The natural construction, kentron's cut by a motion (`sweep::cut_by_motion`) of the pin as the
//! solid of revolution of its rectangle under the turn, fails here: the pin's axis meets the
//! turn's, so its side and its end are characteristic along whole lines, and the section's
//! boundary does not close. So the slot is built as elementary geometry gives it. The pin starts
//! and ends clear of the block, so over the block each circle about the z axis through a point
//! the pin covers is covered whole; and the pin's point (x, y, z) lies at ρ² = x² + y² from the
//! z axis, with 6 ≤ x ≤ 11 and y² + z² ≤ 0.25. So its meridian section is |z| ≤ 0.5, ρ ≥ 6,
//! ρ² + z² ≤ 11² + 0.5²: the slab |z| ≤ 0.5 beyond radius 6 within the ball of radius
//! √121.25 about the origin, here revolved a little past the block's sides.
use crate::bench::p;
use crate::shared::primitives::rectangle_xz;
use kentron::boolean;
use kentron::math::{Axis, Direction, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::Sphere;
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_3;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let z = Axis::new(p(0.0, 0.0, 0.0), Direction::Z);
    let section = rectangle_xz(m, (8.0, -1.0), (12.0, 1.0))?;
    let block = sweep::revolve(m, section, z, FRAC_PI_3)?.shape;
    let slab = rectangle_xz(m, (6.0, -0.5), (12.0, 0.5))?;
    let slab = modify::transform(m, slab, &Transform::rotation(z, (-10f64).to_radians()), TransformOptions::default())?;
    let slab = sweep::revolve(m, slab, z, 80f64.to_radians())?.shape;
    let ball = Sphere::new(121.25f64.sqrt()).build(m)?;
    let slot = boolean::intersection(m, slab, ball)?;
    boolean::difference(m, block, slot)
}
