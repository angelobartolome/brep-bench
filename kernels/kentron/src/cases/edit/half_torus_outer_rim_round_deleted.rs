//! The half above z = 0 of the ring torus about the z axis whose tube has radius 2 about the
//! circle of radius 6, its flat the ring 4 ≤ r ≤ 8, its outer rim rounded to radius 1/2. The round
//! is deleted: the flat runs on outward and the torus down until they meet in the circle of radius
//! 8 at z = 0, and the result is the half torus.
use crate::bench::{edge_through, p};
use crate::shared::edit::{deleted, round_middle};
use crate::Refusal;
use kentron::primitives::{Cuboid, Torus};
use kentron::topology::{Model, Shape};
use kentron::{blend, boolean};
use std::f64::consts::SQRT_2;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // (`Torus::with_tube_angles(0.0, PI)` would make the flat two overlapping discs, of radius 8
    // and 4, so the half is the whole torus cut down to the box above z = 0.)
    let torus = Torus::new(6.0, 2.0).build(m)?;
    let above = Cuboid::from_corners(p(-9.0, -9.0, 0.0), p(9.0, 9.0, 3.0)).build(m)?;
    let half = boolean::intersection(m, torus, above)?;
    let outer = edge_through(m, half, p(0.0, 8.0, 0.0))?;
    let rounded = blend::fillet(m, half, &[outer], 0.5)?;
    // The round's center is 1/2 above the flat and 3/2 from the tube's center, √2 out from it;
    // it touches the flat below and the torus away from the tube's center.
    let (r, z) = round_middle((6.0 + SQRT_2, 0.5), (0.0, -1.0), (SQRT_2 / 1.5, 0.5 / 1.5), 0.5);
    deleted(m, rounded, &[p(0.0, r, z)])
}
