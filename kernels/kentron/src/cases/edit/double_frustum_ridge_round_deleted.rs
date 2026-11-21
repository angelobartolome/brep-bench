//! Two cone frustums on the z axis made one solid: from radius 3 at z = 0 to radius 5 at z = 4,
//! and from radius 5 at z = 4 to radius 2 at z = 8. Their ridge, the circle of radius 5 at z = 4,
//! is rounded to radius 1. The round is deleted: the cones run on until they meet in the ridge
//! again.
use crate::bench::{edge_through, frame, p};
use crate::shared::edit::{corner_round_middle, deleted};
use crate::Refusal;
use kentron::math::Direction;
use kentron::primitives::Cone;
use kentron::topology::{Model, Shape};
use kentron::{blend, boolean};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let lower = Cone::new(3.0, 5.0, 4.0).build(m)?;
    let upper = Cone::new(5.0, 2.0, 4.0).with_frame(frame(p(0.0, 0.0, 4.0), Direction::Z)).build(m)?;
    let part = boolean::union(m, lower, upper)?;
    let ridge = edge_through(m, part, p(0.0, 5.0, 4.0))?;
    let rounded = blend::fillet(m, part, &[ridge], 1.0)?;
    // The lower cone rises 4 as it widens 2, the upper 4 as it narrows 3: their outward normals.
    let (lower_out, upper_out) = ((2.0 / 5f64.sqrt(), -1.0 / 5f64.sqrt()), (0.8, 0.6));
    let (r, z) = corner_round_middle((5.0, 4.0), lower_out, upper_out, 1.0);
    deleted(m, rounded, &[p(0.0, r, z)])
}
