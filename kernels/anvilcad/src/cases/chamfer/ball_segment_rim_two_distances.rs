//! A ball of radius 5 about the origin below z = 3, its rim chamfered 2 on the flat and 1 on the
//! ball.
use anvil_brep::Solid;

use crate::shared::{ball, beyond, chamfer_distances, ring, subtract};

pub fn build() -> Result<Solid, String> {
    let segment = subtract(&ball([0., 0., 0.], 5.)?, &beyond([0., 0., 3.], [0., 0., 1.], 10.)?)?;
    chamfer_distances(&segment, ring(4., 3.), ring(1., 3.), 2., 1.)
}
