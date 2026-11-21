//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a V-groove turned
//! into it: everything farther from the axis than 3 + |z − 10| cut away, two cones meeting at
//! radius 3 at z = 10. Both cones are deleted together: the cylinder runs on across the groove,
//! and the result is the plain shaft.
use crate::bench::p;
use crate::shared::edit::deleted;
use crate::shared::geometry::{flat, segment, turned};
use crate::Refusal;
use kentron::boolean;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let shaft = Cylinder::new(5.0, 20.0).build(m)?;
    // The groove's tool: the triangle (r, z) = (3, 10), (6, 13), (6, 7) turned about the axis,
    // reaching past the shaft.
    let (tip, top, bottom) = (p(3.0, 0.0, 10.0), p(6.0, 0.0, 13.0), p(6.0, 0.0, 7.0));
    let sides = [segment(m, tip, bottom)?, segment(m, bottom, top)?, segment(m, top, tip)?];
    let section = flat(m, &sides)?;
    let tool = turned(m, section)?;
    let grooved = boolean::difference(m, shaft, tool)?;
    deleted(m, grooved, &[p(0.0, 4.0, 9.0), p(0.0, 4.0, 11.0)])
}
