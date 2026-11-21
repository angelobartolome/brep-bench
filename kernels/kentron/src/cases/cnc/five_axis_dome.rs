//! The blank, the solid cylinder of radius 50 about the z axis from z = 0 to z = 45, machined into a
//! dome by a five-axis program of four cutters: a bull-nose mill roughing in level rings, a
//! ball-end mill held square to the dome finishing along 180 meridians (each a turn of 72° about
//! a level line through the origin) and round 15 circles near the pole (each a whole turn about z
//! of the cutter leaning), then twelve spot faces and twelve holes square to the dome.
//!
//! Refused. kentron has no sweep of a solid (`sweep::pipe` refuses one), and its cut by a motion
//! (`sweep::cut_by_motion`) takes a turn of a solid of revolution only from a ring sector about
//! the turn's own axis, starting and ending clear of it: here the blank is a disc about z and the
//! finishing turns are about level lines, through the blank, with the cutter's axis across them.
//! Built instead from elementary pieces (each cutter at the ends of each move, its faces revolved
//! about each turn's axis, split where the turn runs along them), the program is over a thousand
//! such pieces cut from the blank in turn, tori and surfaces of revolution among them. That
//! build is not attempted here: the 944 simpler moves of `cnc/ball-end-surface-raster`, built
//! that way, already run past the harness's 60 s.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err("kentron has no sweep of a solid, and its cut by a motion (sweep::cut_by_motion) turns a \
         cutter only past a ring sector about the turn's own axis, not a disc blank turned through \
         about level lines; a build from over a thousand elementary pieces cut in turn is not \
         attempted, the 944 simpler moves of cnc/ball-end-surface-raster already running past 60 s"
        .into())
}
