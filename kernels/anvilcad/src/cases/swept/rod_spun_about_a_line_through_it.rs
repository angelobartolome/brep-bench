//! What a rod (radius 0.5 about the x axis from x = −3 to 5) fills as it turns 120° about the z
//! axis, which crosses it.
use anvil_brep::Solid;
use anvil_brep::profile::Segment;

use crate::shared::{intersect, line, rod, swung, turn_z, union, v2};

/// The half section an arm of the rod reaching `r` from the axis covers: from the axis out to the
/// sphere of radius √(r² + 0.5²) its far rim reaches, within |z| ≤ 0.5.
fn arm(r: f64) -> Vec<Segment> {
    let far = (r * r + 0.25).sqrt();
    let a = (0.5 / far).asin();
    vec![
        line(v2(0., -0.5), v2(far * a.cos(), -0.5)),
        Segment::Arc { center: v2(0., 0.), radius: far, start: -a, sweep: 2. * a },
        line(v2(far * a.cos(), 0.5), v2(0., 0.5)),
        line(v2(0., 0.5), v2(0., -0.5)),
    ]
}

pub fn build() -> Result<Solid, String> {
    // anvil-brep has no sweep of a solid. Each arm turned about the axis fills the revolve of what
    // it covers in a half-plane: the long arm's fan from 0° to 120°, the short one's from 180° to
    // 300°; and the rod itself where it starts and stops
    let r = rod([-3., 0., 0.], [5., 0., 0.], 0.5)?;
    let turned = turn_z(120.);
    let with_fans = union(&union(&r, &swung(arm(5.), 0., 120.)?)?, &swung(arm(3.), 180., 120.)?)?;
    let mut solid = union(&with_fans, &r.transformed(&turned))?;
    // Beyond an arm's end (5 or 3 from the axis) the rod covers, on a circle about the axis, only
    // the angles between its end cap and its side, so the fans above leave out a sliver before each
    // fan's start and after its end, which the end corners sweep. On the start side the sliver is
    // what the rod's cylinder, carried on past its end, covers there within the arm's reach, and on
    // the end side the same turned by 120°
    let long = rod([-3.5, 0., 0.], [5.5, 0., 0.], 0.5)?;
    let slivers = [
        intersect(&long, &swung(arm(5.), -90., 90.)?)?,
        intersect(&long, &swung(arm(3.), 90., 90.)?)?,
        intersect(&long.transformed(&turned), &swung(arm(5.), 120., 90.)?)?,
        intersect(&long.transformed(&turned), &swung(arm(3.), 300., 90.)?)?,
    ];
    for sliver in &slivers {
        solid = union(&solid, sliver)?;
    }
    Ok(solid)
}
