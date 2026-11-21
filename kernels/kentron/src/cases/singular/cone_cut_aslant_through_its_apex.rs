//! A solid cone on the z axis, its base the disk of radius 5 at z = 0 and its apex at (0, 0, 10),
//! less the half-space where 4x + z > 10, beyond the plane through the apex that meets the base
//! plane in the line x = 2.5.
use crate::bench::{d, p};
use crate::shared::boolean::half_space;
use kentron::boolean;
use kentron::primitives::Cone;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, crate::Refusal> {
    let cone = Cone::new(5.0, 0.0, 10.0).build(m)?;
    let beyond = half_space(m, p(0.0, 0.0, 10.0), d(4.0, 0.0, 1.0), p(10.0, 0.0, 10.0))?;
    Ok(boolean::difference(m, cone, beyond)?)
}
