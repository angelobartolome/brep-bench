//! An L-block, the box from the origin to (20, 10, 6) joined to the box from the origin to
//! (6, 10, 20). The concave edge along y where the foot meets the upright rounded to radius 2,
//! then the end face's edge it turns into one tangent chain, up the upright, round that round
//! and along the foot, rounded to radius 2: a quarter torus at the corner, a convex round turned
//! round a concave one.
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::boolean;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let foot = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(20.0, 10.0, 6.0)).build(m)?;
    let upright = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(6.0, 10.0, 20.0)).build(m)?;
    let block = boolean::union(m, foot, upright)?;
    let inside = edge_through(m, block, p(6.0, 5.0, 6.0))?;
    let block = blend::fillet(m, block, &[inside], 2.0)?;
    // The chain's edge up the upright: the rounding runs on round the arc and along the foot.
    let end = edge_through(m, block, p(6.0, 0.0, 15.0))?;
    Ok(blend::fillet(m, block, &[end], 2.0)?)
}
