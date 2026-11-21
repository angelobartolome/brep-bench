//! What the fillet implementations share.
use kentron::geometry::curve::{BSplineCurve, Curve};
use kentron::make;
use kentron::math::{Point, Vector};
use kentron::sweep;
use kentron::topology::{Model, Shape};

/// The prism of the closed polygon through `points`, extruded along `v`.
pub fn prism(m: &mut Model, points: &[Point], v: Vector) -> kentron::Result<Shape> {
    let mut sides = Vec::with_capacity(points.len());
    for (i, &a) in points.iter().enumerate() {
        sides.push(make::edge_between(m, a, points[(i + 1) % points.len()])?);
    }
    let outline = make::wire(m, &sides)?;
    let section = make::planar_face(m, outline)?;
    Ok(sweep::extrude(m, section, v)?.shape)
}

/// The bean of `fillet/spline-boss-rim` and `chamfer/spline-boss-rim` extruded from z = 0 to
/// z = 6: the periodic uniform cubic B-spline on eight control points, dented at the top, its
/// knots 0 … 8 each once.
pub fn spline_boss(m: &mut Model) -> kentron::Result<Shape> {
    let corners = [(-10.0, -5.0), (0.0, -7.0), (10.0, -5.0), (12.0, 4.0), (4.0, 7.0), (0.0, 3.0), (-4.0, 7.0), (-12.0, 4.0)];
    let poles = corners.iter().map(|&(x, y)| Point::new(x, y, 0.0)).collect();
    let knots = (0..=8).map(f64::from).collect();
    let curve = BSplineCurve::try_new_periodic(poles, knots, vec![1; 9], 3)?;
    let rim = make::edge_from_curve(m, Curve::BSpline(curve))?;
    let rim = make::wire(m, &[rim])?;
    let section = make::planar_face(m, rim)?;
    Ok(sweep::extrude(m, section, Vector::new(0.0, 0.0, 6.0))?.shape)
}
