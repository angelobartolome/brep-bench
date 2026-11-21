//! A turned post on the z axis: the cylinder of radius 5 from z = 0 to z = 6 and on it the cone
//! frustum from radius 5 at z = 6 to radius 2 at z = 10, made turned 1 radian about z, made one
//! solid. Its shoulder is two edges, split where each surface's seam is, rounded to radius 1 in two
//! faces. Both are deleted: the cylinder and the cone meet in the shoulder again.
use crate::bench::{d, edge_through, p};
use crate::shared::edit::{corner_round_middle, deleted, turned_point};
use crate::Refusal;
use kentron::math::{Direction, Frame};
use kentron::primitives::{Cone, Cylinder};
use kentron::topology::{Model, Shape};
use kentron::{blend, boolean};
use std::f64::consts::PI;

const TURN: f64 = 1.0;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let post = Cylinder::new(5.0, 6.0).build(m)?;
    let turned = Frame::new(p(0.0, 0.0, 6.0), Direction::Z, d(TURN.cos(), TURN.sin(), 0.0));
    let cone = Cone::new(5.0, 2.0, 4.0).with_frame(turned).build(m)?;
    let part = boolean::union(m, post, cone)?;
    // The shoulder's two edges run between the seams at angles 0 and 1.
    let (a, b) = (TURN / 2.0, TURN / 2.0 + PI);
    let shoulder = [edge_through(m, part, turned_point((5.0, 6.0), a))?, edge_through(m, part, turned_point((5.0, 6.0), b))?];
    let rounded = blend::fillet(m, part, &shoulder, 1.0)?;
    // The cylinder's outward normal, and the cone's: it narrows 3 as it rises 4.
    let middle = corner_round_middle((5.0, 6.0), (1.0, 0.0), (0.8, 0.6), 1.0);
    deleted(m, rounded, &[turned_point(middle, a), turned_point(middle, b)])
}
