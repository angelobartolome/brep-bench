//! A dog-bone plate, two lobes of radius 10 about (±30, 0) joined by a waist 8 wide through
//! concave blends of radius 6, extruded 10 up z, united with a round boss of radius 10 on the
//! right lobe's axis, 30 tall: the boss and the lobe share their side.
use anvil_brep::profile::Loop2;
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{arc_deg, at, cylinder, e, line, region, union, v2};

pub fn build() -> Result<Solid, String> {
    // the blends' centres, 6 off the sides and 16 from the lobes' centres
    let bx = 30. - 2. * 39f64.sqrt();
    // the blend–lobe tangent points lie on the line between their centres
    let t = 10f64.atan2(2. * 39f64.sqrt()).to_degrees();
    let outline = Loop2::new(vec![
        line(v2(-bx, -4.), v2(bx, -4.)),
        arc_deg((bx, -10.), 6., 90., t - 90.),
        arc_deg((30., 0.), 10., 180. + t, 360. - 2. * t),
        arc_deg((bx, 10.), 6., -t, t - 90.),
        line(v2(bx, 4.), v2(-bx, 4.)),
        arc_deg((-bx, 10.), 6., -90., t - 90.),
        arc_deg((-30., 0.), 10., t, 360. - 2. * t),
        arc_deg((-bx, -10.), 6., 180. - t, t - 90.),
    ]);
    let plate = extrude::extrude(&Frame::world(), &region(&[outline])?, 0., 10.).map_err(e)?;
    union(&plate, &cylinder(&at([30., 0., 0.]), 10., 0., 30.)?)
}
