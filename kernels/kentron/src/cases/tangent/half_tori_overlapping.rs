//! Two halves of the solid torus about the z axis whose tube's middle circle has radius 10 and
//! whose tube has radius 3, united: the torus less the half-space below z = 0, and a copy turned a
//! quarter turn about z (its seam a quarter turn from the first's) less the half-space where
//! x < 0. Their curved faces coincide where x > 0 and z > 0.
use crate::bench::{d, p};
use crate::shared::boolean::half_space;
use kentron::boolean;
use kentron::math::{Direction, Frame};
use kentron::primitives::Torus;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, crate::Refusal> {
    let origin = p(0.0, 0.0, 0.0);
    let torus = Torus::new(10.0, 3.0).build(m)?;
    let below = half_space(m, origin, Direction::Z, p(0.0, 0.0, -1.0))?;
    let upper = boolean::difference(m, torus, below)?;
    let turned = Torus::new(10.0, 3.0).with_frame(Frame::new(origin, Direction::Z, d(0.0, 1.0, 0.0))).build(m)?;
    let behind = half_space(m, origin, Direction::X, p(-1.0, 0.0, 0.0))?;
    let toward_x = boolean::difference(m, turned, behind)?;
    Ok(boolean::union(m, upper, toward_x)?)
}
