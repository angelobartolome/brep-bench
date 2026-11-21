//! An elliptic cylinder from z = 0 to z = 8 on the ellipse x²/10² + y²/6² = 1, the ellipse
//! where its side meets its top chamfered by 2 on both faces: a ruled surface of constant slope
//! 45° between the ellipse's inner parallel curve on the top and the ellipse at z = 6.
use crate::bench::{edge_through, p, v};
use crate::shared::geometry::flat;
use crate::Refusal;
use kentron::blend;
use kentron::geometry::curve::{Curve, Ellipse};
use kentron::make;
use kentron::math::Frame;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let rim = make::edge_from_curve(m, Curve::Ellipse(Ellipse::new(Frame::WORLD, 10.0, 6.0)))?;
    let end = flat(m, &[rim])?;
    let cylinder = sweep::extrude(m, end, v(0.0, 0.0, 8.0))?.shape;
    // (A point on the rim away from the ellipse's start.)
    let rim = edge_through(m, cylinder, p(0.0, 6.0, 8.0))?;
    Ok(blend::chamfer(m, cylinder, &[rim], 2.0)?)
}
