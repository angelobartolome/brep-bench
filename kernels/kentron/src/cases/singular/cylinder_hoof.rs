//! A cylinder of radius 5 on the z axis from z = −1 to z = 10, kept above the plane z = 0 and below
//! the plane z = x·tan 30°, which meets the first along the y axis: a hoof over the half disk
//! where x ≥ 0.
use crate::bench::{d, frame, p};
use crate::shared::boolean::half_space;
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, crate::Refusal> {
    let origin = p(0.0, 0.0, 0.0);
    let cylinder = Cylinder::new(5.0, 11.0).with_frame(frame(p(0.0, 0.0, -1.0), Direction::Z)).build(m)?;
    let above = half_space(m, origin, Direction::Z, p(0.0, 0.0, 1.0))?;
    let below_slope = half_space(m, origin, d(-30f64.to_radians().tan(), 0.0, 1.0), p(0.0, 0.0, -1.0))?;
    let base_up = boolean::intersection(m, cylinder, above)?;
    Ok(boolean::intersection(m, base_up, below_slope)?)
}
