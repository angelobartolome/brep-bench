//! A prism from z = 0 to z = 6 on the periodic uniform cubic B-spline on the control points
//! (−10, −5), (0, −7), (10, −5), (12, 4), (4, 7), (0, 3), (−4, 7), (−12, 4), a bean dented at the
//! top, its rim round the top rounded to radius 1.
//!
//! The spline is a sketch's periodic spline, extruded, and the edges round the top are filleted.
use crate::shared::fillet::{bean, edges_at_height, fillet};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let prism = bean(k, 6.0)?;
    let rim = edges_at_height(k, prism, 6.0)?;
    fillet(k, prism, &rim, 1.0)
}
