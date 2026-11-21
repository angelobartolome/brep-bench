//! A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a round-bottomed
//! groove turned into it: the torus about the z axis whose tube, of radius 1.5, runs round the
//! circle of radius 5 at z = 10, cut away. The groove's face is deleted: the cylinder runs on across
//! it, and the result is the plain shaft.
use crate::bench::{frame, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cylinder, Torus};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let shaft = Cylinder::new(5.0, 20.0).build(m)?;
    let ring = Torus::new(5.0, 1.5).with_frame(frame(p(0.0, 0.0, 10.0), Direction::Z)).build(m)?;
    let grooved = boolean::difference(m, shaft, ring)?;
    // The groove's bottom, at radius 3.5.
    deleted(m, grooved, &[p(0.0, 3.5, 10.0)])
}
