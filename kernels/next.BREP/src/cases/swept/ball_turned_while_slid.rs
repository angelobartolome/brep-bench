//! A ball of radius 1 centered at (5, 0, 0), turned right-handed about the z axis by 180°·t and
//! slid by (25t, 0, 0) for t from 0 to 1: everything it passes through.
//!
//! A ball looks the same whichever way it is turned, so what it sweeps depends on its centre's path
//! alone, here the curtate trochoid c(t) = (5 cos πt + 25t, 5 sin πt, 0), which curves no tighter
//! than 1.75, more than the ball's radius: the solid is the pipe of radius 1 about that path,
//! closed by a half ball at each end. The path is interpolated through 201 points of it (a cubic,
//! within about 1e-9 of the trochoid), the unit circle swept along it square to it
//! (`sweep_profile_along_path`), and the pipe united with the balls about its ends, whose other
//! halves lie inside it.
use crate::solids::p;
use crate::solids::sweep::circle;
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, interpolate_curve, make_sphere_brep, sweep_profile_along_path, BooleanOperation, BrepSolid};
use std::f64::consts::PI;

const STATIONS: usize = 200;

fn centre(t: f64) -> brep_kernel::Vec3 {
    p(5.0 * (PI * t).cos() + 25.0 * t, 5.0 * (PI * t).sin(), 0.0)
}

pub fn build() -> Result<BrepSolid, Refusal> {
    let parameters: Vec<f64> = (0..=STATIONS).map(|k| k as f64 / STATIONS as f64).collect();
    let points: Vec<_> = parameters.iter().map(|&t| centre(t)).collect();
    let path = interpolate_curve(&points, 3, &parameters)?;
    let profile = circle(p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), 1.0)?;
    let pipe = sweep_profile_along_path(&profile, &path, None)?;
    let up = p(0.0, 0.0, 1.0);
    let start = make_sphere_brep(centre(0.0), 1.0, up)?;
    let end = make_sphere_brep(centre(1.0), 1.0, up)?;
    Ok(boolean_operation_nary(&[pipe, start, end], BooleanOperation::Union)?)
}
