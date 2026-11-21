//! Helpers shared by the fillet cases, and the chamfer cases on the same solids: the elliptic
//! cylinder, the plate with an elliptic hole, the prism on a periodic cubic spline, and the half
//! cylinder.
use std::f64::consts::{FRAC_PI_2, PI};

use springrep::boolean::difference;
use springrep::geom::BSpline;
use springrep::math::{Vec2, Vec3};
use springrep::primitive::block;
use springrep::profile::{NamedSegment, Profile, Segment};
use springrep::sweep::extrude;
use springrep::topo::Brep;

use crate::shared::geometry::ellipse;
use crate::shared::{at, e};

/// The elliptic cylinder x²/10² + y²/6² ≤ 1 from z = 0 to 8, its faces `cyl.side` and `cyl.end`.
pub fn elliptic_cylinder() -> Result<Brep, String> {
    let section = Profile::outline(ellipse(Vec2::ZERO, 10., 6., "side"));
    extrude(&section, &at([0., 0., 0.]), 8., "cyl").map_err(e)
}

/// The plate (−15, −10, 0)–(15, 10, 4) less the hole x²/6² + y²/3² < 1 right through it, cut by
/// the ellipse's extrusion running past both faces; its top is `plate.end`, the hole's wall
/// `bore.wall`.
pub fn elliptic_hole_plate() -> Result<Brep, String> {
    let plate = block(&at([-15., -10., 0.]), Vec3::new(30., 20., 4.), "plate").map_err(e)?;
    let section = Profile::outline(ellipse(Vec2::ZERO, 6., 3., "wall"));
    let bore = extrude(&section, &at([0., 0., -1.]), 6., "bore").map_err(e)?;
    difference(&plate, &bore).map_err(e)
}

/// The prism from z = 0 to 6 on the periodic uniform cubic B-spline bean, its faces `bean.side`
/// and `bean.end`.
pub fn spline_prism() -> Result<Brep, String> {
    let polygon: Vec<Vec3> = [
        (-10., -5.),
        (0., -7.),
        (10., -5.),
        (12., 4.),
        (4., 7.),
        (0., 3.),
        (-4., 7.),
        (-12., 4.),
    ]
    .iter()
    .map(|&(x, y)| Vec3::new(x, y, 0.))
    .collect();
    let curve = BSpline::periodic_uniform(3, &polygon).map_err(e)?;
    let side = Segment::Spline {
        curve,
        reversed: false,
    };
    let section = Profile::outline(vec![NamedSegment::new(side, "side")]);
    extrude(&section, &at([0., 0., 0.]), 6., "bean").map_err(e)
}

/// The half of the cylinder of radius 5 about the z axis from z = 0 to 6 on the side x ≥ 0, its
/// curved side `half.rim`, its flat face `half.flat` and its top `half.end`.
pub fn half_cylinder() -> Result<Brep, String> {
    let rim = Segment::Arc {
        center: Vec2::ZERO,
        radius: 5.,
        start: -FRAC_PI_2,
        sweep: PI,
    };
    let flat = Segment::Line {
        a: Vec2::new(0., 5.),
        b: Vec2::new(0., -5.),
    };
    let section = Profile::outline(vec![
        NamedSegment::new(rim, "rim"),
        NamedSegment::new(flat, "flat"),
    ]);
    extrude(&section, &at([0., 0., 0.]), 6., "half").map_err(e)
}
