//! Newell's teapot made a solid: the 32 bicubic Bézier patches of the usual data set, in Newell's
//! coordinates, z up, as the union of four solids, each its patches sewn with flat faces:
//!
//! - the body, the rim, upper and lower body and bottom rings, closed by the disk inside the rim's
//!   inner edge at z = 2.4;
//! - the lid, the knob and lid rings, closed by the disk inside the lid's edge at z = 2.4;
//! - the spout, its two patches on y ≤ 0 and their mirror images, closed at its base (x = 1.7) and
//!   at its mouth (z = 2.4);
//! - the handle, its two patches and their mirror images, closed at both ends.
//!
//! A ring is four patches from a profile of four points (r, z): the first, from +x to −y, has row
//! i (r, 0, z), (r, −0.56r, z), (0.56r, −r, z), (0, −r, z), and the others are it turned about z.
use crate::bench::p;
use crate::shared::geometry::{flat, solid_of};
use crate::Refusal;
use kentron::geometry::curve::{BezierCurve, Curve};
use kentron::geometry::surface::{BezierSurface, Surface};
use kentron::math::{Axis, Point, Transform, Transformable};
use kentron::topology::{Model, Shape};
use kentron::{boolean, make};

type Patch = [[Point; 4]; 4];

const RIM: [(f64, f64); 4] = [(1.4, 2.4), (1.3375, 2.53125), (1.4375, 2.53125), (1.5, 2.4)];
const UPPER_BODY: [(f64, f64); 4] = [(1.5, 2.4), (1.75, 1.875), (2.0, 1.35), (2.0, 0.9)];
const LOWER_BODY: [(f64, f64); 4] = [(2.0, 0.9), (2.0, 0.45), (1.5, 0.225), (1.5, 0.15)];
const BOTTOM: [(f64, f64); 4] = [(1.5, 0.15), (1.5, 0.075), (1.425, 0.0), (0.0, 0.0)];
const KNOB: [(f64, f64); 4] = [(0.0, 3.15), (0.8, 3.15), (0.0, 2.85), (0.2, 2.7)];
const LID: [(f64, f64); 4] = [(0.2, 2.7), (0.4, 2.55), (1.3, 2.55), (1.3, 2.4)];

/// The patches on y ≤ 0, their control points by row: (x, y, z).
const SPOUT_LOWER: [[(f64, f64, f64); 4]; 4] = [
    [(1.7, 0.0, 1.425), (1.7, -0.66, 1.425), (1.7, -0.66, 0.6), (1.7, 0.0, 0.6)],
    [(2.6, 0.0, 1.425), (2.6, -0.66, 1.425), (3.1, -0.66, 0.825), (3.1, 0.0, 0.825)],
    [(2.3, 0.0, 2.1), (2.3, -0.25, 2.1), (2.4, -0.25, 2.025), (2.4, 0.0, 2.025)],
    [(2.7, 0.0, 2.4), (2.7, -0.25, 2.4), (3.3, -0.25, 2.4), (3.3, 0.0, 2.4)],
];
const SPOUT_LIP: [[(f64, f64, f64); 4]; 4] = [
    [(2.7, 0.0, 2.4), (2.7, -0.25, 2.4), (3.3, -0.25, 2.4), (3.3, 0.0, 2.4)],
    [(2.8, 0.0, 2.475), (2.8, -0.25, 2.475), (3.525, -0.25, 2.49375), (3.525, 0.0, 2.49375)],
    [(2.9, 0.0, 2.475), (2.9, -0.15, 2.475), (3.45, -0.15, 2.5125), (3.45, 0.0, 2.5125)],
    [(2.8, 0.0, 2.4), (2.8, -0.15, 2.4), (3.2, -0.15, 2.4), (3.2, 0.0, 2.4)],
];
const HANDLE_UPPER: [[(f64, f64, f64); 4]; 4] = [
    [(-1.6, 0.0, 2.025), (-1.6, -0.3, 2.025), (-1.5, -0.3, 2.25), (-1.5, 0.0, 2.25)],
    [(-2.3, 0.0, 2.025), (-2.3, -0.3, 2.025), (-2.5, -0.3, 2.25), (-2.5, 0.0, 2.25)],
    [(-2.7, 0.0, 2.025), (-2.7, -0.3, 2.025), (-3.0, -0.3, 2.25), (-3.0, 0.0, 2.25)],
    [(-2.7, 0.0, 1.8), (-2.7, -0.3, 1.8), (-3.0, -0.3, 1.8), (-3.0, 0.0, 1.8)],
];
const HANDLE_LOWER: [[(f64, f64, f64); 4]; 4] = [
    [(-2.7, 0.0, 1.8), (-2.7, -0.3, 1.8), (-3.0, -0.3, 1.8), (-3.0, 0.0, 1.8)],
    [(-2.7, 0.0, 1.575), (-2.7, -0.3, 1.575), (-3.0, -0.3, 1.35), (-3.0, 0.0, 1.35)],
    [(-2.5, 0.0, 1.125), (-2.5, -0.3, 1.125), (-2.65, -0.3, 0.9375), (-2.65, 0.0, 0.9375)],
    [(-2.0, 0.0, 0.9), (-2.0, -0.3, 0.9), (-1.9, -0.3, 0.6), (-1.9, 0.0, 0.6)],
];

/// A ring's four patches, in turn round the axis from +x through −y.
fn ring(profile: &[(f64, f64); 4]) -> Vec<Patch> {
    let first: Patch = profile.map(|(r, z)| [p(r, 0.0, z), p(r, -0.56 * r, z), p(0.56 * r, -r, z), p(0.0, -r, z)]);
    turned(first)
}

/// The patch and it turned −90°, −180° and −270° about the z axis.
fn turned(first: Patch) -> Vec<Patch> {
    (0..4)
        .map(|k| {
            let turn = Transform::rotation(Axis::Z, -(k as f64) * std::f64::consts::FRAC_PI_2);
            first.map(|row| row.map(|q| q.transformed(&turn)))
        })
        .collect()
}

/// A patch from its table, and its mirror image in the plane y = 0.
fn mirrored(table: &[[(f64, f64, f64); 4]; 4]) -> (Patch, Patch) {
    (table.map(|row| row.map(|(x, y, z)| p(x, y, z))), table.map(|row| row.map(|(x, y, z)| p(x, -y, z))))
}

fn face(m: &mut Model, patch: &Patch) -> kentron::Result<Shape> {
    let poles = patch.iter().map(|row| row.to_vec()).collect();
    make::face_on_surface(m, Surface::Bezier(BezierSurface::new(poles)))
}

/// The flat face inside the loop of `rows`, Bézier curves each starting where the last ends.
fn cap(m: &mut Model, rows: &[[Point; 4]]) -> kentron::Result<Shape> {
    let mut edges = Vec::new();
    for row in rows {
        edges.push(make::edge_from_curve(m, Curve::Bezier(BezierCurve::new(row.to_vec())))?);
    }
    flat(m, &edges)
}

/// A row read back to front.
fn reversed(row: [Point; 4]) -> [Point; 4] {
    [row[3], row[2], row[1], row[0]]
}

/// The solid bounded by `patches` and the flat faces inside the loops `caps`.
fn closed(m: &mut Model, patches: &[Patch], caps: &[Vec<[Point; 4]>]) -> Result<Shape, Refusal> {
    let mut faces = Vec::new();
    for patch in patches {
        faces.push(face(m, patch)?);
    }
    for rows in caps {
        faces.push(cap(m, rows)?);
    }
    solid_of(m, &faces)
}

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let rim = ring(&RIM);
    let body_cap: Vec<[Point; 4]> = rim.iter().map(|patch| patch[0]).collect();
    let body_patches: Vec<Patch> = [rim, ring(&UPPER_BODY), ring(&LOWER_BODY), ring(&BOTTOM)].concat();
    let body = closed(m, &body_patches, &[body_cap])?;

    // The knob's row 1 has 0.45 where the rule gives 0.448.
    let mut knob = KNOB.map(|(r, z)| [p(r, 0.0, z), p(r, -0.56 * r, z), p(0.56 * r, -r, z), p(0.0, -r, z)]);
    knob[1] = [p(0.8, 0.0, 3.15), p(0.8, -0.45, 3.15), p(0.45, -0.8, 3.15), p(0.0, -0.8, 3.15)];
    let lid_ring = ring(&LID);
    let lid_cap: Vec<[Point; 4]> = lid_ring.iter().map(|patch| patch[3]).collect();
    let lid = closed(m, &[turned(knob), lid_ring].concat(), &[lid_cap])?;

    let (lower, lower_mirror) = mirrored(&SPOUT_LOWER);
    let (lip, lip_mirror) = mirrored(&SPOUT_LIP);
    let base = vec![lower[0], reversed(lower_mirror[0])];
    let mouth = vec![lip[3], reversed(lip_mirror[3])];
    let spout = closed(m, &[lower, lower_mirror, lip, lip_mirror], &[base, mouth])?;

    let (upper, upper_mirror) = mirrored(&HANDLE_UPPER);
    let (low, low_mirror) = mirrored(&HANDLE_LOWER);
    let top_end = vec![upper[0], reversed(upper_mirror[0])];
    let bottom_end = vec![low[3], reversed(low_mirror[3])];
    let handle = closed(m, &[upper, upper_mirror, low, low_mirror], &[top_end, bottom_end])?;

    let mut teapot = boolean::union(m, body, lid)?;
    for part in [spout, handle] {
        teapot = boolean::union(m, teapot, part)?;
    }
    Ok(teapot)
}
