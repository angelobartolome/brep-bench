//! A tube about the z axis from z = 0 to z = 20, outer radius 10 and bore radius 4, with a ring
//! groove turned into its bore: everything inside radius 6 from z = 8 to z = 12 cut away. The
//! groove's floor and both its walls are deleted together: the bore runs on across it, and the
//! result is the plain tube.
use crate::bench::{frame, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let outside = Cylinder::new(10.0, 20.0).build(m)?;
    let bore = Cylinder::new(4.0, 22.0).with_frame(frame(p(0.0, 0.0, -1.0), Direction::Z)).build(m)?;
    let tube = boolean::difference(m, outside, bore)?;
    let groove = Cylinder::new(6.0, 4.0).with_frame(frame(p(0.0, 0.0, 8.0), Direction::Z)).build(m)?;
    let grooved = boolean::difference(m, tube, groove)?;
    deleted(m, grooved, &[p(0.0, 6.0, 10.0), p(0.0, 5.0, 8.0), p(0.0, 5.0, 12.0)])
}
