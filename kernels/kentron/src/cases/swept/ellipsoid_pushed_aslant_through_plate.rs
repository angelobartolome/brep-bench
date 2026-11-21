//! A plate, the box from (−15, −15, 0) to (15, 15, 4), with a hole punched by an ellipsoid pushed
//! aslant through it: semi-axes 3, 2 and 1.5 along x, y and z, centred at (−2, −1.5, −4), slid
//! without turning along (4, 3, 12), from wholly below the plate to wholly above it. Everything the
//! ellipsoid passes through, cut.
//!
//! kentron has no ellipsoid and no sweep of a solid, so the sweep is built as elementary geometry
//! gives it: the ellipsoid is the unit ball under the stretch S = diag(3, 2, 1.5) moved to its
//! centre, and a stretch carries a slide along w to the slide along S w. The unit ball slid along
//! S⁻¹(4, 3, 12) = (4/3, 3/2, 8) passes through its great disc square to that slid along it, with
//! the ball at each end; stretched, the disc becomes a flat ellipse through the ellipsoid's centre
//! (where the push runs along the ellipsoid) and the balls the ellipsoid where it starts and where
//! it ends, both clear of the plate. So the plate loses that ellipse slid along (4, 3, 12).
use crate::bench::{frame, p, v};
use crate::Refusal;
use kentron::boolean;
use kentron::geometry::convert::curve_to_bspline;
use kentron::geometry::curve::{Circle, Curve};
use kentron::make;
use kentron::math::{AffineTransform, Direction, Matrix3};
use kentron::modify;
use kentron::primitives::Cuboid;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = Cuboid::from_corners(p(-15.0, -15.0, 0.0), p(15.0, 15.0, 4.0)).build(m)?;
    // The unit ball's great circle square to (4/3, 3/2, 8), in its rational B-spline form, which
    // the stretch maps pole by pole, keeping its parameters.
    let circle = Circle::new(frame(p(0.0, 0.0, 0.0), Direction::new(4.0 / 3.0, 3.0 / 2.0, 8.0)), 1.0);
    let circle = curve_to_bspline(&Curve::Circle(circle)).ok_or("no B-spline form of a circle")?;
    let rim = make::edge_from_curve(m, Curve::BSpline(circle))?;
    let rim = make::wire(m, &[rim])?;
    let great = make::planar_face(m, rim)?;
    let stretch = Matrix3::from_rows([[3.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 1.5]]);
    let ellipse = modify::transform_affine(m, great, &AffineTransform::new(stretch, v(-2.0, -1.5, -4.0)))?;
    let hole = sweep::extrude(m, ellipse, v(4.0, 3.0, 12.0))?.shape;
    Ok(boolean::difference(m, plate, hole)?)
}
