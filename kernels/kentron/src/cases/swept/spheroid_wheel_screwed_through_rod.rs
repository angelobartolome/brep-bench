//! A rod, the cylinder of radius 5 about the z axis from z = 0 to 20, grooved by an oblate spheroid
//! wheel (centre (10.5, 0, 0), semi-axes 6, 6 and 1.5, its short axis (0, cos h, −sin h),
//! h = atan(2π·4.75/30)) screwed about the z axis, rising 30 a turn, from −90° to 330°.
//!
//! Refused. kentron's cut by a motion (`sweep::cut_by_motion`) takes the screw and a solid of
//! revolution, but its meridian only of lines and circular arcs: the spheroid's is half an
//! ellipse. Its characteristic curve under the screw is no curve kentron finds otherwise.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "`sweep::cut_by_motion` takes a solid of revolution whose meridian is lines and circular arcs \
         only, and the spheroid's is an ellipse; kentron has no other sweep of a solid along a \
         screw to find the groove's envelope",
    ))
}
