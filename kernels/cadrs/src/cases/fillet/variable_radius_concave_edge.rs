//! An L-shaped block 10 deep, the L (0, 0), (20, 0), (20, 4), (6, 4), (6, 12), (0, 12) in x and z
//! extruded from y = 0 to y = 10, its inside edge from (6, 0, 4) to (6, 10, 4) rounded with a
//! radius growing linearly along it, r = 1 + y/5: 1 at y = 0 and 3 at y = 10, adding material.
//!
//! The L is sketched on the plane y = 0 and extruded along +y; the inside edge is picked by the
//! point at its middle and given a variable fillet.
use crate::bench::{extrude_along, p3, v3};
use crate::shared::fillet::fillet_linear;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    // the sketch's x along x and its y along z: the plane's normal −y
    let xz = Plane::new(p3(0.0, 0.0, 0.0), v3(1.0, 0.0, 0.0), v3(0.0, -1.0, 0.0));
    let l = Loop::polygon(&[[0.0, 0.0], [20.0, 0.0], [20.0, 4.0], [6.0, 4.0], [6.0, 12.0], [0.0, 12.0]]);
    let block = extrude_along(k, &Profile::of(xz, vec![l]), v3(0.0, 1.0, 0.0), 10.0)?;
    let edge = k.edge_at(block, p3(6.0, 5.0, 4.0))?;
    fillet_linear(k, block, edge, &|p| p.y < 5.0, 1.0, 3.0)
}
