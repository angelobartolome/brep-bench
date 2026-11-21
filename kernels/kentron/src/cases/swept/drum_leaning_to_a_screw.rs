//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to 1
//! and polar angle from 0° to 60°, grooved by a drum, the solid cylinder of radius 0.8 and length 3
//! about the line through (10, 0, 0) along (0, sin 25°, cos 25°), carried along the screw about the
//! z axis, right-handed, rising 1 a turn, from a turn of −140° to +140°. Everything the drum
//! passes through, cut.
//!
//! Refused. `sweep::cut_by_motion` screws a solid of revolution only through a rod, the tool beyond
//! both its ends where the motion starts and ends; this drum, standing out of the block above and
//! below, rises under 1 over the motion, so no rod about the block's axis is cleared. Built instead
//! from its faces each screwed (`sweep::screw_extrude`), its side would first be split where the
//! screw runs along it: the drum's axis is skew to the z axis, so that is a curve on the cylinder
//! that is neither one of its lines nor a circle nor a plane section, which kentron has no operation
//! to find: the groove's wall there is an envelope.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron's cut by a motion screws a tool only through a rod it clears at both ends, and this \
         drum rises under 1 while standing out of the block; its side, its axis skew to the screw's, \
         touches its sweep along curves that are no lines, circles or plane sections, an envelope \
         kentron has no operation to find",
    ))
}
