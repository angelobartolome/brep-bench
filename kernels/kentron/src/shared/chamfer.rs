//! What the chamfer implementations share.
use crate::bench::{edge_through, face_through, frame, p};
use crate::Refusal;
use kentron::blend::{self, ChamferSize, ChamferedEdge};
use kentron::boolean;
use kentron::make;
use kentron::math::{Direction, Point, Vector};
use kentron::primitives::{Cone, Cylinder, Sphere};
use kentron::sew::{self, SewOptions};
use kentron::sweep;
use kentron::topology::{Model, NodeKind, Shape};
use std::f64::consts::FRAC_PI_2;

/// A planar face bounded by the closed polygon through `points`.
fn polygon(m: &mut Model, points: &[Point]) -> kentron::Result<Shape> {
    let mut sides = Vec::with_capacity(points.len());
    for (i, &a) in points.iter().enumerate() {
        sides.push(make::edge_between(m, a, points[(i + 1) % points.len()])?);
    }
    let outline = make::wire(m, &sides)?;
    make::planar_face(m, outline)
}

/// The prism of the closed polygon through `points`, extruded along `v`.
pub fn prism(m: &mut Model, points: &[Point], v: Vector) -> kentron::Result<Shape> {
    let section = polygon(m, points)?;
    Ok(sweep::extrude(m, section, v)?.shape)
}

/// The octahedron |x| + |y| + |z| ≤ `size`: its eight triangles sewn into a shell, the shell
/// made a solid.
pub fn octahedron(m: &mut Model, size: f64) -> Result<Shape, Refusal> {
    let mut faces = Vec::new();
    for sx in [1.0, -1.0] {
        for sy in [1.0, -1.0] {
            for sz in [1.0, -1.0] {
                let corners = [p(sx * size, 0.0, 0.0), p(0.0, sy * size, 0.0), p(0.0, 0.0, sz * size)];
                faces.push(polygon(m, &corners)?);
            }
        }
    }
    let shell = sew::sew(m, &faces, &SewOptions::default())?;
    if shell.kind() != NodeKind::Shell {
        return Err(format!("the octahedron's faces sewed into a {:?}, not a shell", shell.kind()).into());
    }
    let solid = m.new_empty(NodeKind::Solid);
    m.add(solid, shell)?;
    Ok(solid)
}

/// `solid` with the edge through `rim` chamfered by `size`, its first distance measured along
/// the face through `face`: a rim picked, and the face it is measured on, by points on them.
pub fn chamfer_rim(m: &mut Model, solid: Shape, rim: Point, face: Point, size: ChamferSize) -> Result<Shape, Refusal> {
    let edge = edge_through(m, solid, rim)?;
    let face = face_through(m, solid, face)?;
    Ok(blend::chamfer_edges(m, solid, &[ChamferedEdge { edge, size, face: Some(face) }])?)
}

/// The spherical sector of the ball of radius 5 about the origin inside the cone with its apex
/// there, about the z axis, opening upward with half angle α (`cos_alpha` its cosine): the ball
/// in common with a solid cone reaching past it, from that apex to height 6.
pub fn spherical_sector(m: &mut Model, cos_alpha: f64) -> kentron::Result<Shape> {
    let tan_alpha = (1.0 - cos_alpha * cos_alpha).sqrt() / cos_alpha;
    let ball = Sphere::new(5.0).build(m)?;
    let cone = Cone::new(0.0, 6.0 * tan_alpha, 6.0).build(m)?;
    boolean::intersection(m, ball, cone)
}

/// The ball of radius 5 about the origin standing in a conical seat: the cylinder of radius 8
/// about the z axis from z = −6 to 5, less the solid cone above z = 1 + r/2 (its apex at
/// (0, 0, 1), opening upward through the cylinder's top rim and on past it), the ball joined to
/// it. The seat's cone meets the ball in the circle of radius 4 at z = 3.
pub fn ball_in_conical_seat(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cylinder::new(8.0, 11.0).with_frame(frame(p(0.0, 0.0, -6.0), Direction::Z)).build(m)?;
    let hollow = Cone::new(0.0, 12.0, 6.0).with_frame(frame(p(0.0, 0.0, 1.0), Direction::Z)).build(m)?;
    let seat = boolean::difference(m, plate, hollow)?;
    let ball = Sphere::new(5.0).build(m)?;
    boolean::union(m, seat, ball)
}

/// The plate, the cylinder of radius 8 about the z axis from z = −6 to 0, with the ball of
/// radius 5 about the origin joined to it, its upper half standing on the plate's top.
pub fn ball_on_plate(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cylinder::new(8.0, 6.0).with_frame(frame(p(0.0, 0.0, -6.0), Direction::Z)).build(m)?;
    let ball = Sphere::new(5.0).build(m)?;
    boolean::union(m, plate, ball)
}

/// The frustum about the z axis widening from radius 2 at z = 0 to 4 at z = 3, capped by the part
/// of the ball of radius 5 about the origin above z = 3 (the sphere primitive from latitude
/// asin(3/5) up), the two joined on the disc of radius 4 at z = 3.
pub fn capped_frustum(m: &mut Model) -> kentron::Result<Shape> {
    let frustum = Cone::new(2.0, 4.0, 3.0).build(m)?;
    let cap = Sphere::new(5.0).with_latitudes((3.0f64 / 5.0).asin(), FRAC_PI_2).build(m)?;
    boolean::union(m, frustum, cap)
}
