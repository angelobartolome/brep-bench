use crate::cases::fillet::elliptic_cylinder_rim::ellipse;
use crate::solids::*;
use std::f64::consts::FRAC_1_SQRT_2;

pub fn build() -> R<Body> {
    let plate = block((-20.0, -15.0, 0.0), (20.0, 15.0, 4.0))?;
    let part = union(&plate, &extrude(&Plane::XY, ellipse(10.0, 6.0), 4.0, 10.0)?)?;
    // the boss's root, picked at a quarter's middle and followed round
    let root = solidcraft_kernel::tangent_chain(&part, &[v3(10.0 * FRAC_1_SQRT_2, 6.0 * FRAC_1_SQRT_2, 4.0)])?;
    solidcraft_kernel::fillet(&part, &root, 2.0)
}
