//! An L-shaped block: the L through (0, 0), (20, 0), (20, 4), (6, 4), (6, 12), (0, 12) as (x, z),
//! extruded from y = 0 to y = 10. Its inside edge, from (6, 0, 4) to (6, 10, 4), rounded with a
//! radius growing linearly along it: r = 1 + y/5, 1 at y = 0 and 3 at y = 10.
use crate::bench::{edge_through, p, v};
use crate::shared::fillet::prism;
use crate::Refusal;
use kentron::blend;
use kentron::geometry::ParametricCurve;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let l = [(0.0, 0.0), (20.0, 0.0), (20.0, 4.0), (6.0, 4.0), (6.0, 12.0), (0.0, 12.0)];
    let outline = l.map(|(x, z)| p(x, 0.0, z));
    let block = prism(m, &outline, v(0.0, 10.0, 0.0))?;
    let edge = edge_through(m, block, p(6.0, 5.0, 4.0))?;
    // The law runs from the edge's start to its end: each end gets the radius at its y.
    let (curve, first, _) = m.edge_curve(edge).ok_or("the edge has no curve")?;
    let law = if curve.point(first).y < 5.0 { vec![(0.0, 1.0), (1.0, 3.0)] } else { vec![(0.0, 3.0), (1.0, 1.0)] };
    Ok(blend::fillet_evolving(m, block, &[(edge, law)])?)
}
