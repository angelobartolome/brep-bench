//! A prism from z = 0 to 6 on a periodic uniform cubic B-spline bean, the rim of its top
//! chamfered by 1 on both faces.
use springrep::blend::chamfer;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::fillet::spline_prism;

pub fn build() -> Result<Brep, String> {
    let body = spline_prism()?;
    chamfer(&body, &[("bean.end", "bean.side")], 1., "bevel").map_err(e)
}
