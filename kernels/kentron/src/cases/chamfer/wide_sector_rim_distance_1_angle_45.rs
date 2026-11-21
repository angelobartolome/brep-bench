//! A wide spherical sector: the ball of radius 5 about the origin inside the cone with its apex
//! there, about the z axis, opening upward with half angle α, cos α = 5/13 (about 67.38°). The rim
//! where the cone meets the ball, chamfered by a distance and an angle: 1 down the cone, the
//! chamfer making 45° with it.
use crate::bench::p;
use crate::shared::chamfer::{chamfer_rim, spherical_sector};
use crate::Refusal;
use kentron::blend::ChamferSize;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_4;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let sector = spherical_sector(m, 5.0 / 13.0)?;
    // (The rim, and a point halfway up the cone's side, away from the seams.)
    let (rim, side) = (p(0.0, 60.0 / 13.0, 25.0 / 13.0), p(0.0, 30.0 / 13.0, 12.5 / 13.0));
    chamfer_rim(m, sector, rim, side, ChamferSize::DistanceAngle(1.0, FRAC_PI_4))
}
