//! What the tangent implementations share.
use crate::bench::p;
use kentron::math::{Direction, Frame, Transform, Vector};
use kentron::primitives::{Cuboid, Cylinder, Sphere};
use kentron::topology::{Model, Shape};

/// Viviani's ball and cylinder: the ball of radius 10 about the origin, its poles on the y axis
/// and its seam through (−10, 0, 0), and the solid cylinder of radius 5 about the line x = 5,
/// y = 0, from z = −12 to z = 12, its seam on the z axis. The cylinder's wall touches the sphere
/// from inside at (10, 0, 0).
pub fn viviani(m: &mut Model) -> kentron::Result<(Shape, Shape)> {
    let away = Direction::new(-1.0, 0.0, 0.0);
    let ball = Sphere::new(10.0).with_frame(Frame::new(p(0.0, 0.0, 0.0), Direction::Y, away)).build(m)?;
    let cylinder =
        Cylinder::new(5.0, 24.0).with_frame(Frame::new(p(5.0, 0.0, -12.0), Direction::Z, away)).build(m)?;
    Ok((ball, cylinder))
}

/// The small cube of side 0.0015 from the origin to (0.0015, 0.0015, 0.0015), and a copy of it
/// moved by `offset`.
pub fn small_cube_and_copy(m: &mut Model, offset: Vector) -> kentron::Result<(Shape, Shape)> {
    let cube = Cuboid::new(0.0015, 0.0015, 0.0015).build(m)?;
    Ok((cube, cube.moved(&Transform::translation(offset))))
}

/// The small cylinder of radius 0.00075 and height 0.0015 about the z axis from z = 0, and a copy
/// of it moved by `offset`.
pub fn small_cylinder_and_copy(m: &mut Model, offset: Vector) -> kentron::Result<(Shape, Shape)> {
    let cylinder = Cylinder::new(0.00075, 0.0015).build(m)?;
    Ok((cylinder, cylinder.moved(&Transform::translation(offset))))
}

/// The solid paraboloid of revolution about the z axis with its vertex at the origin, every point
/// with x² + y² ≤ 4·`focal`·z and z ≤ `height`: the parabola of the xz plane from its vertex to
/// the top's rim, closed by the top's radius and the axis, turned a whole turn about the z axis.
pub fn paraboloid(m: &mut Model, focal: f64, height: f64) -> kentron::Result<Shape> {
    use crate::shared::geometry::{arc, flat, segment, turned};
    use kentron::geometry::curve::{Curve, Parabola};
    // (t²/(4·focal), t) along the frame's x and y: +z, and +x when its normal is +y
    let frame = Frame::new(p(0.0, 0.0, 0.0), Direction::Y, Direction::Z);
    let parabola = Curve::Parabola(Parabola::new(frame, focal));
    let rim = (4.0 * focal * height).sqrt();
    let side = arc(m, parabola, 0.0, rim)?;
    let top = segment(m, p(rim, 0.0, height), p(0.0, 0.0, height))?;
    let axis = segment(m, p(0.0, 0.0, height), p(0.0, 0.0, 0.0))?;
    let section = flat(m, &[side, top, axis])?;
    turned(m, section)
}
