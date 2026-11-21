//! A post: the cylinder of radius 4 on the z axis from z = 0 to z = 6, capped by the part above
//! z = 6 of the ball of radius 5 about (0, 0, 3), the cap made turned 1.5 radians about z. The rim
//! where they meet is two edges, split where each surface's seam is, and is rounded to radius 1 in
//! two faces. Both are deleted: the cylinder and the dome meet again at z = 6.
use crate::bench::{d, edge_through, p};
use crate::shared::edit::{deleted, round_middle, turned_point};
use crate::Refusal;
use kentron::math::{Direction, Frame};
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};
use kentron::{blend, boolean};
use std::f64::consts::{FRAC_PI_2, PI};

const TURN: f64 = 1.5;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let post = Cylinder::new(4.0, 6.0).build(m)?;
    let turned = Frame::new(p(0.0, 0.0, 3.0), Direction::Z, d(TURN.cos(), TURN.sin(), 0.0));
    // The ball's zone above z = 6, 3 over its center: latitude asin(3/5) up.
    let dome = Sphere::new(5.0).with_frame(turned).with_latitudes(0.6f64.asin(), FRAC_PI_2).build(m)?;
    let part = boolean::union(m, post, dome)?;
    // The rim's two edges run between the seams at angles 0 and 1.5 (and π apart round the back).
    let (a, b) = (TURN / 2.0, TURN / 2.0 + PI);
    let rim = [edge_through(m, part, turned_point((4.0, 6.0), a))?, edge_through(m, part, turned_point((4.0, 6.0), b))?];
    let rounded = blend::fillet(m, part, &rim, 1.0)?;
    // The round's center, 1 in from the cylinder and from the ball: radius 3, 4 from the ball's
    // center. It touches the cylinder outward and the ball away from the ball's center.
    let center = (3.0, 3.0 + 7f64.sqrt());
    let middle = round_middle(center, (1.0, 0.0), (3.0 / 4.0, 7f64.sqrt() / 4.0), 1.0);
    deleted(m, rounded, &[turned_point(middle, a), turned_point(middle, b)])
}
