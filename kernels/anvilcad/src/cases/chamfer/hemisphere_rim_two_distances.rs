//! A hemisphere, its rim chamfered 2 on the flat and 1 on the ball.
use anvil_brep::Solid;

use crate::shared::{ball, beyond, chamfer_distances, ring, subtract};

pub fn build() -> Result<Solid, String> {
    let half = subtract(&ball([0., 0., 0.], 5.)?, &beyond([0., 0., 0.], [0., 0., -1.], 10.)?)?;
    chamfer_distances(&half, ring(5., 0.), ring(1., 0.), 2., 1.)
}
