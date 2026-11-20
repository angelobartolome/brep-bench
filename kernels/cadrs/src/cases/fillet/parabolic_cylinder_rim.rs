//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, the circle round its top blended by
//! a conic of rho 0.5, a parabola tangent to the top and the side each 2 from the edge.
//!
//! The rim is given cadrs's conic fillet, whose contact lines are a circular fillet's of the same
//! size: a radius of 2 sets them 2 from an edge between faces at 90°.
use crate::bench::{cylinder, p3, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, FilletProfile, FilletSpec, Kernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let c = cylinder(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 5.0, 10.0)?;
    let rim = k.edge_at(c, p3(5.0, 0.0, 10.0))?;
    let spec = FilletSpec { profile: FilletProfile::Conic { rho: 0.5 }, ..FilletSpec::radius(2.0) };
    Ok(k.fillet_with(c, &[rim], &spec)?.first()?)
}
