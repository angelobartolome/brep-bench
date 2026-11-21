//! A solid cone on the z axis, its base the disk of radius 5 in the xy plane and its apex at
//! (0, 0, 10), with everything where x < 0 cut away: the cut passes through the apex.
use crate::bench::p;
use crate::shared::boolean::half_space;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cone;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, crate::Refusal> {
    let cone = Cone::new(5.0, 0.0, 10.0).build(m)?;
    let negative_x = half_space(m, p(0.0, 0.0, 0.0), Direction::X, p(-1.0, 0.0, 0.0))?;
    Ok(boolean::difference(m, cone, negative_x)?)
}
