//! A narrow spherical sector: the ball of radius 5 about the origin inside the cone with its apex
//! there, about the z axis, opening upward with half angle α, cos α = 4/5 (about 36.87°). The rim
//! where the cone meets the ball, chamfered by a distance and an angle: 2 down the cone, the
//! chamfer making 45° with it.
use crate::bench::p;
use crate::shared::chamfer::{chamfer_rim, spherical_sector};
use crate::Refusal;
use kentron::blend::ChamferSize;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_4;

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let sector = spherical_sector(m, 4.0 / 5.0)?;
    // (The rim, and a point halfway up the cone's side, away from the seams.)
    let (rim, side) = (p(0.0, 3.0, 4.0), p(0.0, 1.5, 2.0));
    chamfer_rim(m, sector, rim, side, ChamferSize::DistanceAngle(2.0, FRAC_PI_4))
}
