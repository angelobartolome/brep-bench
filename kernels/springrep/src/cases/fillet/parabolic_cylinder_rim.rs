//! A cylinder of radius 5 from z = 0 to 10, the circle where its side meets its top blended by a
//! conic of rho 0.5, a parabola, tangent to both faces 2 from the edge.
use springrep::blend::{EdgeName, fillet_conic};
use springrep::pick::edges_named;
use springrep::primitive::cylinder;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let body = cylinder(&at([0., 0., 0.]), 5., 10., "cyl").map_err(e)?;
    let rim = EdgeName { a: "cyl.end", b: "cyl.side", exact: true };
    let edges = edges_named(&body, &rim).map_err(e)?;
    fillet_conic(&body, &edges, 2., 0.5, "conic").map_err(e)
}
