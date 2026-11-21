use crate::cases::spline::peanut_prism::periodic_cubic;
use crate::solids::*;

/// The bean's eight control points, counter-clockwise.
const BEAN: [(f64, f64); 8] =
    [(-10.0, -5.0), (0.0, -7.0), (10.0, -5.0), (12.0, 4.0), (4.0, 7.0), (0.0, 3.0), (-4.0, 7.0), (-12.0, 4.0)];

/// The bean as its Bézier pieces (`spline/peanut-prism`), extruded from z = 0 to z = 6, and its
/// top rim rounded to radius 1.
pub fn build() -> Result<Body, crate::Refusal> {
    let outer = Loop2 { segs: periodic_cubic(&BEAN)? };
    let prism = extrude(&Plane::XY, Region2 { outer, holes: vec![] }, 0.0, 6.0)?;
    // the rim, picked at the middle of the piece on the first four control points (the uniform
    // cubic's weights there 1, 23, 23, 1 over 48) and followed round
    let rim = solidcraft_kernel::tangent_chain(&prism, &[v3(232.0 / 48.0, -277.0 / 48.0, 6.0)])?;
    Ok(solidcraft_kernel::fillet(&prism, &rim, 1.0)?)
}
