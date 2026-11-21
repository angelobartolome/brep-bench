//! A small square prism, 0.0015 square about the z axis and 0.0015 tall from z = 0, and a copy a
//! hundredth wider, 0.001515 square, between the same two planes. The copy less the prism: a
//! square tube whose walls are 0.0000075 thick.
use crate::bench::p;
use kentron::boolean;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let prism = Cuboid::from_corners(p(-0.00075, -0.00075, 0.0), p(0.00075, 0.00075, 0.0015)).build(m)?;
    let wider = Cuboid::from_corners(p(-0.0007575, -0.0007575, 0.0), p(0.0007575, 0.0007575, 0.0015)).build(m)?;
    boolean::difference(m, wider, prism)
}
