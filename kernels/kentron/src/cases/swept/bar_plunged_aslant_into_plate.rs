//! A plate, the box from (0, −5, 0) to (20, 5, 4), pocketed from below by a bar pushed up aslant
//! into it: the prism 6 ≤ x ≤ 14, |y| + |z + 6| ≤ 1 (a square stood on a corner), slid without
//! turning by 8 along (0.3, 0, √0.91), stopping inside the plate. Everything the bar passes
//! through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes a slide only of a prism square to it
//! through a disc, and this one slides partly along the bar, so the sweep is built as elementary
//! geometry gives it: the bar where it starts, the bar where it ends, and each of its faces slid
//! along the vector (none is parallel to it). A point the bar covers at some instant that no face
//! ever crosses stays inside the bar all the way, so is in it at both ends; and a flat face slid
//! out of its plane never folds over itself, so each face's prism is exactly what that face passes
//! through.
use crate::bench::{d, p, v};
use crate::shared::swept::{compound, cut_each, solids};
use kentron::math::{Direction, Frame, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::Cuboid;
use kentron::sweep;
use kentron::topology::{Model, NodeKind, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(0.0, -5.0, 0.0), p(20.0, 5.0, 4.0)).build(m)?;
    // The bar: a √2 square along x from its lowest edge (y, z) = (0, −7), its sides along
    // (0, 1, 1) and (0, −1, 1).
    let start = Cuboid::new(2f64.sqrt(), 2f64.sqrt(), 8.0).with_frame(Frame::new(p(6.0, 0.0, -7.0), Direction::X, d(0.0, 1.0, 1.0))).build(m)?;
    let slide = v(0.3, 0.0, 0.91f64.sqrt()) * 8.0;
    let end = modify::transform(m, start, &Transform::translation(slide), TransformOptions::default())?;
    // The bar's six faces, each slid along the vector to the prism it passes through.
    let faces: Vec<Shape> = m.explore(start, NodeKind::Face).collect();
    let faces = compound(m, &faces)?;
    let prisms = sweep::extrude(m, faces, slide)?.shape;
    let mut pieces = solids(m, prisms);
    pieces.extend([start, end]);
    cut_each(m, plate, &pieces)
}
