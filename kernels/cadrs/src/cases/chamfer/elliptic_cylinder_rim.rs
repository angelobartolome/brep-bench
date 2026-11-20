//! An elliptic cylinder from z = 0 to z = 8 on the ellipse of semi-axes 10 along x and 6 along y,
//! the ellipse round its top chamfered by 2 on both faces.
//!
//! The ellipse is sketched and extruded, and the edges round the top are chamfered.
use crate::bench::extrude;
use crate::shared::chamfer::chamfer;
use crate::shared::fillet::edges_at_height;
use crate::Refusal;
use cadrs_kernel::{BodyId, ChamferSpec, DefaultKernel, Loop, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let cylinder = extrude(k, &Profile::of(Plane::xy(0.0), vec![Loop::ellipse([0.0, 0.0], 10.0, 6.0)]), 8.0)?;
    let rim = edges_at_height(k, cylinder, 8.0)?;
    chamfer(k, cylinder, &rim, ChamferSpec::EqualDistance(2.0))
}
