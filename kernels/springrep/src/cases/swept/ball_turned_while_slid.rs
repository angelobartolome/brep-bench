//! The solid a ball of radius 1 centered at (5, 0, 0) passes through as it turns half a turn about
//! the z axis while sliding 25 along x, both at a steady rate.
use std::f64::consts::PI;

use springrep::math::{Point3, Vec3};
use springrep::primitive::sphere;
use springrep::sweep::{Motion, solid};
use springrep::topo::Brep;

use crate::shared::{at, e};

pub fn build() -> Result<Brep, String> {
    let ball = sphere(&at([5., 0., 0.]), 1., "ball").map_err(e)?;
    let rolling = Motion::Then(
        Box::new(Motion::Turn {
            origin: Point3::ZERO,
            axis: Vec3::Z,
            from: 0.,
            to: PI,
        }),
        Box::new(Motion::Slide(Vec3::new(25., 0., 0.))),
    );
    solid(&ball, &rolling, "swept").map_err(e)
}
