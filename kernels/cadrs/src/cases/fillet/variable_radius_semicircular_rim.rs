//! The half of the cylinder of radius 5 on the z axis from z = 0 to z = 6 on the side x ≥ 0, the
//! half circle where its curved side meets its top rounded with a radius growing linearly with
//! the angle round the axis: 1 where it starts at (0, −5, 6) and 2 where it ends at (0, 5, 6).
//!
//! The half disk is sketched and extruded; the half circle is picked by the point at its middle
//! and given a variable fillet (its fraction along a circle is its angle's).
use crate::bench::p3;
use crate::shared::fillet::{fillet_linear, half_cylinder};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Pick};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let half = half_cylinder(k, 5.0, 6.0)?;
    let rim = k.edge_at(half, p3(5.0, 0.0, 6.0))?;
    fillet_linear(k, half, rim, &|p| p.y < 0.0, 1.0, 2.0)
}
