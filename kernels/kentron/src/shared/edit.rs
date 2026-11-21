//! What the edit implementations share: a block drilled right through along z, faces moved, and
//! faces deleted, picked by points or as the solid's curved faces.
//!
//! kentron moves a face by offsetting it (`offset::shape_with_faces`, OCCT's SetOffsetOnFace):
//! the solid as a whole is offset by nothing and each face given by its own distance, along its
//! outward normal, the faces beside it extended or trimmed to meet it where their surfaces
//! intersect. A plane moved along its normal is the plane translated, and a cylinder offset is
//! the cylinder about the same axis with its radius changed.
//!
//! kentron deletes faces with `defeature::remove_faces` (OCCT's BRepAlgoAPI_Defeaturing): the
//! faces are taken out and the faces round them run on along their own surfaces until they close
//! the gap.
use crate::bench::{face_through, frame, p};
use crate::Refusal;
use kentron::defeature;
use kentron::math::{Direction, Point};
use kentron::offset::{self, Join};
use kentron::primitives::{Cuboid, Cylinder};
use kentron::geometry::surface::Surface;
use kentron::topology::{Model, NodeKind, Shape};
use kentron::{boolean, Result};

/// The box from the origin to `corner`, drilled right through along z by a hole of `radius`
/// about the vertical line through (`x`, `y`), the drill running 1 past the floor and the top.
pub fn drilled_block(m: &mut Model, corner: [f64; 3], x: f64, y: f64, radius: f64)
    -> Result<Shape> {
    let [a, b, c] = corner;
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(a, b, c)).build(m)?;
    let axis = frame(p(x, y, -1.0), Direction::Z);
    let drill = Cylinder::new(radius, c + 2.0).with_frame(axis).build(m)?;
    boolean::difference(m, block, drill)
}

/// `solid` with each (face, distance) of `moves` moved that far along its outward normal, and
/// the faces beside it extended or trimmed to meet it.
pub fn moved(m: &mut Model, solid: Shape, moves: &[(Shape, f64)]) -> Result<Shape> {
    offset::shape_with_faces(m, solid, 0.0, moves, Join::Intersection)
}

/// `solid` with the faces through `points` deleted, together: taken out, and the faces round
/// them run on along their own surfaces until they close the gap.
pub fn deleted(m: &mut Model, solid: Shape, points: &[Point]) -> std::result::Result<Shape, Refusal> {
    let mut faces = Vec::with_capacity(points.len());
    for &point in points {
        faces.push(face_through(m, solid, point)?);
    }
    Ok(defeature::remove_faces(m, solid, &faces)?.shape)
}

/// The faces of `solid` that are not flat: a box's rounds, once its edges are rounded.
pub fn curved_faces(m: &Model, solid: Shape) -> Vec<Shape> {
    let mut found: Vec<Shape> = Vec::new();
    for face in m.explore(solid, NodeKind::Face).collect::<Vec<_>>() {
        let flat = matches!(m.surface(face), Some(Surface::Plane(_)));
        if !flat && !found.iter().any(|f| f.node == face.node) {
            found.push(face);
        }
    }
    found
}

/// The point at `angle` about the z axis, `r` from it, at height `z`: a point of a turned part's
/// section (r, z), turned.
pub fn turned_point((r, z): (f64, f64), angle: f64) -> Point {
    p(r * angle.cos(), r * angle.sin(), z)
}

/// In a section through an axis, (r, z), the point of a round of `radius` about `center` midway
/// between where it touches its two faces, in the unit directions `a` and `b` from its center: a
/// point to pick the round's face by.
pub fn round_middle(center: (f64, f64), a: (f64, f64), b: (f64, f64), radius: f64) -> (f64, f64) {
    let (x, z) = (a.0 + b.0, a.1 + b.1);
    let length = x.hypot(z);
    (center.0 + radius * x / length, center.1 + radius * z / length)
}

/// In a section through an axis, (r, z), the middle of a round of `radius` in the corner at
/// `corner` between two straight sides with outward unit normals `a` and `b` (a plane's or a
/// cone's): its center is `radius` in from both, back from the corner along their bisector.
pub fn corner_round_middle(corner: (f64, f64), a: (f64, f64), b: (f64, f64), radius: f64) -> (f64, f64) {
    let back = radius / (1.0 + a.0 * b.0 + a.1 * b.1);
    let center = (corner.0 - back * (a.0 + b.0), corner.1 - back * (a.1 + b.1));
    round_middle(center, a, b, radius)
}
