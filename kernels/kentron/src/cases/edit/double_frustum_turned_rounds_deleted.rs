//! Two cone frustums on the z axis made one solid: from radius 3 at z = 0 to radius 5 at z = 4,
//! and, made turned 2 radians about z, from radius 5 at z = 4 to radius 2 at z = 8. Their ridge,
//! the circle of radius 5 at z = 4, is two edges split where each cone's seam is, rounded to
//! radius 1 in two faces. Both are deleted: the cones meet in the ridge again.
use crate::bench::{d, edge_through, p};
use crate::shared::edit::{corner_round_middle, deleted, turned_point};
use crate::Refusal;
use kentron::math::{Direction, Frame};
use kentron::primitives::Cone;
use kentron::topology::{Model, Shape};
use kentron::{blend, boolean};
use std::f64::consts::PI;

const TURN: f64 = 2.0;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let lower = Cone::new(3.0, 5.0, 4.0).build(m)?;
    let turned = Frame::new(p(0.0, 0.0, 4.0), Direction::Z, d(TURN.cos(), TURN.sin(), 0.0));
    let upper = Cone::new(5.0, 2.0, 4.0).with_frame(turned).build(m)?;
    let part = boolean::union(m, lower, upper)?;
    // The ridge's two edges run between the seams at angles 0 and 2.
    let (a, b) = (TURN / 2.0, TURN / 2.0 + PI);
    let ridge = [edge_through(m, part, turned_point((5.0, 4.0), a))?, edge_through(m, part, turned_point((5.0, 4.0), b))?];
    let rounded = blend::fillet(m, part, &ridge, 1.0)?;
    // The lower cone rises 4 as it widens 2, the upper 4 as it narrows 3: their outward normals.
    let (lower_out, upper_out) = ((2.0 / 5f64.sqrt(), -1.0 / 5f64.sqrt()), (0.8, 0.6));
    let middle = corner_round_middle((5.0, 4.0), lower_out, upper_out, 1.0);
    deleted(m, rounded, &[turned_point(middle, a), turned_point(middle, b)])
}
