//! A plate from (−40, −25, 0) to (40, 25, 13), its upright edges rounded to radius 10, united with
//! a lug of radius 9 about the upright through (0, 25); the two concave edges where the lug meets
//! the north side rounded to radius 8, then the rim of the bottom rounded to radius 2 as one run.
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{at, cylinder, e, fillet, region, rounded_rect, rounded_rect_sides, union};

pub fn build() -> Result<Solid, String> {
    let (lo, hi) = ((-40., -25.), (40., 25.));
    let plate = extrude::extrude(&Frame::world(), &region(&[rounded_rect(lo, hi, 10.)])?, 0., 13.)
        .map_err(e)?;
    let body = union(&plate, &cylinder(&at([0., 25., 0.]), 9., 0., 13.)?)?;
    let blended = fillet(&body, &[[9., 25., 6.5], [-9., 25., 6.5]], 8.)?;
    // the bottom rim: the plate's south, east and west sides and four corners, the north side's
    // two pieces, the two blends and the lug's arc
    let corners = rounded_rect_sides(lo, hi, 10., 0.);
    let mut points = vec![corners[0], corners[1], corners[3]];
    points.extend_from_slice(&corners[4..]);
    points.push([22.5, 25., 0.]);
    points.push([-22.5, 25., 0.]);
    points.push([0., 34., 0.]);
    // each blend, about (±15, 33), runs from straight below its centre to the line to the lug's axis
    let mid = (-90f64 + (-8f64).atan2(-15.).to_degrees()) / 2.;
    let (s, c) = mid.to_radians().sin_cos();
    points.push([15. + 8. * c, 33. + 8. * s, 0.]);
    points.push([-15. - 8. * c, 33. + 8. * s, 0.]);
    fillet(&blended, &points, 2.)
}
