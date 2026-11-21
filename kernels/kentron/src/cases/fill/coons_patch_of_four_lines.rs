//! A block over the square from (0, 0) to (10, 10), capped by the Coons patch of the four
//! straight edges (0, 0, 5) → (10, 0, 5) → (10, 10, 15) → (0, 10, 5) → (0, 0, 5): the hyperbolic
//! paraboloid z = 5 + xy/10.
//!
//! With straight edges the Coons patch is the ruled surface between the edges on y = 0 and
//! y = 10, so the block is the ruled loft between its ends, the faces y = 0 and y = 10, each
//! corner of one joined to the same corner of the other: the top is ruled between the two top
//! edges, the other sides and the floor are flat.
use crate::bench::p;
use crate::shared::loft::polygon;
use kentron::loft::{self, LoftOptions};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let near = polygon(m, &[p(0.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 0.0, 5.0), p(0.0, 0.0, 5.0)])?;
    let far = polygon(m, &[p(0.0, 10.0, 0.0), p(10.0, 10.0, 0.0), p(10.0, 10.0, 15.0), p(0.0, 10.0, 5.0)])?;
    let options = LoftOptions { solid: true, ruled: true, ..LoftOptions::default() };
    Ok(loft::loft(m, &[near, far], &options)?.shape)
}
