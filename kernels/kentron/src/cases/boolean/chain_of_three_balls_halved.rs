//! The chain of three balls (radius 5 about the origin, 4 about (7, 0, 0) and 3 about
//! (10, 4, 0), united one after another), with everything below the plane z = 0 through their
//! centres cut away.
use crate::bench::{frame, p};
use crate::shared::boolean::half_space;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, crate::Refusal> {
    let first = Sphere::new(5.0).build(m)?;
    let second = Sphere::new(4.0).with_frame(frame(p(7.0, 0.0, 0.0), Direction::Z)).build(m)?;
    let third = Sphere::new(3.0).with_frame(frame(p(10.0, 4.0, 0.0), Direction::Z)).build(m)?;
    let two = boolean::union(m, first, second)?;
    let chain = boolean::union(m, two, third)?;
    let below = half_space(m, p(0.0, 0.0, 0.0), Direction::Z, p(0.0, 0.0, -1.0))?;
    Ok(boolean::difference(m, chain, below)?)
}
