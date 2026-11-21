//! A plate, the box from (0, −5, 0) to (20, 5, 4), pocketed from below by a drum pushed up aslant
//! into it: the solid cylinder of radius 2 about the line y = 0, z = −5, from x = 6 to 14, slid
//! without turning by 8 along (0.3, 0, √0.91), stopping inside the plate. Everything the drum
//! passes through, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) takes screws and turns of a solid of
//! revolution, not a slide, so the sweep is built as elementary geometry gives it: the drum where
//! it starts, where it ends, and each of its faces slid along the vector. A point the drum covers
//! at some instant that no face ever crosses stays inside it all the way, so is in it at both
//! ends. The flat ends slide out of their planes (the slide leans along x), so never fold over
//! themselves; the side's normal (0, cos u, sin u) is square to the slide only on its lines at
//! y = ±2, so split there, into its upper and lower halves, each slides without folding.
use crate::bench::{p, v};
use crate::shared::sweep::disc;
use crate::shared::swept::{compound, cut_each, solids};
use kentron::geometry::surface::{Cylinder as CylinderSurface, Surface};
use kentron::make;
use kentron::math::{Direction, Frame, Transform};
use kentron::modify::{self, TransformOptions};
use kentron::primitives::{Cuboid, Cylinder};
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::{PI, TAU};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let plate = Cuboid::from_corners(p(0.0, -5.0, 0.0), p(20.0, 5.0, 4.0)).build(m)?;
    // The drum's frame: z along its axis, x along +y, so that its angle u runs from +y up over +z.
    let axis = Frame::new(p(6.0, 0.0, -5.0), Direction::X, Direction::Y);
    let start = Cylinder::new(2.0, 8.0).with_frame(axis).build(m)?;
    let slide = v(0.3, 0.0, 0.91f64.sqrt()) * 8.0;
    let end = modify::transform(m, start, &Transform::translation(slide), TransformOptions::default())?;
    // The two flat ends, and the side in its upper (u from 0 to π) and lower halves.
    let mut faces = vec![disc(m, p(6.0, 0.0, -5.0), Direction::X, 2.0)?, disc(m, p(14.0, 0.0, -5.0), Direction::X, 2.0)?];
    let side = Surface::Cylinder(CylinderSurface::new(axis, 2.0));
    for (u1, u2) in [(0.0, PI), (PI, TAU)] {
        faces.push(make::face_on_domain(m, side.clone(), u1, u2, 0.0, 8.0)?);
    }
    let faces = compound(m, &faces)?;
    let prisms = sweep::extrude(m, faces, slide)?.shape;
    let mut pieces = solids(m, prisms);
    pieces.extend([start, end]);
    cut_each(m, plate, &pieces)
}
