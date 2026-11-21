//! A cube of side 10 between the origin and (10, 10, 10), the edge where its faces x = 10 and
//! y = 10 meet rounded with a radius shrinking linearly along it to nothing: r = 2 − z/5, 2 at the
//! bottom and 0 at the top, where the round, an oblique cone, comes to a point at the corner.
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::geometry::ParametricCurve;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let edge = edge_through(m, cube, p(10.0, 10.0, 5.0))?;
    // The law runs from the edge's start to its end: each end gets the radius at its height.
    let (curve, first, _) = m.edge_curve(edge).ok_or("the edge has no curve")?;
    let law = if curve.point(first).z < 5.0 { vec![(0.0, 2.0), (1.0, 0.0)] } else { vec![(0.0, 0.0), (1.0, 2.0)] };
    Ok(blend::fillet_evolving(m, cube, &[(edge, law)])?)
}
