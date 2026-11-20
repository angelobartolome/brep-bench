//! A regular hexagonal prism from z = 0 to z = 10 on the hexagon of side 6 about the z axis with a
//! corner at (6, 0), its edge from (6, 0, 0) to (6, 0, 10), where two sides meet at 120°, rounded
//! with a radius growing linearly along it, r = 1 + 3z/10: 1 at the bottom and 4 at the top.
//!
//! The hexagon is sketched and extruded; the edge is picked by the point at its middle and given a
//! variable fillet.
use crate::bench::{extrude, p3};
use crate::shared::fillet::fillet_linear;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let corners: Vec<[f64; 2]> = (0..6)
        .map(|i| {
            let a = (60.0 * i as f64).to_radians();
            [6.0 * a.cos(), 6.0 * a.sin()]
        })
        .collect();
    let prism = extrude(k, &Profile::of(Plane::xy(0.0), vec![Loop::polygon(&corners)]), 10.0)?;
    let edge = k.edge_at(prism, p3(6.0, 0.0, 5.0))?;
    fillet_linear(k, prism, edge, &|p| p.z < 5.0, 1.0, 4.0)
}
