//! What the offset implementations share: a planar face through a polygon, the L-shaped plate
//! three of them grow or shrink, and the solids of revolution with rounded rims that several of
//! them (and some shell implementations) grow, shrink or hollow past their rounds.
use crate::bench::{d, edge_through, frame, p, v};
use crate::Refusal;
use kentron::geometry::curve::{Curve, Ellipse};
use kentron::math::{Axis, Direction, Frame, Point};
use kentron::primitives::{Cone, Cuboid, Cylinder};
use kentron::sketch::{self, Outcome, Step};
use kentron::topology::{Model, Shape};
use kentron::{blend, boolean, make, sweep};
use std::f64::consts::TAU;

/// The planar face bounded by the polygon through `corners`, in order.
pub fn polygon_face(m: &mut Model, corners: &[Point]) -> kentron::Result<Shape> {
    let n = corners.len();
    let edges = (0..n).map(|i| make::edge_between(m, corners[i], corners[(i + 1) % n])).collect::<kentron::Result<Vec<_>>>()?;
    let wire = make::wire(m, &edges)?;
    make::planar_face(m, wire)
}

/// The L-shaped plate: the outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and
/// (0, 15) in the xy plane, extruded 8 along +z.
pub fn l_plate(m: &mut Model) -> kentron::Result<Shape> {
    let outline = [(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)];
    let corners: Vec<Point> = outline.iter().map(|&(x, y)| p(x, y, 0.0)).collect();
    let face = polygon_face(m, &corners)?;
    Ok(sweep::extrude(m, face, v(0.0, 0.0, 8.0))?.shape)
}

/// A point on the circle of radius `radius` about the z axis at height `z`, off the seams the
/// primitives put in the xz plane: a point by which to pick a rim.
pub fn on_rim(radius: f64, z: f64) -> Point {
    p(0.0, radius, z)
}

/// `solid` with the edges through `rims` (each picked by a point on it) rounded to `radius`.
pub fn rounded(m: &mut Model, solid: Shape, rims: &[Point], radius: f64) -> Result<Shape, Refusal> {
    let edges = rims.iter().map(|&at| edge_through(m, solid, at)).collect::<Result<Vec<_>, _>>()?;
    Ok(blend::fillet(m, solid, &edges, radius)?)
}

/// The cone with its tip rounded: the cone on the z axis with its base, of radius 6, at z = 0 and
/// its apex at z = 12, its tip rounded by the ball of radius 1 tangent to its side, centred on the
/// axis at z = 12 − √5. Its half-section in the xz plane is turned a whole turn about the axis:
/// the base, the side up to the circle where the ball touches it, (2/√5, 12 − 4/√5), the ball's
/// arc on to the axis, and the axis back down.
pub fn ball_tipped_cone(m: &mut Model) -> Result<Shape, Refusal> {
    use Step::{Arc, Start, To};
    let s5 = 5.0f64.sqrt();
    let xz = Frame::new(p(0.0, 0.0, 0.0), Direction::NEG_Y, Direction::X);
    // the side heads along (−1, 2); the arc, its centre on the left, turns it (in degrees) to
    // (−1, 0)
    let steps = [Start(0.0, 0.0), To(6.0, 0.0), To(2.0 / s5, 12.0 - 4.0 / s5), Arc { radius: 1.0, angle: 2.0f64.atan().to_degrees() }];
    let section = sketch::profile(m, xz, &steps, Outcome::Face)?;
    Ok(sweep::revolve(m, section, Axis::Z, TAU)?.shape)
}

/// The cone frustum on the z axis from the disc of radius 6 at z = 0 to the disc of radius 3 at
/// z = 6, its top rim rounded to `top`, its bottom rim to `bottom`, where given.
pub fn rounded_frustum(m: &mut Model, top: Option<f64>, bottom: Option<f64>) -> Result<Shape, Refusal> {
    let mut frustum = Cone::new(6.0, 3.0, 6.0).build(m)?;
    if let Some(r) = top {
        frustum = rounded(m, frustum, &[on_rim(3.0, 6.0)], r)?;
    }
    if let Some(r) = bottom {
        frustum = rounded(m, frustum, &[on_rim(6.0, 0.0)], r)?;
    }
    Ok(frustum)
}

/// The cone frustum on the z axis from the disc of radius `bottom` at z = 0 to the disc of radius
/// `top` at z = `height`.
pub fn frustum(m: &mut Model, bottom: f64, top: f64, height: f64) -> Result<Shape, Refusal> {
    Ok(Cone::new(bottom, top, height).build(m)?)
}

/// The puck, the cylinder of radius 6 on the z axis from z = 0 to z = 10, its bottom rim rounded
/// to radius 2, and its top rim too if `both`.
pub fn rounded_puck(m: &mut Model, both: bool) -> Result<Shape, Refusal> {
    let puck = Cylinder::new(6.0, 10.0).build(m)?;
    let mut rims = vec![on_rim(6.0, 0.0)];
    if both {
        rims.push(on_rim(6.0, 10.0));
    }
    rounded(m, puck, &rims, 2.0)
}

/// The turned post: the cylinder of radius 5 from z = 0 to z = 6 and on it the cone frustum from
/// radius 5 at z = 6 to radius 2 at z = 10, joined, the edge between them rounded to radius 1.
pub fn turned_post(m: &mut Model) -> Result<Shape, Refusal> {
    let shaft = Cylinder::new(5.0, 6.0).build(m)?;
    let head = Cone::new(5.0, 2.0, 4.0).with_frame(frame(p(0.0, 0.0, 6.0), Direction::Z)).build(m)?;
    let post = boolean::union(m, shaft, head)?;
    rounded(m, post, &[on_rim(5.0, 6.0)], 1.0)
}

/// The plate, the cylinder of radius 8 on the z axis from z = −`thickness` to z = 0, with `boss`
/// standing on it, joined, the concave edge where the boss meets the plate's top, the circle of
/// radius `foot` at z = 0, filleted to `radius`.
pub fn on_plate(m: &mut Model, thickness: f64, boss: Shape, foot: f64, radius: f64) -> Result<Shape, Refusal> {
    let plate = Cylinder::new(8.0, thickness).with_frame(frame(p(0.0, 0.0, -thickness), Direction::Z)).build(m)?;
    let joined = boolean::union(m, plate, boss)?;
    rounded(m, joined, &[on_rim(foot, 0.0)], radius)
}

/// The cylinder of radius 3 about the z axis from z = −10 up to where the plane z = x·tan
/// `angle` cuts it: the box above that plane taken away.
pub fn slant_cut_cylinder(m: &mut Model, angle: f64) -> Result<Shape, Refusal> {
    let cylinder = Cylinder::new(3.0, 20.0).with_frame(frame(p(0.0, 0.0, -10.0), Direction::Z)).build(m)?;
    let (s, c) = angle.sin_cos();
    // the box's frame on the plane, its z the plane's upward normal; 20 across, centred on the axis
    let above = Frame::new(p(-10.0 * c, -10.0, -10.0 * s), d(-s, 0.0, c), d(c, 0.0, s));
    let cutter = Cuboid::new(20.0, 20.0, 20.0).with_frame(above).build(m)?;
    Ok(boolean::difference(m, cylinder, cutter)?)
}

/// The elliptic cylinder of `geometry/elliptic-cylinder`: the ellipse x²/25 + y²/9 ≤ 1 in the
/// plane z = 0 extruded straight up to z = 10.
pub fn elliptic_cylinder(m: &mut Model) -> kentron::Result<Shape> {
    let rim = make::edge_from_curve(m, Curve::Ellipse(Ellipse::new(Frame::WORLD, 5.0, 3.0)))?;
    let end = crate::shared::geometry::flat(m, &[rim])?;
    Ok(sweep::extrude(m, end, v(0.0, 0.0, 10.0))?.shape)
}

/// The plate of `offset/drilled-plate-grown`, from (−10, −10, 0) to (10, 10, 4), with a hole of
/// radius 3 drilled through it along the z axis, the drill running 1 past both faces.
pub fn drilled_plate(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(-10.0, -10.0, 0.0), p(10.0, 10.0, 4.0)).build(m)?;
    let drill = Cylinder::new(3.0, 6.0).with_frame(frame(p(0.0, 0.0, -1.0), Direction::Z)).build(m)?;
    boolean::difference(m, plate, drill)
}
