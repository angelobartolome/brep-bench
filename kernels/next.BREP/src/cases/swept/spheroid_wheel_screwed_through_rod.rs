//! A rod with a helical groove ground by an oblate spheroid carried along a screw, through the
//! kernel's sweep of a solid within a region (`sweep_solid_within`): the spheroid is a solid of
//! revolution about the line through (10.5, 0, 0) along (0, cos h, −sin h), h = atan(2π·4.75/30),
//! its meridian the half ellipse of semi-axes 6 (from the axis) and 1.5 (along it) closed by the
//! axis; screwed about the z axis, rising 30 a turn, from −90° to 330°; cut from the rod.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_cylinder_brep, sweep_solid_within, BooleanOperation, BrepSolid,
    OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool,
};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let h = (TAU * 4.75 / 30.0).atan();
    let wheel = SweptTool::Revolved {
        axis_origin: p(10.5, 0.0, 0.0),
        axis_direction: p(0.0, h.cos(), -h.sin()),
        radial: p(-1.0, 0.0, 0.0),
        meridian: vec![
            OutlinePiece::Ellipse { centre: [0.0, 0.0], semi: [6.0, 1.5], rotation: 0.0, start: -FRAC_PI_2, end: FRAC_PI_2 },
            OutlinePiece::Line { from: [0.0, 1.5], to: [0.0, -1.5] },
        ],
    };
    let screw = SolidMotion {
        steps: vec![ScrewStep {
            axis_origin: p(0.0, 0.0, 0.0),
            axis_direction: p(0.0, 0.0, 1.0),
            turn: 1.0,
            advance: 30.0 / TAU,
        }],
        from: (-90.0f64).to_radians(),
        to: 330.0f64.to_radians(),
    };
    let around_rod = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 5.5],
        axial: [-1.0, 21.0],
        angle: None,
    };
    let mut operands = vec![rod];
    operands.extend(sweep_solid_within(&wheel, &screw, &around_rod)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
