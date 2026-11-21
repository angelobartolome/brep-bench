//! A ball of radius 5 centred on the origin, its poles on the z axis, with the cap beyond the
//! plane x + z = 5 cut away. The cut's rim runs through the pole (0, 0, 5) aslant to every
//! meridian.
use crate::bench::{d, p};
use crate::shared::boolean::half_space;
use kentron::boolean;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, crate::Refusal> {
    let ball = Sphere::new(5.0).build(m)?;
    let cap = half_space(m, p(5.0, 0.0, 0.0), d(1.0, 0.0, 1.0), p(5.0, 0.0, 5.0))?;
    Ok(boolean::difference(m, ball, cap)?)
}
