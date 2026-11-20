//! A cube of side 10 between the origin and (10, 10, 10), the edge where its faces x = 10 and
//! y = 10 meet rounded with a radius that shrinks linearly along it to nothing, r = 2 − z/5: 2 at
//! the bottom and 0 at the top, so the round is a cone whose apex is the corner (10, 10, 10).
//!
//! The cube is a square extruded; the edge is picked by the point at its middle and given a
//! variable fillet, radius 2 at its end on the bottom and 0 at its end on the top.
use crate::bench::{block, p3};
use crate::shared::fillet::fillet_linear;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let cube = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 10.0, 10.0))?;
    let edge = k.edge_at(cube, p3(10.0, 10.0, 5.0))?;
    fillet_linear(k, cube, edge, &|p| p.z < 5.0, 2.0, 0.0)
}
