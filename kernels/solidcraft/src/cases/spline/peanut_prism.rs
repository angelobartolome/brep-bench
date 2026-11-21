//! A prism from z = 0 to z = 10 on the closed, periodic uniform cubic B-spline over the peanut's
//! twelve control points: the curve extruded straight up, capped, its side one smooth face. On
//! solidcore-ops, which holds clamped B-splines only and clamps the periodic curve itself
//! (`geometry/periodic-bspline-prism`).
use crate::shared::nurbs::periodic;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

/// The peanut's twelve control points, counter-clockwise.
const PEANUT: [[f64; 3]; 12] = [
    [14.0, 0.0, 0.0],
    [12.0, 8.0, 0.0],
    [4.0, 6.0, 0.0],
    [0.0, -21.0 / 8.0, 0.0],
    [-4.0, 6.0, 0.0],
    [-12.0, 8.0, 0.0],
    [-14.0, 0.0, 0.0],
    [-12.0, -8.0, 0.0],
    [-4.0, -6.0, 0.0],
    [0.0, 21.0 / 8.0, 0.0],
    [4.0, -6.0, 0.0],
    [12.0, -8.0, 0.0],
];

pub fn build() -> Result<Body, Refusal> {
    let section = periodic(3, &PEANUT)?;
    Ok(extrude(&[section], [0.0, 0.0, 1.0], 10.0, 0.0, DraftCorner::Curvature, true)?)
}
