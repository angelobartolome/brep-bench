//! The half cylinder of radius 5 and height 6 on the side x ≥ 0, its curved rim chamfered by a
//! distance growing linearly with the angle round the axis, 1 to 2.
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(_k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    Err(Refusal::from(
        "cadrs's Chamfer has no distance that varies along its edge (a ChamferSpec is the same all \
         along), and the chamfer here, ruled between a spiral of Archimedes and a helix, is no \
         surface its other features make",
    ))
}
