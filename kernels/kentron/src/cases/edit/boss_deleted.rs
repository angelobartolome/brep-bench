//! A plate, the cylinder of radius 8 on the z axis from z = −4 to z = 0, with a boss on it, the
//! cylinder of radius 3 from z = 0 to z = 8. The boss's side and top are deleted: the plate's top
//! closes over its foot, and the result is the plate.
use crate::bench::{frame, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = Cylinder::new(8.0, 4.0).with_frame(frame(p(0.0, 0.0, -4.0), Direction::Z)).build(m)?;
    let boss = Cylinder::new(3.0, 8.0).build(m)?;
    let part = boolean::union(m, plate, boss)?;
    deleted(m, part, &[p(0.0, 3.0, 4.0), p(0.0, 0.0, 8.0)])
}
