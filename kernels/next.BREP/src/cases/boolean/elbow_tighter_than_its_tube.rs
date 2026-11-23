//! The outside of a pipe elbow whose bend is tighter than the pipe is thick: a tube of radius 20
//! turned a quarter turn about the y axis 18 from its centre-line, with a straight leg of 40 at
//! each end.
//!
//! The disk reaches 2 past the bend axis. Its part on the near side of the axis (z ≥ 0 in the
//! plane x = 0) sweeps the spindle torus's outer part: that half-plane region revolved a quarter
//! turn by the kernel (its chord on the axis). The part beyond the axis turns the other way, within
//! the first leg (each of its points moves nearer the leg's axis as it turns), so it adds nothing.
//! The legs are cylinders; the three are united by the kernel.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, make_arc, make_cylinder_brep, revolve_profile_brep, BooleanOperation, BrepSolid, NurbsCurve, Vec4};
use std::f64::consts::FRAC_PI_2;

pub fn build() -> Result<BrepSolid, Refusal> {
    let half = 2.0 * 19f64.sqrt();
    // The disk's boundary in the plane x = 0 (axes y and z), from its foot (0, half, 0) on the axis
    // round through the top to (0, −half, 0), then back along the axis.
    let centre = p(0.0, 0.0, 18.0);
    let start = (-18.0f64).atan2(half);
    let end = std::f64::consts::PI - start;
    let rim = make_arc(centre, p(0.0, 1.0, 0.0), p(0.0, 0.0, 1.0), 20.0, start, end)?;
    let chord = NurbsCurve::new(1, vec![0.0, 0.0, 1.0, 1.0], vec![Vec4::from_point(p(0.0, -half, 0.0), 1.0), Vec4::from_point(p(0.0, half, 0.0), 1.0)])?;
    let turn = revolve_profile_brep(&[rim, chord], p(0.0, 0.0, 0.0), p(0.0, 1.0, 0.0), FRAC_PI_2)?;
    let first = make_cylinder_brep(p(-40.0, 0.0, 18.0), p(1.0, 0.0, 0.0), 20.0, 40.0)?;
    let second = make_cylinder_brep(p(18.0, 0.0, -40.0), p(0.0, 0.0, 1.0), 20.0, 40.0)?;
    Ok(boolean_operation_nary(&[turn, first, second], BooleanOperation::Union)?)
}
