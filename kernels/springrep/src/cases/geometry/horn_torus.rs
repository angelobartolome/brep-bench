//! The horn torus about the z axis, middle circle radius 5 and tube radius 5: the tube touches the
//! axis at the origin, where its face pinches to a point.
use springrep::primitive::horn_torus;
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    horn_torus(&at([0., 0., 0.]), 5., "ring").map_err(e)
}
