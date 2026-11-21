//! An L-shaped block 10 deep, its inside edge rounded with a radius growing from 1 at y = 0 to 3
//! at y = 10, adding material.
use springrep::blend::{EdgeName, fillet_variable};
use springrep::math::{Vec2, Vec3};
use springrep::pick::edges_named;
use springrep::profile::Profile;
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::{e, frame};

pub fn build() -> Result<Brep, String> {
    // the section in (x, z) on the plane y = 0, extruded along +y
    let xz = frame([0., 0., 0.], [0., 1., 0.], [1., 0., 0.])?;
    let p = |x: f64, z: f64| Vec2::new(x, -z); // the frame's y is −z
    let section = Profile::polygon(
        &[p(0., 0.), p(20., 0.), p(20., 4.), p(6., 4.), p(6., 12.), p(0., 12.)],
        &["bottom", "end", "ledge", "wall", "top", "back"],
    );
    let ell = extrude(&section, &xz, 10., "l").map_err(e)?;
    let edge = EdgeName { a: "l.ledge", b: "l.wall", exact: true };
    let [edge] = edges_named(&ell, &edge).map_err(e)?[..] else {
        return Err("the ledge and the wall meet in more than one edge".into());
    };
    // radius 1 at the end on the face y = 0, 3 at the face y = 10
    fillet_variable(&ell, edge, Vec3::new(6., 0., 4.), 1., 3., "round").map_err(e)
}
