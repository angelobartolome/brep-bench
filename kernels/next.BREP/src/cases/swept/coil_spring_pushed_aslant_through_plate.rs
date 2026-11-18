//! A plate with a hole punched through it by a coil spring pushed aslant wholly through it, through
//! the kernel's sweep of a tube slid across a slab (`sweep_tube_slid_across`): the spring is the
//! ball of radius 0.4 swept along the helix of radius 2 about the z axis from (2, 0, −12), turning
//! right-handed and rising 1.5 a turn for three turns (its ends half balls); it slides by
//! (3, 1, 18), wholly below the plate to wholly above it, and the plate (z from 0 to 4) loses
//! everything the spring passes through.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{boolean_operation_nary, make_box_brep, sweep_tube_slid_across, BooleanOperation, BrepSolid};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let plate = make_box_brep(p(-15.0, -15.0, 0.0), 30.0, 30.0, 4.0)?;
    let helix = |s: f64| {
        let angle = 3.0 * TAU * s;
        p(2.0 * angle.cos(), 2.0 * angle.sin(), -12.0 + 4.5 * s)
    };
    let hole = sweep_tube_slid_across(&helix, 0.4, p(3.0, 1.0, 18.0), p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), [0.0, 4.0])?;
    Ok(boolean_operation_nary(&[plate, hole], BooleanOperation::Subtract)?)
}
