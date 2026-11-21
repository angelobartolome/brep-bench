//! A rod, the cylinder of radius 5 about the z axis from z = 0 to 20, threaded by a bar: the closed
//! uniform cubic B-spline through control points (3.5, 0), (4.5, −1.5), (8, −1.5), (12, −1),
//! (12, 1), (8, 1.5), (4.5, 1.5) in (x, z) on y = 0, carried from y = −8 to 8, screwed about the z
//! axis rising 6 a turn from −90° to 1350°.
//!
//! Refused. kentron's cut by a motion (`sweep::cut_by_motion`) screws only a solid of revolution
//! about the rod, and the bar is a prism. Screwing its faces (`sweep::screw_extrude`) would need
//! them split where the screw runs along the spline surface, a curve with no closed form that
//! kentron has no operation to find.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "`sweep::cut_by_motion` screws only a solid of revolution, and the bar is a spline prism; \
         screwing its faces instead needs them split along the screw's contact curve on the spline \
         surface, which kentron has no operation to find",
    ))
}
