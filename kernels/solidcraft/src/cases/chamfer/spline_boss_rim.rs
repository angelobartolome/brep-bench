//! The bean prism from z = 0 to z = 6 on the periodic uniform cubic B-spline through the control
//! points (−10, −5), (0, −7), (10, −5), (12, 4), (4, 7), (0, 3), (−4, 7), (−12, 4), its top rim
//! chamfered by 1 on both faces.
//!
//! On solidcore-ops, whose profile curves take the periodic B-spline as the kernel clamps it
//! (`geometry/periodic-bspline-prism`).
use crate::bench::{edge_through, reloaded};
use crate::shared::nurbs::periodic;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::{blend, tangent_chain, Blend, Body, Corners};

const POLYGON: [[f64; 3]; 8] = [
    [-10.0, -5.0, 0.0],
    [0.0, -7.0, 0.0],
    [10.0, -5.0, 0.0],
    [12.0, 4.0, 0.0],
    [4.0, 7.0, 0.0],
    [0.0, 3.0, 0.0],
    [-4.0, 7.0, 0.0],
    [-12.0, 4.0, 0.0],
];

pub fn build() -> Result<Body, Refusal> {
    let section = periodic(3, &POLYGON)?;
    let prism = extrude(&[section], [0.0, 0.0, 1.0], 6.0, 0.0, DraftCorner::Curvature, true)?;
    let (prism, order) = reloaded(&prism)?;
    // the rim picked at the middle of the first span, (P₀ + 23P₁ + 23P₂ + P₃)/48, off its knots,
    // and the edges that run on from it tangentially
    let [p0, p1, p2, p3] = [POLYGON[0], POLYGON[1], POLYGON[2], POLYGON[3]];
    let mid = |i: usize| (p0[i] + 23.0 * p1[i] + 23.0 * p2[i] + p3[i]) / 48.0;
    let pick = edge_through(&prism, &order, [mid(0), mid(1), 6.0])?;
    let chain = tangent_chain(&prism, None, &[pick], &[], 2f64.to_radians())?;
    let sizes = vec![1.0; chain.members.len()];
    blend(&prism, Blend::Chamfer, &chain.members, &sizes, &[], Corners::Filled)?;
    Ok(prism)
}
