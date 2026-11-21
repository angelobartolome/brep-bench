//! A ball of radius 5 centred on the origin, its poles on the z axis, and the half-space above the
//! plane z = 4.9999: their common part, the cap of the ball 10⁻⁴ high around the pole (0, 0, 5).
use crate::bench::p;
use crate::shared::boolean::half_space;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, crate::Refusal> {
    let ball = Sphere::new(5.0).build(m)?;
    let above = half_space(m, p(0.0, 0.0, 4.9999), Direction::Z, p(0.0, 0.0, 6.0))?;
    Ok(boolean::intersection(m, ball, above)?)
}
