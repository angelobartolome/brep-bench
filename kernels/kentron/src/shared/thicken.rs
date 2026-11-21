//! What the thicken implementations share: a sheet (a face, or a shell of faces that is not
//! closed) thickened into a solid.
//!
//! `thickened` is kentron's thickening of a sheet, `offset::thicken` (OCCT's
//! `BRepOffset_MakeOffset` in its thickening mode): the space the sheet sweeps moving along its
//! normals, bounded by the sheet, its offset (`offset::shape`), the walls its free edges sweep
//! and, where the offset rounds a crease, the sectors closing the rounds' ends; where the sweep
//! folds over itself (a bend thickened past its radius) or moves a face to its centre, the
//! space it covers.
use crate::bench::{face_through, p, v};
use crate::Refusal;
use kentron::geometry::curve::Circle;
use kentron::offset::{self, Join};
use kentron::math::{Axis, Direction, Frame};
use kentron::primitives::Cuboid;
use kentron::sketch::{self, Outcome, Step};
use kentron::topology::{Model, NodeKind, Shape};
use kentron::{make, sweep};
use std::f64::consts::TAU;

/// The creased sheet of the angle-bracket cases: the floor from (0, 0, 0) to (10, 8, 0) and the
/// wall from (0, 0, 0) to (0, 8, 10), the polyline (10, 0, 0), (0, 0, 0), (0, 0, 10) swept along
/// +y.
pub fn creased_sheet(m: &mut Model) -> Result<Shape, Refusal> {
    let floor = make::edge_between(m, p(10.0, 0.0, 0.0), p(0.0, 0.0, 0.0))?;
    let wall = make::edge_between(m, p(0.0, 0.0, 0.0), p(0.0, 0.0, 10.0))?;
    let crease = make::wire(m, &[floor, wall])?;
    Ok(sweep::extrude(m, crease, v(0.0, 8.0, 0.0))?.shape)
}

/// The bent sheet of the bent-flange cases: the profile in the xz plane along the line from
/// (−10, 0, 0) to (0, 0, 0), round the quarter circle of radius 2 about (0, 0, 2) to (2, 0, 2)
/// and up the line to (2, 0, 12), swept along +y from y = 0 to y = 8.
pub fn bent_sheet(m: &mut Model) -> Result<Shape, Refusal> {
    let floor = make::edge_between(m, p(-10.0, 0.0, 0.0), p(0.0, 0.0, 0.0))?;
    let half = std::f64::consts::FRAC_1_SQRT_2 * 2.0;
    let arc = Circle::arc_through(p(0.0, 0.0, 0.0), p(half, 0.0, 2.0 - half), p(2.0, 0.0, 2.0))
        .map_err(|e| format!("the bend's arc: {e:?}"))?;
    // the edge on the arc's circle itself, so that sweeping it makes a cylinder
    let (first, last) = arc.range();
    let bend = make::edge_from_curve_range(m, arc.basis().clone(), first, last)?;
    let wall = make::edge_between(m, p(2.0, 0.0, 2.0), p(2.0, 0.0, 12.0))?;
    let profile = make::wire(m, &[floor, bend, wall])?;
    Ok(sweep::extrude(m, profile, v(0.0, 8.0, 0.0))?.shape)
}

/// The open tray of the open-box cases: the floor and four walls of the cube from the origin to
/// (10, 10, 10), without its top, as one shell, each face as the cube has it (its normal out of
/// the cube).
pub fn tray(m: &mut Model) -> Result<Shape, Refusal> {
    let cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let shell = m.new_empty(NodeKind::Shell);
    for at in [p(5.0, 5.0, 0.0), p(5.0, 0.0, 5.0), p(5.0, 10.0, 5.0), p(0.0, 5.0, 5.0), p(10.0, 5.0, 5.0)] {
        let face = face_through(m, cube, at)?;
        m.add(shell, face)?;
    }
    Ok(shell)
}

/// The solid between `sheet` and its faces moved `distance` along their normals (to the other
/// side for a negative distance), the moved faces joined as `join` says where they part.
pub fn thickened(m: &mut Model, sheet: Shape, distance: f64, join: Join) -> Result<Shape, Refusal> {
    Ok(offset::thicken(m, sheet, distance, join)?)
}

/// The cup of the thicken cases turned about the z axis: in the xz plane, the floor along the
/// line from the axis out to (`floor`, 0, 0), the bend round the quarter circle of radius `bend`
/// about (`floor`, 0, `bend`) out and up to (`floor` + `bend`, 0, `bend`), and the wall up the
/// line to (`floor` + `bend`, 0, `top`), turned a whole turn: a disc, a piece of a torus and a
/// cylinder, meeting tangentially.
pub fn cup(m: &mut Model, floor: f64, bend: f64, top: f64) -> Result<Shape, Refusal> {
    use Step::{Arc, Start, To};
    let xz = Frame::new(p(0.0, 0.0, 0.0), Direction::NEG_Y, Direction::X);
    // heading along +x, the arc's centre on the left, above the floor
    let steps = [Start(0.0, 0.0), To(floor, 0.0), Arc { radius: bend, angle: 90.0 }, To(floor + bend, top)];
    let profile = sketch::profile(m, xz, &steps, Outcome::OpenWire)?;
    Ok(sweep::revolve(m, profile, Axis::Z, TAU)?.shape)
}
