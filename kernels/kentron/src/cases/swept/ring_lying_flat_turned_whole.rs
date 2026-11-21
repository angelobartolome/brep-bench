//! The solid a ring fills as it turns a whole turn about the z axis: the torus whose axis is the
//! vertical line through (8, 0), the circle through its tube's centres of radius 3, its tube of
//! radius 1, centred on the plane z = 0.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) gives a blank less what the tool passes
//! through, and its whole turn of this torus fails (the section's boundary does not close), so
//! the solid is built as elementary geometry gives it:
//! each point of the ring turns along its circle about the z axis, so what it passes through is
//! the ring's shadow on a half-plane through the axis, along those circles, revolved whole. At
//! height z the ring spans 8 ± (3 + √(1 − z²)) from the axis, so the shadow is the stadium: the band
//! |z| ≤ 1 between 5 and 11 out, ended by half discs of radius 1.
use crate::shared::parts::xz_plane;
use kentron::math::Axis;
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Arc, Start, To};
    let stadium = [Start(5.0, -1.0), To(11.0, -1.0), Arc { radius: 1.0, angle: 180.0 }, To(5.0, 1.0), Arc { radius: 1.0, angle: 180.0 }];
    let section = sketch::profile(m, xz_plane(0.0), &stadium, Outcome::Face)?;
    Ok(sweep::revolve(m, section, Axis::Z, TAU)?.shape)
}
