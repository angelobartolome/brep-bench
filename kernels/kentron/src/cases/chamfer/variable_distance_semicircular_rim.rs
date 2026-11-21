//! The half of the cylinder of radius 5 on the z axis from z = 0 to z = 6 with x ≥ 0, the half
//! circle where its curved side meets its top chamfered by the same distance on both faces,
//! growing linearly with the angle round the axis, d = 3/2 + φ/π: a ruled surface between a
//! spiral of Archimedes on the top and a helix on the side.
//!
//! Refused: kentron's chamfers (`blend::chamfer`, `blend::chamfer_edges`) take distances that are
//! constant along each edge; only its fillets vary along an edge (`blend::fillet_evolving`).
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no chamfer whose distance varies along the edge: blend::chamfer_edges takes \
         two distances, or a distance and an angle, constant along each edge",
    ))
}
