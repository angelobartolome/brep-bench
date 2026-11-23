//! A thick disc, counterbored, with a countersunk hole drilled from the counterbore's floor so
//! close to its wall that the hole and its countersink reach under it.
//!
//! The disc (radius 100, z −20 to 20) less the counterbore (radius 60, from the floor z = 0 up past
//! the top), and then less the drill: the hole (radius 3.5 about (59.9, 0), z −15 to 0) united with
//! the countersink (the 90° cone from its point at (59.9, 0, −4) to radius 4 on z = 0), each step a
//! kernel boolean. The drill stops at z = 0, so nothing above the floor's plane is cut beyond the
//! wall.
use crate::solids::p;
use crate::Refusal;
use crate::solids::boolean::{cut, union};
use brep_kernel::{make_cone_brep, make_cylinder_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let up = p(0.0, 0.0, 1.0);
    let disc = make_cylinder_brep(p(0.0, 0.0, -20.0), up, 100.0, 40.0)?;
    let counterbore = make_cylinder_brep(p(0.0, 0.0, 0.0), up, 60.0, 21.0)?;
    let hole = make_cylinder_brep(p(59.9, 0.0, -15.0), up, 3.5, 15.0)?;
    let countersink = make_cone_brep(p(59.9, 0.0, -4.0), up, 0.0, 4.0, 4.0)?;
    let bored = cut(&disc, &counterbore)?;
    let drill = union(&hole, &countersink)?;
    Ok(cut(&bored, &drill)?)
}
