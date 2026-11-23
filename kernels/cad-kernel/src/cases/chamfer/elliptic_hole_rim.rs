//! The plate from (−15, −10, 0) to (15, 10, 4) with a hole right through it on the ellipse
//! x²/6² + y²/3² = 1 (semi-axes 6 along x and 3 along y), the hole's rim on the top face chamfered
//! by 1 on both faces: 1 out from the rim across the top, square to it, and 1 down the wall. The
//! chamfer is a countersink of constant slope 45° between the ellipse at z = 3 on the wall and the
//! ellipse's outer parallel curve at distance 1 on the top; the rim on the bottom stays sharp.
//!
//! The drill's section is the ellipse itself, exactly (`Curve::Ellipse` in a profile), so the
//! hole's wall is the ellipse swept up (`Surface::Extrusion`), closed round it.
//! `fillet::flat_along` cuts the flat between the two legs: on the top, where it stands 1 from the
//! wall (the wall's offset is the ellipse's spline carried along the sweep, read by its signed
//! distance); down the wall, where it stands 1 below the top's plane.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{prism, Profile, ProfileEdge};
use cad_kernel::fillet::flat_along;
use cad_kernel::geom::{tolerance, Curve, Frame};
use cad_kernel::topo::Brep;

const A: f64 = 6.;
const B: f64 = 3.;
const THICKNESS: f64 = 4.;
const DISTANCE: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    let plate = block([-15., -10., 0.], [15., 10., THICKNESS])?;
    // the hole's outline at z = −1: the drill runs past both faces, from z = −1 to z = 5
    let below = Frame { o: [0., 0., -1.], ..Frame::WORLD };
    let section = Profile { origin: below.o, normal: [0., 0., 1.],
                            loops: vec![vec![ProfileEdge::Exact(Curve::Ellipse(below, A, B))]],
                            names: vec![vec!["wall".into()]] };
    let drill = prism(&section, 0., THICKNESS + 2.)?;
    let drilled = combined(&plate, &drill, Op::Cut, 0.)?;
    // the top rim picked by a point on it
    let piece = flat_along(&drilled, [0., B, THICKNESS], [DISTANCE, DISTANCE], None,
                           tolerance(drilled.size()))?;
    // the rim is convex (the plate's material lies in the wedge between the top and the wall): its
    // piece is taken away
    Ok(combined(&drilled, &piece, Op::Cut, 0.)?)
}
