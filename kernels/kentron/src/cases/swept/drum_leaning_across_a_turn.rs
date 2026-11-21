//! A ring, everything between 3 and 8 from the z axis with |z| ≤ 2.5, with a sealed channel cut
//! inside it by a drum, the solid cylinder of radius 1 and length 3 about the line through
//! (5, 1.2, 0) along (sin 25°, 0, cos 25°), turned rigidly about the z axis by 100°, wholly inside
//! the ring. Everything the drum passes through, cut.
//!
//! Refused. `sweep::cut_by_motion` turns a solid of revolution only clear past a ring sector, and
//! this drum starts and ends inside the ring. Built instead as a box slid is, the drum where it
//! starts and where it ends and its faces each turned, each face would first be split where the
//! turn runs along it. The drum's axis is skew to the z axis, so on its side, at angle θ about
//! its axis with normal n, that is where (c + t a)·(n × ẑ) = 0, a curve t(θ) that is neither a line
//! of the cylinder nor a circle nor a plane section. kentron has no operation that finds it: the
//! channel's wall there is an envelope.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron's cut by a motion turns a tool only clear past a ring sector, and this drum starts and \
         ends inside the ring; its side, its axis skew to the turn's, touches its sweep along curves \
         that are no lines, circles or plane sections, an envelope kentron has no operation to find",
    ))
}
