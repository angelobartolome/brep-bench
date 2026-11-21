//! The solid a thick disc fills as it is turned about an upright line through its rim: the solid
//! cylinder of radius 2 about the line through (2, 0, 0) along y, from y = 0 to 0.5, turned rigidly
//! about the z axis by 90°.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) cuts a blank, and turns a tool clear past
//! it, so the solid is built as elementary geometry gives it, as a box slid is. A point the coin
//! covers at some instant stays inside it until the motion ends or a face carries it out, so it is
//! in the coin where it ends or in what one of its trailing faces passes through, those the turn's
//! velocity ẑ × r = (−y, x, 0) leaves behind: the flat face y = 0 (its normal −ŷ meets it in
//! −x) and the half of the side farther from the z axis (its normal (x − 2, 0, z)/2 meets it in
//! −y (x − 2)/2). A face turned passes through exactly the solid its revolve makes where no two of
//! its points lie on one circle about the z axis, as here, the velocity never running along either
//! face inside it.
use crate::bench::{frame, p};
use crate::shared::sweep::disc;
use kentron::boolean;
use kentron::geometry::surface::{Cylinder as CylinderSurface, Surface};
use kentron::make;
use kentron::math::{Axis, Direction, Frame, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::Cylinder;
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::{FRAC_PI_2, PI};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let coin = Cylinder::new(2.0, 0.5).with_frame(frame(p(2.0, 0.0, 0.0), Direction::Y)).build(m)?;
    let end = modify::transform(m, coin, &Transform::rotation(Axis::Z, FRAC_PI_2), TransformOptions::default())?;
    // The side, its angle u about the coin's axis from +z: from 0 to π the half farther from the z
    // axis (x ≥ 2).
    let side = Surface::Cylinder(CylinderSurface::new(Frame::new(p(2.0, 0.0, 0.0), Direction::Y, Direction::Z), 2.0));
    let faces = [disc(m, p(2.0, 0.0, 0.0), Direction::Y, 2.0)?, make::face_on_domain(m, side, 0.0, PI, 0.0, 0.5)?];
    let mut body = end;
    for face in faces {
        let turned = sweep::revolve(m, face, Axis::Z, FRAC_PI_2)?.shape;
        body = boolean::union(m, body, turned)?;
    }
    Ok(body)
}
