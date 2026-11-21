//! A plate from (−20, −15, 0) to (20, 15, 4) with an elliptic boss on it, the elliptic cylinder
//! on x²/10² + y²/6² = 1 from z = 4 to z = 10. The boss's root, the ellipse where its side meets
//! the plate at z = 4, rounded to radius 2, adding material: a pipe, its ball's centre on the
//! ellipse's outer parallel curve at distance 2, at height 6. The boss's top rim stays sharp.
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
    let plate = Cuboid::from_corners(p(-20.0, -15.0, 0.0), p(20.0, 15.0, 4.0)).build(m)?;
    let frame = Frame::new(p(0.0, 0.0, 4.0), Direction::Z, Direction::X);
    let rim = make::edge_from_curve(m, Curve::Ellipse(Ellipse::new(frame, 10.0, 6.0)))?;
    let base = flat(m, &[rim])?;
    let boss = sweep::extrude(m, base, v(0.0, 0.0, 6.0))?.shape;
    let part = boolean::union(m, plate, boss)?;
    let root = edge_through(m, part, p(0.0, 6.0, 4.0))?;
    Ok(blend::fillet(m, part, &[root], 2.0)?)
}
