//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circle where its side meets its
//! top rounded unequally, 1 across the top and 3 down the side, its section a quarter ellipse.
//!
//! Refused: kentron's fillets are rolling-ball rounds (`blend::fillet`, `blend::fillet_edges`,
//! `blend::fillet_evolving`), of circular section with one radius at each point of the edge; it
//! has no round with two distances, of elliptic section.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no fillet of elliptic section (two distances): its fillets are rolling-ball \
         rounds of circular section",
    ))
}
