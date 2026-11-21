//! A block, the box from (−3, −3, −3) to (3, 3, 3), with a sealed void inside it: everything the
//! cube from (−1, −1, −1) to (1, 1, 1) passes through as it turns a whole turn about its own long
//! diagonal, cut.
//!
//! kentron's cut by a motion (`sweep::cut_by_motion`) cuts a blank, and turns a tool clear past
//! it, so the void is built as elementary geometry gives it: turned a whole turn, the cube passes
//! through the solid of revolution about the diagonal whose radius at each point of it is the
//! farthest the cube's section square to the diagonal there reaches. That section is a polygon
//! whose corners lie on the cube's edges, all equally far from the diagonal (the cube turned a
//! third of a turn about it is itself), so the radius is that of any one of its corners: along
//! the edge from (−1, −1, −1) to (1, −1, −1), then the edge on to (1, 1, −1), skew to the
//! diagonal, then the edge on to (1, 1, 1). So the void is bounded by those three edges turned a
//! whole turn about the diagonal: two cones and a hyperboloid of one sheet between them.
use crate::bench::{d, p};
use crate::shared::geometry::solid_of;
use crate::shared::sweep::polyline;
use crate::Refusal;
use kentron::boolean;
use kentron::math::Axis;
use kentron::primitives::Cuboid;
use kentron::sweep;
use kentron::topology::{Model, NodeKind, Shape};
use std::f64::consts::TAU;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(-3.0, -3.0, -3.0), p(3.0, 3.0, 3.0)).build(m)?;
    let diagonal = Axis::new(p(-1.0, -1.0, -1.0), d(1.0, 1.0, 1.0));
    let edges = polyline(m, &[p(-1.0, -1.0, -1.0), p(1.0, -1.0, -1.0), p(1.0, 1.0, -1.0), p(1.0, 1.0, 1.0)], false)?;
    let turned = sweep::revolve(m, edges, diagonal, TAU)?.shape;
    let faces: Vec<Shape> = m.explore(turned, NodeKind::Face).collect();
    let void = solid_of(m, &faces)?;
    Ok(boolean::difference(m, block, void)?)
}
