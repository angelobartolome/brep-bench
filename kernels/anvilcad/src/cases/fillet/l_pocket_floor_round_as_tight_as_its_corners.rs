//! A block from (−20, −20, 0) to (20, 20, 8) with an L-shaped pocket cut 5 deep into its top, its
//! convex corners rounded to radius 1 and its re-entrant corner to radius 3; the rim of the
//! pocket's floor rounded to radius 1, as tight as the convex corners.
use anvil_brep::profile::Loop2;
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{arc_deg, block, e, fillet, line, region, rim, subtract, v2};

pub fn build() -> Result<Solid, String> {
    let pocket = Loop2::new(vec![
        line(v2(-11., -12.), v2(11., -12.)),
        arc_deg((11., -11.), 1., -90., 90.),
        line(v2(12., -11.), v2(12., -1.)),
        arc_deg((11., -1.), 1., 0., 90.),
        line(v2(11., 0.), v2(3., 0.)),
        arc_deg((3., 3.), 3., -90., -90.), // the re-entrant corner
        line(v2(0., 3.), v2(0., 11.)),
        arc_deg((-1., 11.), 1., 0., 90.),
        line(v2(-1., 12.), v2(-11., 12.)),
        arc_deg((-11., 11.), 1., 90., 90.),
        line(v2(-12., 11.), v2(-12., -11.)),
        arc_deg((-11., -11.), 1., 180., 90.),
    ]);
    let cut = extrude::extrude(&Frame::world(), &region(&[pocket.clone()])?, 3., 9.).map_err(e)?;
    let body = subtract(&block([-20., -20., 0.], [20., 20., 8.])?, &cut)?;
    fillet(&body, &rim(&pocket, 3.), 1.)
}
