//! A regular hexagonal prism from z = 0 to z = 10 on the hexagon of side 6 about the z axis with
//! corners at (6 cos 60k°, 6 sin 60k°). Its edge from (6, 0, 0) to (6, 0, 10), where the sides
//! meet at 120°, rounded with a radius growing linearly along it: r = 1 + 3z/10, 1 at the bottom
//! and 4 at the top.
use crate::bench::{edge_through, p, v};
use crate::shared::fillet::prism;
use crate::Refusal;
use kentron::blend;
use kentron::geometry::ParametricCurve;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let corners: Vec<_> =
        (0..6).map(|k| (60.0 * k as f64).to_radians()).map(|a: f64| p(6.0 * a.cos(), 6.0 * a.sin(), 0.0)).collect();
    let hexagon = prism(m, &corners, v(0.0, 0.0, 10.0))?;
    let edge = edge_through(m, hexagon, p(6.0, 0.0, 5.0))?;
    // The law runs from the edge's start to its end: each end gets the radius at its height.
    let (curve, first, _) = m.edge_curve(edge).ok_or("the edge has no curve")?;
    let law = if curve.point(first).z < 5.0 { vec![(0.0, 1.0), (1.0, 4.0)] } else { vec![(0.0, 4.0), (1.0, 1.0)] };
    Ok(blend::fillet_evolving(m, hexagon, &[(edge, law)])?)
}
