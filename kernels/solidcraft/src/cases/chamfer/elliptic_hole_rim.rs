//! The plate from (−15, −10, 0) to (15, 10, 4) with a hole right through it on the ellipse
//! x²/6² + y²/3² = 1, the hole's rim on the top face chamfered by 1 on both faces: a countersink.
use super::elliptic_cylinder_rim::{ellipse, rim};
use crate::solids::*;

pub fn build() -> R<Body> {
    let plate = rect(-15.0, -10.0, 15.0, 10.0);
    let region = Region2 { outer: plate.outer, holes: vec![ellipse(6.0, 3.0)] };
    let holed = extrude(&Plane::XY, region, 0.0, 4.0)?;
    solidcraft_kernel::chamfer(&holed, &rim(&holed, 6.0, 3.0, 4.0)?, 1.0)
}
