//! A plate from (−15, −10, 0) to (15, 10, 4) with a hole through it on the ellipse of semi-axes 6
//! along x and 3 along y, the hole's rim on the top chamfered by 1 on both faces: a countersink of
//! constant slope.
//!
//! The hole is an elliptic cylinder, run past both faces, cut from the plate; its rim is the
//! curved edge of the top.
use crate::bench::{block, cut, extrude, p3};
use crate::shared::chamfer::chamfer;
use crate::Refusal;
use cadrs_kernel::{BodyId, ChamferSpec, CurveKind, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = block(k, p3(-15.0, -10.0, 0.0), p3(15.0, 10.0, 4.0))?;
    let hole = extrude(k, &Profile::of(Plane::xy(-1.0), vec![Loop::ellipse([0.0, 0.0], 6.0, 3.0)]), 6.0)?;
    let drilled = cut(k, plate, hole)?;
    // the top's edges but the plate's four straight ones
    let rim = k.edges_where(drilled, &|e| {
        [e.start, e.mid, e.end].iter().all(|p| (p.z - 4.0).abs() < 1e-7) && e.curve != CurveKind::Line
    })?;
    if rim.is_empty() {
        return Err(Refusal::from("the hole has no rim on the top"));
    }
    chamfer(k, drilled, &rim, ChamferSpec::EqualDistance(1.0))
}
