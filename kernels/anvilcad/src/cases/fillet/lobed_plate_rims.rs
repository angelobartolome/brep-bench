//! A plate from z = 0 to 12 whose outline is six lobes of radius 8, 40 from the z axis, and six
//! hollows of radius 30 touching them, twelve arcs in one tangent loop; both rims rounded to
//! radius 1.
use anvil_brep::profile::Loop2;
use anvil_brep::{Frame, Solid, extrude};
use glam::DVec2;

use crate::shared::{arc_deg, e, fillet, region, rim};

pub fn build() -> Result<Solid, String> {
    let d = 20. * 3f64.sqrt() + 6. * 29f64.sqrt(); // the hollows' centres from the axis
    let lobe = |k: i32| DVec2::from_angle((60. * k as f64).to_radians()) * 40.;
    let hollow = |k: i32| DVec2::from_angle((60. * k as f64 + 30.).to_radians()) * d;
    let deg = |v: DVec2| v.y.atan2(v.x).to_degrees();
    let wrap = |a: f64| a.rem_euclid(360.);
    let mut segments = Vec::new();
    for k in 0..6 {
        // round lobe k anticlockwise, from the hollow before it to the hollow after it
        let (l, before, after) = (lobe(k), hollow(k - 1), hollow(k));
        let (a0, a1) = (deg(before - l), deg(after - l));
        segments.push(arc_deg((l.x, l.y), 8., a0, wrap(a1 - a0)));
        // round hollow k clockwise, from lobe k to lobe k + 1
        let next = lobe(k + 1);
        let (b0, b1) = (deg(l - after), deg(next - after));
        segments.push(arc_deg((after.x, after.y), 30., b0, -wrap(b0 - b1)));
    }
    let outline = Loop2::new(segments);
    let plate = extrude::extrude(&Frame::world(), &region(&[outline.clone()])?, 0., 12.).map_err(e)?;
    let mut rims = rim(&outline, 12.);
    rims.extend(rim(&outline, 0.));
    fillet(&plate, &rims, 1.)
}
