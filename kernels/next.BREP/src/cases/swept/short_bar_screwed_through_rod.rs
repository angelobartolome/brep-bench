//! A rod threaded by a short bar carried along a screw about its axis.
//!
//! Through the kernel's sweep of a solid within a region (`sweep_solid_clipped`, the bar as a convex
//! polyhedron; else `sweep_solid_within`, the bar as a prism); the bar, its
//! rectangle 4 ≤ x ≤ 12, |y| ≤ 1.5 from z = −1 to 1, along the screw about the z axis that rises 6
//! a turn, from −90° to +1350°, within 5.2 of the z axis from z = −0.4 to 20.4 (which holds the rod
//! and which neither end position reaches); cut from the rod. The kernel's sweep carries a solid of
//! revolution along a screw and a prism only along a planar motion, so it may refuse.
use crate::solids::p;
use crate::Refusal;
#[allow(unused_imports)]
use brep_kernel::{
    boolean_operation_nary, make_box_brep, make_cylinder_brep, revolve_profile_brep,
    sweep_solid_clipped, sweep_solid_within, SolidTool, BooleanOperation, BrepSolid, OutlinePiece, ScrewStep, SolidMotion,
    SweepClip, SweptTool, Vec3,
};
#[allow(unused_imports)]
use std::f64::consts::{FRAC_PI_2, PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let mut operands = vec![rod];
    let tool0 = SweptTool::Prism {
        origin: p(0.0, 0.0, 0.0),
        normal: p(0.0, 0.0, 1.0),
        x_axis: p(1.0, 0.0, 0.0),
        outline: vec![
            OutlinePiece::Line { from: [4.0, -1.5], to: [12.0, -1.5] },
            OutlinePiece::Line { from: [12.0, -1.5], to: [12.0, 1.5] },
            OutlinePiece::Line { from: [12.0, 1.5], to: [4.0, 1.5] },
            OutlinePiece::Line { from: [4.0, 1.5], to: [4.0, -1.5] },
        ],
        from: -1.0,
        to: 1.0,
    };
    let motion0 = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 1.0, advance: 6.0 / TAU },
        ],
        from: (-90.0f64).to_radians(),
        to: 1350.0f64.to_radians(),
    };
    let clip0 = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 5.2],
        axial: [-0.4, 20.4],
        angle: None,
    };
    // The bar as the convex polyhedron it is: the kernel sweeps it along the screw and cuts it from
    // the rod faster than its level engine builds the rod less the sweep (host: square bar 1.06 s
    // against 1.53 s, the same volume to 5e-8).
    let bar = SolidTool::Polyhedron {
        vertices: (0..8)
            .map(|i| p(if i & 1 != 0 { 12.0 } else { 4.0 }, if i & 2 != 0 { 1.5 } else { -1.5 }, if i & 4 != 0 { 1.0 } else { -1.0 }))
            .collect(),
        faces: vec![vec![0, 2, 3, 1], vec![4, 5, 7, 6], vec![0, 1, 5, 4], vec![2, 6, 7, 3], vec![0, 4, 6, 2], vec![1, 3, 7, 5]],
    };
    if let Ok(swept) = sweep_solid_clipped(&bar, &motion0, &clip0) {
        operands.push(swept);
        return Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?);
    }
    operands.extend(sweep_solid_within(&tool0, &motion0, &clip0)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
