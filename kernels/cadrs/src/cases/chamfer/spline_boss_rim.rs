//! The bean prism of `fillet/spline-boss-rim`, from z = 0 to z = 6 on a periodic cubic B-spline
//! dented at the top, its rim round the top chamfered by 1 on both faces.
//!
//! The spline is a sketch's periodic spline, extruded, and the edges round the top are chamfered.
use crate::shared::chamfer::chamfer;
use crate::shared::fillet::{bean, edges_at_height};
use crate::Refusal;
use cadrs_kernel::{BodyId, ChamferSpec, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let prism = bean(k, 6.0)?;
    let rim = edges_at_height(k, prism, 6.0)?;
    chamfer(k, prism, &rim, ChamferSpec::EqualDistance(1.0))
}
