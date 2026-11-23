//! A 30-tooth bevel gear's blank with one tooth space cut by a straight-sided tooth of its
//! 15-tooth pinion, the shafts square and meeting at O: the pinion turns by t about the x axis
//! while the gear turns by −t/2 about the z axis, t from −60° to 60°.
//!
//! Seen from the blank the tooth is turned by t about the x axis and then by t/2 about the z axis:
//! two turns about axes through O, the tooth's apex. The kernel sweeps a pyramid frustum under
//! turns about axes through its apex (`sweep_pyramid_within`), exactly within a range of distances
//! from the apex; the range given is the blank's own, from its front cone's nearest point to O (on
//! the pitch cone, 15√5 − 10 from O) to its back corner (√((15√5)² + 6²)). The sweep is cut from
//! the blank, the case's quadrilateral revolved.
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, revolve_profile_brep, sweep_pyramid_within, BooleanOperation, BrepSolid, Pyramid, ScrewStep,
    SolidMotion,
};
use std::f64::consts::{FRAC_PI_2, TAU};

const S5: f64 = 2.236_067_977_499_79;

pub fn build() -> Result<BrepSolid, Refusal> {
    // g along the line where the pitch cones touch, d from the pinion into the gear.
    let (g, d) = (p(2.0 / S5, 0.0, 1.0 / S5), p(-1.0 / S5, 0.0, 2.0 / S5));
    let s0 = 15.0 * S5;
    // The trapezoid in the plane square to g through P = s0·g, (u, w) along d and y; the section's
    // second axis is g × d = −y, so w is read negated.
    let t20 = 20f64.to_radians().tan();
    let trapezoid = [[-2.5, FRAC_PI_2 + 2.5 * t20], [2.5, FRAC_PI_2 - 2.5 * t20], [2.5, -(FRAC_PI_2 - 2.5 * t20)], [-2.5, -(FRAC_PI_2 + 2.5 * t20)]];
    let tooth = Pyramid {
        apex: p(0.0, 0.0, 0.0),
        axis: g,
        x_axis: d,
        at: s0,
        section: trapezoid.iter().map(|q| [q[0], -q[1]]).collect(),
        from: s0 - 12.0,
        to: s0 + 2.0,
    };
    let o = p(0.0, 0.0, 0.0);
    let deg = 1f64.to_radians();
    let motion = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: o, axis_direction: p(1.0, 0.0, 0.0), turn: 1.0, advance: 0.0 },
            ScrewStep { axis_origin: o, axis_direction: p(0.0, 0.0, 1.0), turn: 0.5, advance: 0.0 },
        ],
        from: -60.0 * deg,
        to: 60.0 * deg,
    };
    let space = sweep_pyramid_within(&tooth, &motion, [s0 - 10.0, (s0 * s0 + 36.0).sqrt()])?;
    let q = [(30.894427, 13.211146), (27.316718, 20.366563), (18.372447, 15.894427), (21.683489, 9.272343)];
    let meridian = polygon(&q.iter().map(|&(x, z)| p(x, 0.0, z)).collect::<Vec<_>>())?;
    let blank = revolve_profile_brep(&meridian, p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), TAU)?;
    Ok(boolean_operation_nary(&[blank, space], BooleanOperation::Subtract)?)
}
