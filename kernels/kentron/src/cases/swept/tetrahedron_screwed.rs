//! The solid a regular tetrahedron fills as it is carried along a screw: the tetrahedron with
//! corners (5, 1, 1), (5, −1, −1), (3, 1, −1) and (3, −1, 1), turned right-handed about the z axis
//! while rising 5 a turn, rigidly, from a turn of 0° to 540°.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) cuts with solids of revolution only, and
//! cuts rather than sweeps, so the sweep is built as elementary geometry gives it. A point the
//! tetrahedron covers at some instant and not where it starts came into it through a face the
//! screw carries forward there. The screw's velocity at p, (−y, x, 5/2π) a radian, is linear in
//! p, so on a flat face its sign along the outward normal is set at the corners: forward all over
//! the faces through (5, 1, 1), (3, 1, −1) and either (5, −1, −1) or (3, −1, 1), backward all over
//! the other two. A face carried forward all over never folds over itself, and it never meets
//! itself either: its points' helices come back to its plane only outside its triangle while
//! they have risen less than the tetrahedron's height of 2, and once past that (rising 5 a turn,
//! after about 144°) not at all. Each sweeps its
//! screwed prism (`sweep::screw_extrude`). The sweep is the tetrahedron where it starts and those
//! two prisms.
use crate::bench::p;
use crate::shared::offset::polygon_face;
use kentron::boolean;
use kentron::math::Axis;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (a, b, c, d) = (p(5.0, 1.0, 1.0), p(5.0, -1.0, -1.0), p(3.0, 1.0, -1.0), p(3.0, -1.0, 1.0));
    // The two faces carried forward first, then the two carried back.
    let mut faces = Vec::new();
    for triangle in [[a, b, c], [a, c, d], [a, d, b], [b, d, c]] {
        faces.push(polygon_face(m, &triangle)?);
    }
    let mut body = boolean::enclosed_solids(m, &faces)?;
    for &leading in &faces[..2] {
        let prism = sweep::screw_extrude(m, leading, Axis::Z, 5.0, 7.5)?.shape;
        body = boolean::union(m, body, prism)?;
    }
    Ok(body)
}
