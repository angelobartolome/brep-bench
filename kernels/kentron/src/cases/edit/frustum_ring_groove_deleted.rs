//! A cone frustum about the z axis from radius 6 at z = 0 to radius 3 at z = 20, with a ring groove
//! turned into it: everything outside radius 4 from z = 8 to z = 12 cut away. The groove's floor
//! and both its walls are deleted together: the cone runs on across the groove, and the result is
//! the plain frustum.
use crate::bench::{frame, p};
use crate::shared::edit::deleted;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cone, Cylinder};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let frustum = Cone::new(6.0, 3.0, 20.0).build(m)?;
    let outside = Cylinder::new(7.0, 4.0).with_frame(frame(p(0.0, 0.0, 8.0), Direction::Z)).build(m)?;
    let core = Cylinder::new(4.0, 6.0).with_frame(frame(p(0.0, 0.0, 7.0), Direction::Z)).build(m)?;
    let ring = boolean::difference(m, outside, core)?;
    let grooved = boolean::difference(m, frustum, ring)?;
    // The floor, and the walls at z = 8 (out to radius 4.8) and z = 12 (out to 4.2).
    deleted(m, grooved, &[p(0.0, 4.0, 10.0), p(0.0, 4.4, 8.0), p(0.0, 4.1, 12.0)])
}
