//! A spiroid face gear hobbed by its worm.
//!
//! The kernel's sweep of a thread under two turns (`solid_sweep_thread`), as for the helicon: the
//! worm's thread on its pitch cone (axis through (0, 30, 6) along x, R(x) = 6 + x·tan 5°, lead 2π,
//! π wide at the cone, flanks 20°, 2.5 either side of it, ends x = 0 and x = 52), turned by t about
//! its axis and then by −t/30 about the z axis for t from 0 to 60π, cut from the ring blank (radii
//! 40 to 52, from z = −9 to its conical face, −0.3 at the inner edge and −1.7 at the outer). The
//! kernel traces one tooth space, cuts it from the blank's cell between two teeth and sews the 30
//! cells: the sweep, its spaces and the gear are all made inside it, from the case's data alone.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::solid_sweep_thread::{thread_gear, RingBlank, ThreadTool};
use brep_kernel::{BrepSolid, TwoTurns};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let axis = (p(0.0, 30.0, 6.0), p(1.0, 0.0, 0.0));
    let thread = ThreadTool {
        axis_point: axis.0,
        axis_direction: axis.1,
        // θ = 0 along +y, right-handed about +x.
        reference: p(0.0, 1.0, 0.0),
        lead: TAU,
        pitch_radius: 6.0,
        taper: 5f64.to_radians().tan(),
        half_width: FRAC_PI_2,
        flank_tan: 20f64.to_radians().tan(),
        tip: 2.5,
        root: 2.5,
        start: 0.0,
        end: 52.0,
    };
    let motion = TwoTurns { axes: [axis, (p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0))], rates: [1.0, -1.0 / 30.0], from: 0.0, to: 60.0 * PI };
    let blank = RingBlank { inner: 40.0, outer: 52.0, bottom: -9.0, face: -0.3, face_outer: -1.7 };
    Ok(thread_gear(&thread, &motion, &blank, 30)?)
}
