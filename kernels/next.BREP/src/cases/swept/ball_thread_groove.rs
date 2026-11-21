use crate::solids::p;
use crate::solids::swept::{less_all, revolved_tool, turn_about_z, Piece};
use crate::Refusal;
use brep_kernel::{make_cylinder_brep, sweep_clip_less, sweep_solid, BrepSolid, OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let (x, z) = (p(1.0, 0.0, 0.0), p(0.0, 0.0, 1.0));
    // The ball's two ends keep clear of the rod (below z = 0 and above 20), so the kernel builds
    // the rod less the groove directly, without a boolean, where it does.
    let tool = SweptTool::Revolved {
        axis_origin: p(5.0, 0.0, -2.0),
        axis_direction: z,
        radial: x,
        meridian: vec![
            OutlinePiece::Arc { centre: [0.0, 0.0], radius: 1.0, start: -FRAC_PI_2, end: FRAC_PI_2 },
            OutlinePiece::Line { from: [0.0, 1.0], to: [0.0, -1.0] },
        ],
    };
    let screw = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: z, turn: 1.0, advance: 4.0 / TAU }],
        from: 0.0,
        to: 6.0 * TAU,
    };
    let rod_clip = SweepClip { axis_origin: p(0.0, 0.0, 0.0), axis_direction: z, reference: x, radius: [0.0, 5.0], axial: [0.0, 20.0], angle: None };
    if let Ok(Some(mut bodies)) = sweep_clip_less(&tool, &screw, &rod_clip) {
        if bodies.len() == 1 {
            return Ok(bodies.remove(0));
        }
    }
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), z, 5.0, 20.0)?;
    let ball = revolved_tool(p(5.0, 0.0, -2.0), z, x, &[
        Piece::Arc { centre: [0.0, 0.0], radius: 1.0, start: -FRAC_PI_2, end: FRAC_PI_2 },
        Piece::Line { from: [0.0, 1.0], to: [0.0, -1.0] },
    ]);
    less_all(rod, vec![sweep_solid(&ball, &turn_about_z(0.0, 6.0 * TAU, 4.0 / TAU))?])
}
