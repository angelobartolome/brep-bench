//! An elliptic cylinder from z = 0 to z = 8 on the ellipse x²/10² + y²/6² = 1 (semi-axes 10 along x
//! and 6 along y), the ellipse where its side meets its top chamfered by 2 on both faces: 2 in from
//! the rim across the top, square to it, and 2 down the side. The chamfer is the ruled surface of
//! constant slope 45° between the ellipse's inner parallel curve at distance 2 on the top and the
//! ellipse at z = 6 on the side.
//!
//! The floor's one edge is the ellipse itself, exactly (`Curve::Ellipse` in a profile), so `prism`
//! sweeps it into a side closed round it (`Surface::Extrusion` of the ellipse). `fillet::flat_along`
//! cuts the flat between the two legs: on the top, where it stands 2 from the side (the side's
//! offset is the ellipse's spline carried along the sweep, read by its signed distance); down the
//! side, where it stands 2 below the top's plane.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::{tolerance, Curve, Frame};
use cad_kernel::topo::Brep;

const A: f64 = 10.;
const B: f64 = 6.;
const HEIGHT: f64 = 8.;
const DISTANCE: f64 = 2.;

pub fn build() -> Result<Brep, String> {
    let rim = Curve::Ellipse(Frame::WORLD, A, B);
    let floor = Profile { origin: [0., 0., 0.], normal: [0., 0., 1.],
                          loops: vec![vec![ProfileEdge::Exact(rim)]],
                          names: vec![vec!["side".into()]] };
    let cylinder = prism(&floor, 0., HEIGHT)?;
    // the top rim picked by a point on it
    let piece = flat_along(&cylinder, [0., B, HEIGHT], [DISTANCE, DISTANCE], None,
                           tolerance(cylinder.size()))?;
    // the rim is convex: its piece is taken away
    Ok(combined(&cylinder, &piece, Op::Cut, 0.)?)
}
