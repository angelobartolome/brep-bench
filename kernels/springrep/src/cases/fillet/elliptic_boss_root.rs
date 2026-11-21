//! A plate (−20, −15, 0)–(20, 15, 4) with an elliptic boss on x²/10² + y²/6² = 1 standing on it up
//! to z = 10, the boss's root rounded to radius 2 by a rolling ball, adding material.
use springrep::blend::fillet;
use springrep::boolean::union;
use springrep::math::{Vec2, Vec3};
use springrep::primitive::block;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::geometry::ellipse;
use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let plate = block(&at([-20., -15., 0.]), Vec3::new(40., 30., 4.), "plate").map_err(e)?;
    let section = Profile::outline(ellipse(Vec2::ZERO, 10., 6., "side"));
    let boss = extrude(&section, &at([0., 0., 4.]), 6., "boss").map_err(e)?;
    let body = union(&plate, &boss).map_err(e)?;
    fillet(&body, &[("plate.end", "boss.side")], 2., "round").map_err(e)
}
