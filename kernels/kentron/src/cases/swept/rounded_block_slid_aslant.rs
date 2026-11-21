//! The solid a rounded block fills as it slides, without turning, along (8, 5, 3): the block is
//! everything within 1 of the box from (1, 1, 1) to (9, 5, 3), the box from (0, 0, 0) to
//! (10, 6, 4) with every edge rounded to radius 1.
//!
//! kentron has no sweep of a solid (`sweep::extrude` takes faces, wires, edges, vertices and
//! compounds of them), so the sweep is built as elementary geometry gives it. A point is in the
//! rounded block slid when it lies within 1 of a point of the inner box slid; so the sweep is the
//! inner box's sweep (the box at each end and its faces slid, as in `swept/box-slid-aslant`)
//! grown by 1 with its edges rounded. A point within 1 of a union is within 1 of one of its
//! pieces, so each piece is grown by kentron's offset (`offset::shape`, arcs at the edges) and
//! the grown pieces are united. (Grown whole, the inner box's sweep comes out far too small.)
use crate::bench::{p, v};
use crate::shared::swept::{compound, solids};
use kentron::boolean;
use kentron::math::Transform;
use kentron::modify::{self, TransformOptions};
use kentron::offset::{self, Join};
use kentron::primitives::Cuboid;
use kentron::sweep;
use kentron::topology::{Model, NodeKind, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let slide = v(8.0, 5.0, 3.0);
    let start = Cuboid::from_corners(p(1.0, 1.0, 1.0), p(9.0, 5.0, 3.0)).build(m)?;
    let end = modify::transform(m, start, &Transform::translation(slide), TransformOptions::default())?;
    let faces: Vec<Shape> = m.explore(start, NodeKind::Face).collect();
    let faces = compound(m, &faces)?;
    let prisms = sweep::extrude(m, faces, slide)?.shape;
    let mut body = offset::shape(m, start, 1.0, Join::Arc)?;
    for piece in solids(m, prisms).into_iter().chain([end]) {
        let grown = offset::shape(m, piece, 1.0, Join::Arc)?;
        body = boolean::union(m, body, grown)?;
    }
    Ok(body)
}
