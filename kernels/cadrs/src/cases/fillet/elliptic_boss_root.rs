//! A plate from (−20, −15, 0) to (20, 15, 4) with an elliptic boss on it, semi-axes 10 along x and
//! 6 along y, from z = 4 to z = 10, the boss's root rounded to radius 2, adding material: a pipe
//! about the ellipse's outer parallel curve.
//!
//! The boss, an elliptic cylinder run down into the plate, is united with it; its root is the
//! curved edge of the plate's top.
use crate::bench::{block, extrude, p3, union};
use crate::shared::fillet::fillet;
use crate::Refusal;
use cadrs_kernel::{BodyId, CurveKind, DefaultKernel, Loop, Pick, Plane, Profile};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let plate = block(k, p3(-20.0, -15.0, 0.0), p3(20.0, 15.0, 4.0))?;
    let boss = extrude(k, &Profile::of(Plane::xy(1.0), vec![Loop::ellipse([0.0, 0.0], 10.0, 6.0)]), 9.0)?;
    let joined = union(k, plate, boss)?;
    // the plate's top's edges but its four straight ones
    let root = k.edges_where(joined, &|e| {
        [e.start, e.mid, e.end].iter().all(|p| (p.z - 4.0).abs() < 1e-7) && e.curve != CurveKind::Line
    })?;
    if root.is_empty() {
        return Err(Refusal::from("the boss has no root on the plate"));
    }
    fillet(k, joined, &root, 2.0)
}
