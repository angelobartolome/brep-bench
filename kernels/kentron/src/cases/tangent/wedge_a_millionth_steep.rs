//! A cube of side 10 from (−5, −5, −10) to (5, 5, 0), common with the half-space above the plane
//! z = 10⁻⁶·y: a wedge under the half of the top face where y < 0, 5·10⁻⁶ thick at y = −5.
use crate::bench::{d, p};
use crate::shared::boolean::half_space;
use kentron::boolean;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, crate::Refusal> {
    let cube = Cuboid::from_corners(p(-5.0, -5.0, -10.0), p(5.0, 5.0, 0.0)).build(m)?;
    let above = half_space(m, p(0.0, 0.0, 0.0), d(0.0, -1e-6, 1.0), p(0.0, 0.0, 1.0))?;
    Ok(boolean::intersection(m, cube, above)?)
}
