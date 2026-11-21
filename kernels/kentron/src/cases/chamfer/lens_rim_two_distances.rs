//! The lens common to two balls of radius 5, about (0, 0, 3) and (0, 0, −3): its upper face on the
//! ball about (0, 0, −3), its lower on the one about (0, 0, 3), meeting in its rim, the circle of
//! radius 4 at z = 0. The rim chamfered with two distances, each in a straight line from it: 2 on
//! the upper face and 1 on the lower.
use crate::bench::{frame, p};
use crate::shared::chamfer::chamfer_rim;
use crate::Refusal;
use kentron::blend::ChamferSize;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let above = Sphere::new(5.0).with_frame(frame(p(0.0, 0.0, 3.0), Direction::Z)).build(m)?;
    let below = Sphere::new(5.0).with_frame(frame(p(0.0, 0.0, -3.0), Direction::Z)).build(m)?;
    let lens = boolean::intersection(m, above, below)?;
    // (The rim, and a point on the upper face, away from the seams.)
    chamfer_rim(m, lens, p(0.0, 4.0, 0.0), p(0.0, 3.0, 1.0), ChamferSize::Distances(2.0, 1.0))
}
