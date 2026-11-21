//! The box from the origin to (20, 10, 10), the edge where its top meets its end x = 20 blended by
//! a conic of rho 0.5, a parabola tangent to the top and to the end each 3 from the edge.
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
