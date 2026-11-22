//! The box from (−50, −50, 0) to (50, 50, 20) united with a post of radius 15 about the upright
//! through (45, 0), from z = 0 to 40, overhanging the plate's side; a hole of radius 6 drilled
//! through the post's axis, both its rims chamfered 1.5.
use anvil_brep::Solid;

use crate::shared::{at, block, chamfer, cylinder, subtract, union};

pub fn build() -> Result<Solid, String> {
    let body = union(
        &block([-50., -50., 0.], [50., 50., 20.])?,
        &cylinder(&at([45., 0., 0.]), 15., 0., 40.)?,
    )?;
    let drilled = subtract(&body, &cylinder(&at([45., 0., 0.]), 6., -1., 41.)?)?;
    // the bottom rim picked both where it crosses the post's overhanging foot (x = 51) and the
    // plate's bottom (x = 39), in case the bottom is two faces there; and the top rim
    chamfer(
        &drilled,
        &[[51., 0., 0.], [39., 0., 0.], [45., 6., 40.]],
        1.5,
    )
}
