//! A drum, the solid cylinder of radius 5 about the z axis from z = 0 to z = 10, united with a
//! sleeve between radius 5 and 7 from z = 2 to z = 8 whose bore is the drum's cylinder written as
//! a rational B-spline: degree 2 round the axis on the nine weighted poles of the circle of radius
//! 5 (knots 0, ¼, ½, ¾, 1), degree 1 along it from z = 2 to z = 8. The sleeve is the cylinder of
//! radius 7 less the core that bore bounds with two flat disks.
//!
//! kentron seams a face only on a periodic surface, so the circle round the axis is taken in its
//! periodic form, the same curve (see `circle_nurbs_periodic`), its poles starting at (5, 0)
//! rather than the net's (0, 5): the seam a quarter turn round, the surface the same.
use crate::bench::{frame, p};
use crate::shared::geometry::{circle_nurbs_periodic, flat, solid_of};
use crate::Refusal;
use kentron::geometry::curve::{Circle, Curve};
use kentron::geometry::surface::{BSplineSurface, KnotVector, Surface};
use kentron::math::Direction;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};
use kentron::{boolean, make};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let drum = Cylinder::new(5.0, 10.0).build(m)?;
    let (circle, w, knots, mults) = circle_nurbs_periodic();
    let poles = circle.iter().map(|&(x, y)| vec![p(5.0 * x, 5.0 * y, 2.0), p(5.0 * x, 5.0 * y, 8.0)]).collect();
    let weights = w.iter().map(|&wi| vec![wi, wi]).collect();
    let bore = BSplineSurface::try_new(poles, KnotVector::periodic(knots, mults, 2), KnotVector::new(vec![0.0, 1.0], vec![2, 2], 1))?
        .with_weights(weights);
    let bore = make::face_on_surface(m, Surface::BSpline(bore))?;
    let mut ends = Vec::new();
    for z in [2.0, 8.0] {
        let rim = make::edge_from_curve(m, Curve::Circle(Circle::new(frame(p(0.0, 0.0, z), Direction::Z), 5.0)))?;
        ends.push(flat(m, &[rim])?);
    }
    let core = solid_of(m, &[bore, ends[0], ends[1]])?;
    let outside = Cylinder::new(7.0, 6.0).with_frame(frame(p(0.0, 0.0, 2.0), Direction::Z)).build(m)?;
    let sleeve = boolean::difference(m, outside, core)?;
    Ok(boolean::union(m, drum, sleeve)?)
}
