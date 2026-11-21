//! A regular hexagonal prism of side 6 from z = 0 to 10, its edge through (6, 0) rounded with a
//! radius growing from 1 at the bottom to 4 at the top.
use std::f64::consts::PI;

use springrep::blend::{EdgeName, fillet_variable};
use springrep::math::{Vec2, Vec3};
use springrep::pick::edges_named;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let corners: Vec<Vec2> = (0..6)
        .map(|k| {
            let a = k as f64 * PI / 3.;
            Vec2::new(6. * a.cos(), 6. * a.sin())
        })
        .collect();
    let section = Profile::polygon(&corners, &["s0", "s1", "s2", "s3", "s4", "s5"]);
    let prism = extrude(&section, &at([0., 0., 0.]), 10., "hex").map_err(e)?;
    // the side from the last corner to (6, 0) and the side from (6, 0) to the next
    let edge = EdgeName { a: "hex.s5", b: "hex.s0", exact: true };
    let [edge] = edges_named(&prism, &edge).map_err(e)?[..] else {
        return Err("the two sides meet in more than one edge".into());
    };
    fillet_variable(&prism, edge, Vec3::new(6., 0., 0.), 1., 4., "round").map_err(e)
}
