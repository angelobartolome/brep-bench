use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::FRAC_1_SQRT_2;

/// The ellipse x²/a² + y²/b² ≤ 1: four quarters, counter-clockwise, each the rational quadratic
/// from one axis's end to the next about the corner of the box round the ellipse between them,
/// its middle weighted √2/2.
pub fn ellipse(a: f64, b: f64) -> Region2 {
    let ends = [v2(a, 0.0), v2(0.0, b), v2(-a, 0.0), v2(0.0, -b), v2(a, 0.0)];
    let corners = [v2(a, b), v2(-a, b), v2(-a, -b), v2(a, -b)];
    let segs = (0..4).map(|i| Seg2::Conic { a: ends[i], apex: corners[i], b: ends[i + 1], w: FRAC_1_SQRT_2 }).collect();
    Region2 { outer: Loop2 { segs }, holes: vec![] }
}

pub fn build() -> R<Body> {
    let c = extrude(&Plane::XY, ellipse(10.0, 6.0), 0.0, 8.0)?;
    // the top rim, picked at a quarter's middle and followed round
    let rim = solidcraft_kernel::tangent_chain(&c, &[v3(10.0 * FRAC_1_SQRT_2, 6.0 * FRAC_1_SQRT_2, 8.0)])?;
    solidcraft_kernel::fillet(&c, &rim, 2.0)
}
