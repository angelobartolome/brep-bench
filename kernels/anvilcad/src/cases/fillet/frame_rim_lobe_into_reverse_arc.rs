//! A block from z = 0 to 10 over a plan 80 by 40 with its corners rounded to radius 5 and a lobe
//! hanging from its south side, thirteen pieces in one tangent chain; the rim of its top rounded
//! to radius 2 as one run.
use anvil_brep::profile::Loop2;
use anvil_brep::{Frame, Solid, extrude};

use crate::shared::{arc_deg, e, fillet, line, region, rim, v2};

pub fn build() -> Result<Solid, String> {
    let plan = Loop2::new(vec![
        line(v2(5., 0.), v2(30., 0.)),
        arc_deg((30., -5.), 5., 90., -90.), // dip, concave
        arc_deg((45., -5.), 10., 180., 180.), // lobe
        line(v2(55., -5.), v2(55., 0.)), // rise
        arc_deg((60., 0.), 5., 180., -90.), // reverse arc, concave
        line(v2(60., 5.), v2(75., 5.)), // step
        arc_deg((75., 10.), 5., -90., 90.),
        line(v2(80., 10.), v2(80., 35.)),
        arc_deg((75., 35.), 5., 0., 90.),
        line(v2(75., 40.), v2(5., 40.)),
        arc_deg((5., 35.), 5., 90., 90.),
        line(v2(0., 35.), v2(0., 5.)),
        arc_deg((5., 5.), 5., 180., 90.),
    ]);
    let block = extrude::extrude(&Frame::world(), &region(&[plan.clone()])?, 0., 10.).map_err(e)?;
    fillet(&block, &rim(&plan, 10.), 2.)
}
