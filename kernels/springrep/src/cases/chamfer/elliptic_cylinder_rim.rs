//! An elliptic cylinder from z = 0 to 8 on x²/10² + y²/6² = 1, the ellipse where its side meets
//! its top chamfered by 2 on both faces.
use springrep::blend::chamfer;
use springrep::topo::Brep;

use crate::shared::e;
use crate::shared::fillet::elliptic_cylinder;

pub fn build() -> Result<Brep, String> {
    let body = elliptic_cylinder()?;
    chamfer(&body, &[("cyl.end", "cyl.side")], 2., "bevel").map_err(e)
}
