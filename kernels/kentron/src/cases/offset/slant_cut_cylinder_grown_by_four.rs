//! The cylinder of radius 3 about the z axis from z = −10 up to where the plane z = x·tan 45° cuts
//! it, grown by 4: every point within 4 of it. The bottom rim becomes a piece of a torus, the
//! elliptic top rim a canal surface about the ellipse. The distance is more than the ellipse's least radius of curvature, 2.12: the canal face is
//! the outer part of the tube, where it stays smooth.
use crate::Refusal;
use crate::shared::offset::slant_cut_cylinder;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_4;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cylinder = slant_cut_cylinder(m, FRAC_PI_4)?;
    Ok(offset::shape(m, cylinder, 4.0, Join::Arc)?)
}
