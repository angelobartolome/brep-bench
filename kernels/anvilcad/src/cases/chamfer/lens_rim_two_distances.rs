//! The lens common to balls of radius 5 about (0, 0, ±3), its rim chamfered 2 on the upper face
//! and 1 on the lower.
use anvil_brep::Solid;

use crate::shared::{ball, chamfer_distances, intersect, ring};

pub fn build() -> Result<Solid, String> {
    let lens = intersect(&ball([0., 0., 3.], 5.)?, &ball([0., 0., -3.], 5.)?)?;
    // the upper face is the ball about (0, 0, −3): 1 from the axis it is at −3 + √24
    chamfer_distances(&lens, ring(4., 0.), ring(1., -3. + 24f64.sqrt()), 2., 1.)
}
