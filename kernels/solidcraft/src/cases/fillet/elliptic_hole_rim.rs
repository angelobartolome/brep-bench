use crate::cases::fillet::elliptic_cylinder_rim::ellipse;
use crate::solids::*;
use std::f64::consts::FRAC_1_SQRT_2;

pub fn build() -> R<Body> {
    let plate = block((-15.0, -10.0, 0.0), (15.0, 10.0, 4.0))?;
    let plate = cut(&plate, &extrude(&Plane::XY, ellipse(6.0, 3.0), -1.0, 5.0)?)?;
    // the hole's rim on the top, picked at a quarter's middle and followed round
    let rim = solidcraft_kernel::tangent_chain(&plate, &[v3(6.0 * FRAC_1_SQRT_2, 3.0 * FRAC_1_SQRT_2, 4.0)])?;
    solidcraft_kernel::fillet(&plate, &rim, 1.0)
}
