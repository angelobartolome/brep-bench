//! A cylinder of radius 5 on the z axis from z = 0 to z = 10 whose side is one NURBS surface,
//! the rational quadratic circle of nine weighted poles round the axis times a line along it,
//! and whose ends are flat discs. The circle where the side meets the top rounded to radius 1.
//!
//! kentron seams a face only on a periodic surface, so the circle round the axis is taken in
//! its periodic form, the same curve (see `circle_nurbs_periodic`): the net's first eight
//! columns, its ninth the first again.
use crate::bench::{edge_through, p};
use crate::shared::geometry::{circle_nurbs_periodic, flat, solid_of};
use crate::Refusal;
use kentron::blend;
use kentron::geometry::curve::{Circle, Curve};
use kentron::geometry::surface::{BSplineSurface, KnotVector, Surface};
use kentron::make;
use kentron::math::{Direction, Frame};
use kentron::topology::{Model, Shape};

const R: f64 = 5.0;
const HEIGHT: f64 = 10.0;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let (circle, w, knots, mults) = circle_nurbs_periodic();
    let poles = circle.iter().map(|&(x, y)| vec![p(R * x, R * y, 0.0), p(R * x, R * y, HEIGHT)]).collect();
    let weights = w.iter().map(|&wi| vec![wi, wi]).collect();
    let up = KnotVector::new(vec![0.0, 1.0], vec![2, 2], 1);
    let net = BSplineSurface::try_new(poles, KnotVector::periodic(knots, mults, 2), up)?.with_weights(weights);
    let side = make::face_on_surface(m, Surface::BSpline(net))?;
    let mut ends = Vec::new();
    for z in [0.0, HEIGHT] {
        let rim = Circle::new(Frame::new(p(0.0, 0.0, z), Direction::Z, Direction::X), R);
        let rim = make::edge_from_curve(m, Curve::Circle(rim))?;
        ends.push(flat(m, &[rim])?);
    }
    let cylinder = solid_of(m, &[side, ends[0], ends[1]])?;
    let rim = edge_through(m, cylinder, p(0.0, R, HEIGHT))?;
    Ok(blend::fillet(m, cylinder, &[rim], 1.0)?)
}
