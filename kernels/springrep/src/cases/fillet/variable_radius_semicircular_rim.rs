//! A half cylinder of radius 5 from z = 0 to 6 on the side x ≥ 0, the half circle where its side
//! meets its top rounded with a radius growing linearly with the angle from 1 to 2.
use springrep::blend::{EdgeName, fillet_variable};
use springrep::math::Vec3;
use springrep::pick::edges_named;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::fillet::half_cylinder;

pub fn build() -> Result<Brep, String> {
    let body = half_cylinder()?;
    let rim = EdgeName { a: "half.end", b: "half.rim", exact: true };
    let [rim] = edges_named(&body, &rim).map_err(e)?[..] else {
        return Err("the top and the side meet in more than one edge".into());
    };
    // radius 1 at the end (0, −5, 6), 2 at the end (0, 5, 6)
    fillet_variable(&body, rim, Vec3::new(0., -5., 6.), 1., 2., "round").map_err(e)
}
