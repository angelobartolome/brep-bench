//! A plate (−15, −10, 0)–(15, 10, 4) with a hole through it on x²/6² + y²/3² = 1, the hole's rim
//! on the top rounded to radius 1 by a rolling ball.
use springrep::blend::fillet;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::fillet::elliptic_hole_plate;

pub fn build() -> Result<Brep, String> {
    let body = elliptic_hole_plate()?;
    fillet(&body, &[("plate.end", "bore.wall")], 1., "round").map_err(e)
}
