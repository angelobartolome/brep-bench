//! The cylinder of radius 3 about the z axis from z = −10 up to where the plane z = x·tan 30° cuts
//! it, grown by 1: every point within 1 of it. The bottom rim becomes a piece of a torus, the
//! elliptic top rim a canal surface about the ellipse.
use crate::Refusal;
use crate::shared::offset::slant_cut_cylinder;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cylinder = slant_cut_cylinder(m, 30f64.to_radians())?;
    Ok(offset::shape(m, cylinder, 1.0, Join::Arc)?)
}
