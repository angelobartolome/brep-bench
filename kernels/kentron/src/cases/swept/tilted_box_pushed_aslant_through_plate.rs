//! A plate, the box from (−15, −15, 0) to (15, 15, 4), with a hole punched by a tilted box pushed
//! aslant wholly through it: the 4 × 3 × 2 box (−2, −1, −8) + a e₁ + b e₂ + c e₃, |a| ≤ 2, |b| ≤
//! 1.5, |c| ≤ 1, its edges along e₁ = (2, 1, 2)/3, e₂ = (−2, 2, 1)/3 and e₃ = (1, 2, −2)/3, slid
//! without turning along (4, 2, 18), from wholly below the plate to wholly above it. Everything the
//! box passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes a slide only of a prism square to it
//! through a disc, so the sweep is built as elementary geometry gives it: the box where it starts,
//! the box where it ends, and each of its faces slid along the vector (none is parallel to it). A
//! point the box covers at some instant that no face ever crosses stays inside the box all the
//! way, so is in it at both ends; and a flat face slid out of its plane never folds over itself,
//! so each face's prism is exactly what that face passes through.
use crate::bench::{d, p, v};
use crate::shared::swept::{compound, cut_each, solids};
use kentron::math::{Frame, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::Cuboid;
use kentron::sweep;
use kentron::topology::{Model, NodeKind, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(-15.0, -15.0, 0.0), p(15.0, 15.0, 4.0)).build(m)?;
    // The box's frame: x along e₁, y along e₂ and z along e₁ × e₂ = −e₃, at its lowest corner.
    let (e1, e2, e3) = (v(2.0, 1.0, 2.0) / 3.0, v(-2.0, 2.0, 1.0) / 3.0, v(1.0, 2.0, -2.0) / 3.0);
    let corner = p(-2.0, -1.0, -8.0) - e1 * 2.0 - e2 * 1.5 + e3;
    let start = Cuboid::new(4.0, 3.0, 2.0).with_frame(Frame::new(corner, d(-e3.x, -e3.y, -e3.z), d(e1.x, e1.y, e1.z))).build(m)?;
    let slide = v(4.0, 2.0, 18.0);
    let end = modify::transform(m, start, &Transform::translation(slide), TransformOptions::default())?;
    // The box's six faces, each slid along the vector to the prism it passes through.
    let faces: Vec<Shape> = m.explore(start, NodeKind::Face).collect();
    let faces = compound(m, &faces)?;
    let prisms = sweep::extrude(m, faces, slide)?.shape;
    let mut pieces = solids(m, prisms);
    pieces.extend([start, end]);
    cut_each(m, plate, &pieces)
}
