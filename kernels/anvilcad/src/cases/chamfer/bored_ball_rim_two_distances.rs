//! A ball of radius 5 about the origin bored through along z with radius 2, its upper rim
//! chamfered 2 down the bore and 1 on the ball.
use anvil_brep::{Frame, Solid};

use crate::shared::{ball, chamfer_distances, cylinder, ring, subtract};

pub fn build() -> Result<Solid, String> {
    let bored = subtract(&ball([0., 0., 0.], 5.)?, &cylinder(&Frame::world(), 2., -6., 6.)?)?;
    chamfer_distances(&bored, ring(2., 21f64.sqrt()), ring(2., 0.), 2., 1.)
}
