//! The disk of radius 20 about (0, 0, 18) in the plane x = 0 turned a quarter turn about the y
//! axis, +z toward +x, so that it crosses the axis, united with a straight leg of radius 20 and
//! length 40 at each end.
use anvil_brep::{Frame, Solid, revolve};
use glam::{DVec2, DVec3};

use crate::shared::{circle, e, region, rod, unite_all};

pub fn build() -> Result<Solid, String> {
    // the disk drawn in the plane x = 0, sketch x along world z and sketch y along world y, so
    // that its normal is +x; turning about the sketch y axis by −90° takes +z to +x. anvil-brep
    // refuses it, since the profile crosses the axis; turned as its two sides of the axis it
    // refuses too, since an arc whose circle reaches the axis would sweep a spindle torus
    let plane = Frame {
        origin: DVec3::ZERO,
        x: DVec3::Z,
        y: DVec3::Y,
        z: DVec3::X,
    };
    let turn = revolve::revolve(
        &plane,
        &region(&[circle((18., 0.), 20.)])?,
        DVec2::ZERO,
        DVec2::Y,
        -0.5 * std::f64::consts::PI,
    )
    .map_err(e)?;
    unite_all(
        turn,
        [
            rod([-40., 0., 18.], [0., 0., 18.], 20.),
            rod([18., 0., -40.], [18., 0., 0.], 20.),
        ],
    )
}
