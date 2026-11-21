//! A plate from (−15, −10, 0) to (15, 10, 4) with a hole through it on the ellipse
//! x²/6² + y²/3² = 1, the hole's rim on the top rounded to radius 1: a pipe, its ball's centre on
//! the ellipse's outer parallel curve at distance 1, at height 3. The rim on the bottom stays
//! sharp.
use crate::bench::{edge_through, p, v};
use crate::shared::geometry::flat;
use crate::Refusal;
use kentron::blend;
use kentron::boolean;
use kentron::geometry::curve::{Curve, Ellipse};
use kentron::make;
use kentron::math::{Direction, Frame};
use kentron::primitives::Cuboid;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = Cuboid::from_corners(p(-15.0, -10.0, 0.0), p(15.0, 10.0, 4.0)).build(m)?;
    // The drill: the ellipse at z = −1 extruded past both faces.
    let frame = Frame::new(p(0.0, 0.0, -1.0), Direction::Z, Direction::X);
    let rim = make::edge_from_curve(m, Curve::Ellipse(Ellipse::new(frame, 6.0, 3.0)))?;
    let disc = flat(m, &[rim])?;
    let drill = sweep::extrude(m, disc, v(0.0, 0.0, 6.0))?.shape;
    let holed = boolean::difference(m, plate, drill)?;
    let rim = edge_through(m, holed, p(0.0, 3.0, 4.0))?;
    Ok(blend::fillet(m, holed, &[rim], 1.0)?)
}
