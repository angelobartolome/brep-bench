//! The octahedron |x| + |y| + |z| ≤ 10 with all twelve edges rounded to radius 2, four rounds
//! meeting at each corner, closed there by a patch of the sphere touching all four faces.
use crate::bench::edges;
use crate::shared::chamfer::octahedron;
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let solid = octahedron(m, 10.0)?;
    let all = edges(m, solid);
    Ok(blend::fillet(m, solid, &all, 2.0)?)
}
