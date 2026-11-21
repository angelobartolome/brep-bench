//! A wedge from z = 0 to z = 6 on the triangle (0, 0), (10, 0), (10, 10 tan 5°): its edge along
//! the z axis, where two faces meet at 5°, rounded to radius 0.1. The round turns through 175°
//! and touches each face 0.1 cot 2.5° from the edge.
use crate::bench::{edge_through, p, v};
use crate::shared::fillet::prism;
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let blade = [p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 10.0 * 5f64.to_radians().tan(), 0.0)];
    let wedge = prism(m, &blade, v(0.0, 0.0, 6.0))?;
    let edge = edge_through(m, wedge, p(0.0, 0.0, 3.0))?;
    Ok(blend::fillet(m, wedge, &[edge], 0.1)?)
}
