//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, notched by a ring lying flat swung partway into it: the torus
//! about the vertical line through (10, 0), radii 1 and 0.4, centred on the plane z = 0, turned
//! rigidly about the z axis from −140° to +30°. Everything the ring passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes a turn whose tool ends clear of the
//! blank, and this one stops inside it, so the sweep is built as elementary geometry gives it: at
//! height z the ring is the annulus about (10, 0) between radii 1 ∓ s, s = √(0.16 − z²), which
//! meets a circle about the z axis in one arc or two, either side of its hole, reaching out to
//! the polar angle ±b where the circle crosses the annulus's outer rim. Turned through 170°, wider
//! than the hole, both arcs run together into the angles from −140° − b to 30° + b. That is the
//! ring's shadow on the half-plane through the axis, the band 9 − s to 11 + s from the z axis (a
//! stadium), revolved from −140° to 30°, with the ring's hole filled where it starts (clear of the
//! block) and where it ends, which reaches ±b about it.
use crate::bench::p;
use crate::shared::parts::xz_plane;
use crate::shared::primitives::rectangle_xz;
use crate::shared::swept::cut_each;
use kentron::math::{Axis, Direction, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::{FRAC_PI_3, TAU};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Arc, Start, To};
    let z = Axis::new(p(0.0, 0.0, 0.0), Direction::Z);
    let section = rectangle_xz(m, (8.0, -1.0), (12.0, 1.0))?;
    let block = sweep::revolve(m, section, z, FRAC_PI_3)?.shape;
    // The ring with its hole filled, where it ends: its half stadium, out from its axis, turned
    // whole about it.
    let half = [Start(10.0, -0.4), To(11.0, -0.4), Arc { radius: 0.4, angle: 180.0 }, To(10.0, 0.4)];
    let half = sketch::profile(m, xz_plane(0.0), &half, Outcome::Face)?;
    let filled = sweep::revolve(m, half, Axis::new(p(10.0, 0.0, 0.0), Direction::Z), TAU)?.shape;
    let end = modify::transform(m, filled, &Transform::rotation(z, 30f64.to_radians()), TransformOptions::default())?;
    // Its shadow, the stadium from 8.6 to 11.4 out, from where it starts.
    let shadow = [Start(9.0, -0.4), To(11.0, -0.4), Arc { radius: 0.4, angle: 180.0 }, To(9.0, 0.4), Arc { radius: 0.4, angle: 180.0 }];
    let shadow = sketch::profile(m, xz_plane(0.0), &shadow, Outcome::Face)?;
    let shadow = modify::transform(m, shadow, &Transform::rotation(z, (-140f64).to_radians()), TransformOptions::default())?;
    let notch = sweep::revolve(m, shadow, z, 170f64.to_radians())?.shape;
    cut_each(m, block, &[notch, end])
}
