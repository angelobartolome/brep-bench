//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, notched by a drum swung partway into it: the solid cylinder of
//! radius 1 about the vertical line through (10, 0), |z| ≤ 0.5, turned rigidly about the z axis
//! from −140° to +30°. Everything the drum passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes a turn whose tool ends clear of the
//! blank, and this one stops inside it, so the sweep is built as elementary geometry gives it: at
//! each height the drum is a disc about (10, 0), which meets each circle about the z axis in an
//! arc centred on its own centre's polar angle; turned, that arc runs from where it starts to
//! where it ends. So the drum passes through its section on the half-plane through its axis (9 to
//! 11 out, |z| ≤ 0.5) revolved from −140° to 30°, and itself where it starts (clear of the block)
//! and where it ends.
use crate::bench::{frame, p};
use crate::shared::primitives::rectangle_xz;
use crate::shared::swept::cut_each;
use kentron::math::{Axis, Direction, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::Cylinder;
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_3;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let z = Axis::new(p(0.0, 0.0, 0.0), Direction::Z);
    let section = rectangle_xz(m, (8.0, -1.0), (12.0, 1.0))?;
    let block = sweep::revolve(m, section, z, FRAC_PI_3)?.shape;
    let drum = Cylinder::new(1.0, 1.0).with_frame(frame(p(10.0, 0.0, -0.5), Direction::Z)).build(m)?;
    let end = modify::transform(m, drum, &Transform::rotation(z, 30f64.to_radians()), TransformOptions::default())?;
    let face = rectangle_xz(m, (9.0, -0.5), (11.0, 0.5))?;
    let face = modify::transform(m, face, &Transform::rotation(z, (-140f64).to_radians()), TransformOptions::default())?;
    let notch = sweep::revolve(m, face, z, 170f64.to_radians())?.shape;
    cut_each(m, block, &[notch, end])
}
