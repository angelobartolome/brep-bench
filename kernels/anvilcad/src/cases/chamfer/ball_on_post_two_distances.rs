//! A post of radius 3 on the z axis from z = 0 to 10 joined to a ball of radius 5 about
//! (0, 0, 10), the concave edge between them chamfered 2 down the post and 1 on the ball.
use anvil_brep::{Frame, Solid};

use crate::shared::{ball, chamfer_distances, cylinder, ring, union};

pub fn build() -> Result<Solid, String> {
    let part = union(&cylinder(&Frame::world(), 3., 0., 10.)?, &ball([0., 0., 10.], 5.)?)?;
    chamfer_distances(&part, ring(3., 6.), ring(3., 3.), 2., 1.)
}
