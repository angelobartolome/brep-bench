//! A cube of side 10 between the origin and (10, 10, 10), the edge where its faces x = 10 and
//! y = 10 meet chamfered with a distance on the face x = 10 growing linearly along it,
//! d₁ = 1 + z/5, and a constant 2 on the face y = 10: a hyperbolic paraboloid.
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
