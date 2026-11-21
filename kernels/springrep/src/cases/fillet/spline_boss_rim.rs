//! A prism from z = 0 to 6 on a periodic uniform cubic B-spline bean, the rim of its top rounded
//! to radius 1 by a rolling ball.
use springrep::blend::fillet;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::fillet::spline_prism;

pub fn build() -> Result<Brep, String> {
    let body = spline_prism()?;
    fillet(&body, &[("bean.end", "bean.side")], 1., "round").map_err(e)
}
