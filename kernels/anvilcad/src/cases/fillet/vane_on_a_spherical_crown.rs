//! A disc of radius 100, flat beneath, its rim 3 tall and its top the sphere of radius 800 through
//! the rim's top circle, united with an obround vane within 2 of the segment from (22, 0) to
//! (88, 0), from z = 1 to 31; the loop where the vane meets the crown rounded to radius 2.
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{arc_deg, e, fillet, line, region, revolved, stadium, union, v2};

pub fn build() -> Result<Solid, String> {
    let c = 3. - 100. * 63f64.sqrt(); // the sphere's centre on the z axis
    let top = 800. + c;
    let start = (3. - c).atan2(100.).to_degrees();
    let disc = revolved(vec![
        line(v2(0., 0.), v2(100., 0.)),
        line(v2(100., 0.), v2(100., 3.)),
        arc_deg((0., c), 800., start, 90. - start),
        line(v2(0., top), v2(0., 0.)),
    ])?;
    let vane = extrude::extrude(&Frame::world(), &region(&[stadium((22., 0.), (88., 0.), 2.)])?, 1., 31.)
        .map_err(e)?;
    let body = union(&disc, &vane)?;
    // the foot: where the vane's sides meet the sphere, at the middle of each straight side and
    // the tip of each end
    let z = |x: f64, y: f64| c + (800f64.powi(2) - x * x - y * y).sqrt();
    fillet(
        &body,
        &[[55., -2., z(55., -2.)], [55., 2., z(55., 2.)], [90., 0., z(90., 0.)], [20., 0., z(20., 0.)]],
        2.,
    )
}
