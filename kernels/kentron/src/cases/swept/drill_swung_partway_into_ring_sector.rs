//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, notched by a drill swung partway into it: about the vertical
//! line through (10, 0), a cylinder of radius 1 up to z = 3 ended below by a cone of 118° whose
//! point is (10, 0, −0.8), turned rigidly about the z axis from −140° to +30°. Everything the
//! drill passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes a turn whose tool ends clear of the
//! blank, and this one stops inside it, so the sweep is built as elementary geometry gives it: at
//! each height the drill is a disc about its axis, which meets each circle about the z axis in an
//! arc centred on the axis's polar angle; turned, that arc runs from where it starts to where it
//! ends. So the drill passes through its section on the plane through its axis and the z axis
//! revolved from −140° to 30°, and itself where it starts (clear of the block) and where it ends.
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
    use Step::{Start, To};
    let z = Axis::new(p(0.0, 0.0, 0.0), Direction::Z);
    let section = rectangle_xz(m, (8.0, -1.0), (12.0, 1.0))?;
    let block = sweep::revolve(m, section, z, FRAC_PI_3)?.shape;
    // Where the cone meets the cylinder.
    let rim = -0.8 + 1.0 / 59f64.to_radians().tan();
    // The drill: its half section, out from its axis, turned whole about it.
    let half = [Start(10.0, -0.8), To(11.0, rim), To(11.0, 3.0), To(10.0, 3.0)];
    let half = sketch::profile(m, xz_plane(0.0), &half, Outcome::Face)?;
    let drill = sweep::revolve(m, half, Axis::new(p(10.0, 0.0, 0.0), Direction::Z), TAU)?.shape;
    let end = modify::transform(m, drill, &Transform::rotation(z, 30f64.to_radians()), TransformOptions::default())?;
    // Its whole section through its axis, from where it starts.
    let whole = [Start(9.0, 3.0), To(9.0, rim), To(10.0, -0.8), To(11.0, rim), To(11.0, 3.0)];
    let whole = sketch::profile(m, xz_plane(0.0), &whole, Outcome::Face)?;
    let whole = modify::transform(m, whole, &Transform::rotation(z, (-140f64).to_radians()), TransformOptions::default())?;
    let notch = sweep::revolve(m, whole, z, 170f64.to_radians())?.shape;
    cut_each(m, block, &[notch, end])
}
