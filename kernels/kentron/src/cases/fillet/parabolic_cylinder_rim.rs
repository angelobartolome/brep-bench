//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circle where its side meets its
//! top blended by a conic of rho 0.5, a parabola tangent to the top and to the side each 2 from
//! the edge, turned round the axis.
//!
//! Refused: kentron's blends round an edge by a rolling ball (`blend::fillet`,
//! `blend::fillet_edges`, `blend::fillet_evolving`) or bevel it flat (`blend::chamfer`); it has no
//! conic blend of a given rho.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no conic (rho) blend: its fillets are rolling-ball rounds of circular \
         section and its chamfers are flat",
    ))
}
