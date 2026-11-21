//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, notched by half a drum swung partway into it: the half with
//! y ≤ 0 of the solid cylinder of radius 1 about the vertical line through (10, 0), |z| ≤ 0.5, its
//! flat face the rectangle 9 ≤ x ≤ 11, y = 0, turned rigidly about the z axis from −140° to +30°.
//! Everything the half drum passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) turns only solids of revolution, and its
//! turn cut takes a tool that ends clear of the blank, so the sweep is built as elementary
//! geometry gives it: every point of the half drum lies between 9 and 11 from the z axis and
//! within |z| ≤ 0.5, at a polar angle at most its flat face's, so as it turns its circle about the
//! axis runs behind the flat face's. So the half drum passes through the flat face revolved from
//! −140° to 30° and itself where it starts.
use crate::bench::p;
use crate::shared::primitives::rectangle_xz;
use crate::shared::swept::cut_each;
use kentron::math::{Axis, Direction, Frame, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::Cylinder;
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::{FRAC_PI_3, PI};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let z = Axis::new(p(0.0, 0.0, 0.0), Direction::Z);
    let section = rectangle_xz(m, (8.0, -1.0), (12.0, 1.0))?;
    let block = sweep::revolve(m, section, z, FRAC_PI_3)?.shape;
    // The half drum as placed: the half of the cylinder from polar angle 180° to 360° about its
    // axis, the side facing −y.
    let axis = Frame::new(p(10.0, 0.0, -0.5), Direction::Z, Direction::NEG_X);
    let half = Cylinder::new(1.0, 1.0).with_angle(PI).with_frame(axis).build(m)?;
    let start_turn = Transform::rotation(z, (-140f64).to_radians());
    let start = modify::transform(m, half, &start_turn, TransformOptions::default())?;
    let face = rectangle_xz(m, (9.0, -0.5), (11.0, 0.5))?;
    let face = modify::transform(m, face, &start_turn, TransformOptions::default())?;
    let notch = sweep::revolve(m, face, z, 170f64.to_radians())?.shape;
    cut_each(m, block, &[notch, start])
}
