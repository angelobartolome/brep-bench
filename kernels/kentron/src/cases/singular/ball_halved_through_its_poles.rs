//! A ball of radius 5 centred on the origin, its poles on the z axis, with everything where x < 0
//! cut away: the cut runs along a meridian from pole to pole.
use crate::bench::p;
use crate::shared::boolean::half_space;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, crate::Refusal> {
    let ball = Sphere::new(5.0).build(m)?;
    let negative_x = half_space(m, p(0.0, 0.0, 0.0), Direction::X, p(-1.0, 0.0, 0.0))?;
    Ok(boolean::difference(m, ball, negative_x)?)
}
