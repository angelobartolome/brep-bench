//! The elliptic cylinder x²/10² + y²/6² ≤ 1 from z = 0 to z = 8, its top rim chamfered by 2 on
//! both faces. The ellipse is drawn as the four rational quadratic quarters it is (each from one
//! axis to the next, its apex the corner of the bounding box, weight 1/√2).
use crate::solids::*;
use solidcore_record::Seg2;
use std::f64::consts::FRAC_1_SQRT_2;

/// The ellipse of semi-axes `a` along x and `b` along y about the origin, counter-clockwise.
pub fn ellipse(a: f64, b: f64) -> Loop2 {
    let axes = [v2(a, 0.0), v2(0.0, b), v2(-a, 0.0), v2(0.0, -b)];
    let segs = (0..4)
        .map(|i| {
            let (p, q) = (axes[i], axes[(i + 1) % 4]);
            Seg2::Conic { a: p, apex: p + q, b: q, w: FRAC_1_SQRT_2 }
        })
        .collect();
    Loop2 { segs }
}

/// The rim `ellipse(a, b)` makes at height `z`, as edge picks: its chain of quarters, from a pick
/// at the middle of the first (off the joints on the axes).
pub fn rim(body: &Body, a: f64, b: f64, z: f64) -> R<Vec<Vec3>> {
    solidcraft_kernel::tangent_chain(body, &[v3(a * FRAC_1_SQRT_2, b * FRAC_1_SQRT_2, z)])
}

pub fn build() -> R<Body> {
    let region = Region2 { outer: ellipse(10.0, 6.0), holes: vec![] };
    let cylinder = extrude(&Plane::XY, region, 0.0, 8.0)?;
    solidcraft_kernel::chamfer(&cylinder, &rim(&cylinder, 10.0, 6.0, 8.0)?, 2.0)
}
