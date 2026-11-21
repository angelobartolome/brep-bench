//! A solid torus about the z axis, its tube's middle circle of radius 10 and its tube of radius 3,
//! united with a slab from (−13, −13, −1) to (13, 13, 1). The torus's outer equator touches each of
//! the slab's sides at its middle; the torus's seam is turned 45° about z, off those points.
use crate::bench::{d, p};
use kentron::boolean;
use kentron::math::{Direction, Frame};
use kentron::primitives::{Cuboid, Torus};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let turned = Frame::new(p(0.0, 0.0, 0.0), Direction::Z, d(1.0, 1.0, 0.0));
    let torus = Torus::new(10.0, 3.0).with_frame(turned).build(m)?;
    let slab = Cuboid::from_corners(p(-13.0, -13.0, -1.0), p(13.0, 13.0, 1.0)).build(m)?;
    boolean::union(m, torus, slab)
}
