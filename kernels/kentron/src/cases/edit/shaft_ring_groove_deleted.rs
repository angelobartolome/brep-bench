//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a ring groove
//! turned into it: everything between radius 4 and 5 from z = 8 to z = 12 cut away. The groove's
//! floor and both its walls are deleted together: the cylinder runs on across the groove, and the
//! result is the plain shaft.
use crate::bench::{frame, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let shaft = Cylinder::new(5.0, 20.0).build(m)?;
    let outside = Cylinder::new(6.0, 4.0).with_frame(frame(p(0.0, 0.0, 8.0), Direction::Z)).build(m)?;
    let core = Cylinder::new(4.0, 6.0).with_frame(frame(p(0.0, 0.0, 7.0), Direction::Z)).build(m)?;
    let ring = boolean::difference(m, outside, core)?;
    let grooved = boolean::difference(m, shaft, ring)?;
    deleted(m, grooved, &[p(0.0, 4.0, 10.0), p(0.0, 4.5, 8.0), p(0.0, 4.5, 12.0)])
}
