//! The box from the origin to (20, 10, 10) with a half-round notch in its side y = 0: the cylinder
//! of radius 3 about the vertical line through (10, 0) cut away. The notch's wall is deleted: the
//! side, the top and the bottom close over it, and the result is the box again.
use crate::bench::{frame, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cuboid, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(20.0, 10.0, 10.0)).build(m)?;
    let cutter = Cylinder::new(3.0, 12.0).with_frame(frame(p(10.0, 0.0, -1.0), Direction::Z)).build(m)?;
    let notched = boolean::difference(m, block, cutter)?;
    deleted(m, notched, &[p(10.0, 3.0, 5.0)])
}
